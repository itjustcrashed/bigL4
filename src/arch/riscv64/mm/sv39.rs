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

//! Sv39 memory paging.
//!
//! RISC-V's Sv39 is a 3-level memory paging scheme for RV64. It maps up to 512 GiB of memory.
//!
//! Sv39 pages are 4096 bytes in size, and a page table is page-sized.

use bitflags::bitflags;

const PAGE_SIZE: usize = 4096;

/// An Sv39 memory page table.
#[derive(Debug)]
struct PageTable([Entry; 512]);

impl Default for PageTable {
    fn default() -> Self {
        Self([Entry::default(); 512])
    }
}

bitflags! {
    /// A set of bitflags for a [Pte].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct EntryFlags: usize {
        /// Whether the entry is valid.
        ///
        /// Accessing an invalid page traps the kernel (page fault).
        const V = 1 << 0;
        /// Whether the page can be read.
        const R = 1 << 1;
        /// Whether the page can be written.
        const W = 1 << 2;
        /// Whether the page can be executed.
        const X = 1 << 3;
        /// Whether the page is accessible in U-mode.
        const U = 1 << 4;
        /// Whether the page is globally mapped across all address spaces.
        const G = 1 << 5;
        /// Whether the page has been read, written, or executed.
        const A = 1 << 6;
        /// Whether the page has been written since it was last marked as not dirty.
        const D = 1 << 7;
    }
}

/// An entry for an Sv39 [PageTable].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Entry(usize);

impl Entry {
    fn new(phys_addr: usize, flags: EntryFlags) -> Self {
        // The PPN is bits 53:10 of the entry
        let phys_page_num = (phys_addr >> 12) << 2;
        Self(phys_page_num | flags.bits())
    }

    /// Checks whether the entry is valid.
    fn is_valid(&self) -> bool {
        (self.0 & EntryFlags::V.bits()) != 0
    }

    /// Checks whether the entry is a leaf.
    ///
    /// An entry with any of bits `3:1` is considered a leaf entry.
    fn is_leaf(&self) -> bool {
        (self.0 & (EntryFlags::R.bits() | EntryFlags::W.bits() | EntryFlags::X.bits())) != 0
    }

    /// Extracts the physical address from this page table entry.
    fn phys_addr(&self) -> usize {
        // Clear the flags (lower 10 bits) and shift back to a byte-addressable physical address
        (self.0 >> 2) & !0xFFF
    }
}
