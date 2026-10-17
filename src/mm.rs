// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2026 It Just Crashed
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

//! Memory management.

use fdt_raw::Fdt;
use spin::Mutex;

use crate::arch::linker_symbols;

pub(crate) static MEMORY_MAP: Mutex<MemoryMap> = Mutex::new(MemoryMap {
    regions: [None; 32],
    count: 0,
});

/// A map of physical memory regions.
pub(crate) struct MemoryMap {
    /// A fixed-size buffer of slots for regions of memory that are being tracked by the map.
    pub(crate) regions: [Option<MemoryRegion>; 32], // Safe, comically high upper bound
    /// The number of memory regions that are currently being tracked by the map.
    pub(crate) count: usize,
}

impl MemoryMap {
    fn push(&mut self, phys_base_addr: usize, size: usize, status: MemoryRegionStatus) {
        if size == 0 {
            return;
        }
        if self.count < self.regions.len() {
            self.regions[self.count] = Some(MemoryRegion {
                phys_base_addr,
                size,
                status,
            });
            self.count += 1;
        } else {
            panic!("too many memory regions (this might be a NASA computer)")
        }
    }

    pub(crate) fn populate(&mut self, fdt: &Fdt) {
        let mut local_regions: [Option<MemoryRegion>; 64] = [None; 64];
        let mut local_count = 0;

        let mut push_local = |base: usize, size: usize, status: MemoryRegionStatus| {
            if size == 0 {
                return;
            }
            if local_count < local_regions.len() {
                local_regions[local_count] = Some(MemoryRegion {
                    phys_base_addr: base,
                    size,
                    status,
                });
                local_count += 1;
            }
        };

        // Blocks of physical memory
        for memory in fdt.memory() {
            if let Some(reg) = memory.reg() {
                for info in reg {
                    if let Some(size) = info.size {
                        push_local(
                            info.address as usize,
                            size as usize,
                            MemoryRegionStatus::Vacant,
                        );
                    }
                }
            }
        }

        // Inserts a reserved region and splits overlapping vacant regions
        let mut reserve = |r_base: usize, r_size: usize| {
            if r_size == 0 {
                return;
            }
            let r_end = r_base + r_size;
            let mut next_regions: [Option<MemoryRegion>; 64] = [None; 64];
            let mut next_count = 0;

            let mut push_next = |base: usize, size: usize, status: MemoryRegionStatus| {
                if size == 0 {
                    return;
                }
                if next_count < next_regions.len() {
                    next_regions[next_count] = Some(MemoryRegion {
                        phys_base_addr: base,
                        size,
                        status,
                    });
                    next_count += 1;
                }
            };

            for i in 0..local_count {
                if let Some(reg) = local_regions[i] {
                    let reg_end = reg.phys_base_addr + reg.size;

                    // Check if a Vacant region overlaps with the new Reserved region
                    if matches!(reg.status, MemoryRegionStatus::Vacant)
                        && reg.phys_base_addr < r_end
                        && r_base < reg_end
                    {
                        // Split vacant region: left remainder
                        if reg.phys_base_addr < r_base {
                            push_next(
                                reg.phys_base_addr,
                                r_base - reg.phys_base_addr,
                                MemoryRegionStatus::Vacant,
                            );
                        }
                        // Split vacant region: right remainder
                        if r_end < reg_end {
                            push_next(r_end, reg_end - r_end, MemoryRegionStatus::Vacant);
                        }
                    } else {
                        // Keep non-overlapping or already-reserved regions as-is
                        push_next(reg.phys_base_addr, reg.size, reg.status);
                    }
                }
            }

            // Insert the final reserved region
            push_next(r_base, r_size, MemoryRegionStatus::Reserved);

            local_regions = next_regions;
            local_count = next_count;
        };

        // Reserve the kernel itself
        unsafe {
            reserve(
                linker_symbols::kernel_start.as_ptr() as usize,
                linker_symbols::kernel_end.as_ptr() as usize
                    - linker_symbols::kernel_start.as_ptr() as usize,
            );
        }

        // Early memory reservations
        for reservation in fdt.memory_reservations() {
            if reservation.size as usize > 0 {
                reserve(reservation.address as usize, reservation.size as usize);
            }
        }

        // Entries in /reserved-memory
        for reservation in fdt.reserved_memory() {
            if let Some(reg) = reservation.reg() {
                for info in reg {
                    if let Some(size) = info.size {
                        reserve(info.address as usize, size as usize);
                    }
                }
            }
        }

        // Commit resolved regions
        for i in 0..local_count {
            if let Some(reg) = local_regions[i] {
                self.push(reg.phys_base_addr, reg.size, reg.status);
            }
        }
    }
}

/// A single region of memory.
#[derive(Clone, Copy)]
pub(crate) struct MemoryRegion {
    /// The physical base address of the memory region.
    pub(crate) phys_base_addr: usize,
    /// The size of the memory region in bytes.
    pub(crate) size: usize,
    /// The status of the memory region.
    pub(crate) status: MemoryRegionStatus,
}

/// A status for a [`MemoryRegion`].
///
/// Vacant regions are memory that can be used by the kernel for general allocations, while reserved
/// regions should only be written to for special purposes such as writing to MMIO.
#[derive(Clone, Copy)]
pub(crate) enum MemoryRegionStatus {
    Vacant,
    Reserved,
}

impl MemoryRegionStatus {
    /// Returns the status as a string slice.
    pub(crate) fn str(&self) -> &str {
        match self {
            Self::Vacant => "vacant",
            Self::Reserved => "reserved",
        }
    }
}
