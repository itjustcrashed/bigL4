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

//! Initial platform setup.

use core::arch::naked_asm;
use core::ptr::write_bytes;

use fdt_raw::Fdt;
use riscv::register::sscratch;
use riscv::register::stvec;
use riscv::register::stvec::Stvec;
use riscv::register::stvec::TrapMode;

use crate::arch::frame::TrapFrame;
use crate::arch::linker_symbols;
use crate::arch::riscv64::sbi::dbcn::dbcn;
use crate::arch::riscv64::sbi::dbcn::dbcnln;
use crate::arch::riscv64::trap;
use crate::cpu::CpuContext;
use crate::dev::Driver;
use crate::dev::serial::Serial;
use crate::dev::serial::uart::ns16550a::Ns16550a;
use crate::mm;
use crate::util::ansi;

/// Starts the kernel and calls [`preboot`].
///
/// # Safety
/// Calling this function will probably (definitely) crash the kernel. It is intended to be called
/// only by the SBI.
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text.start")]
unsafe extern "C" fn start() -> ! {
    naked_asm!(
        // Set stack pointer for boot hart
        "la sp, __boot_hart_stack_end",
        // Call preboot (defined below)
        "call preboot"
    )
}

/// Continues the preboot process started in [`start`]
///
/// # Safety
///
/// This function will likely (absolutely) crash the kernel if called by anything other than
/// [`start`].
#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
unsafe extern "C" fn preboot(hart_id: usize, fdt_ptr: *mut u8) -> ! {
    // Things that need to be used later on
    let fdt: Fdt;

    // Print build info
    dbcn!("{}", ansi::BLUE);
    dbcnln!(
        "{} version {} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );
    dbcn!("{}", ansi::RESET);
    dbcnln!(
        "BOOTED at {:#018x} on hart {}",
        unsafe { &linker_symbols::kernel_start as *const u8 as usize },
        hart_id
    );
    dbcnln!("fdt located at {:#018x}", fdt_ptr as usize);

    // SAFETY: A whole bunch of extremely unsafe memory management happens here, and it's not
    //         verified because...how would you?
    unsafe {
        // Clear the BSS memory region
        write_bytes(
            linker_symbols::bss_start.as_ptr() as *mut u8,
            0,
            linker_symbols::bss_end.as_ptr() as usize - linker_symbols::bss_start.as_ptr() as usize,
        );

        // Parse the FDT
        fdt = Fdt::from_ptr(fdt_ptr).unwrap();

        // Make a memory map from the FDT
        let mut guard = mm::MEMORY_MAP.lock();
        guard.populate(&fdt);

        let mut iter = guard.regions.into_iter();
        while let Some(Some(region)) = iter.next() {
            dbcnln!(
                "{} bytes of {} memory at {:#018x}",
                region.size,
                region.status.str(),
                region.phys_base_addr
            )
        }

        let context = CpuContext::new(TrapFrame::empty(), hart_id, fdt.clone());
        sscratch::write(&context as *const CpuContext as usize);
        stvec::write(Stvec::new(
            trap::trap_vec as *const u8 as usize,
            TrapMode::Direct,
        ));
    }

    let uart = unsafe { Ns16550a::from_fdt(&fdt) };
    if let Some(mut uart) = uart {
        uart.send_byte(0x61); // a
        uart.send_byte(0x62); // b
        uart.send_byte(0x63); // c
    }

    panic!("preboot attempted to return")
}
