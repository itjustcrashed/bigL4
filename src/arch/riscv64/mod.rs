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

//! RISC-V 64-bit.
//!
//! # Terminology
//!
//! Some documentation and code in this module may use terms specific to RISC-V systems. Below is a
//! list of terms that you may come across, along with their definitions.
//!
//! - _GPR_ — **G**eneral **P**urpose **R**egister; a register `x0`–`x31`
//! - _CSR_ — **C**ontrol and **S**tatus **R**egister; a register assigned to a special action
//! - _FPR_ — **F**eneral **P**oint **  R**egister; a register `f0`–`f31`
//! - _SBI_ — the **S**upervisor **B**inary **I**nterface; firmware that provides a basic
//! interface to RISC-V hardware.

use core::panic::PanicInfo;

use riscv::asm::ebreak;
use riscv::asm::wfi;

use crate::arch::riscv64::sbi::dbcn::dbcn;
use crate::arch::riscv64::sbi::dbcn::dbcnln;
use crate::cpu::CpuContext;
use crate::util::ansi;

mod cpu;
pub(crate) mod frame;
mod mm;
mod preboot;
pub(crate) mod sbi;
mod trap;

pub(crate) mod linker_symbols {
    unsafe extern "C" {
        #[link_name = "__kernel_start"]
        pub(crate) static kernel_start: [u8; 0];

        #[link_name = "__bss_start"]
        pub(crate) static bss_start: [u8; 0];

        #[link_name = "__bss_end"]
        pub(crate) static bss_end: [u8; 0];

        #[link_name = "__kernel_end"]
        pub(crate) static kernel_end: [u8; 0];
    }
}

#[inline(always)]
pub(crate) unsafe fn panic_handler(info: &PanicInfo) -> ! {
    let hart_id = unsafe { (*CpuContext::fetch()).cpu_id };

    // Create a bitmask representing the harts that arent the current one
    let mut hart_mask: usize = 0;
    for id in 0..4 {
        if id != hart_id {
            hart_mask |= 1 << id;
        }
    }

    // Halt other cores by sending an IPI message
    // Pass 0 as the base so the bitmask maps directly to Hart IDs 0..63.
    // This only targets all other harts
    // TODO: Make this work when harts > 64
    (*sbi::ipi::IPI_MESSAGE.write()) = Some(sbi::ipi::IpiMessage::Panic);

    let _ = sbi::ipi::send(hart_mask, 0);

    // Disable all interrupts
    unsafe {
        riscv::register::sstatus::clear_sie();

        riscv::register::sie::clear_stimer();
        riscv::register::sie::clear_ssoft();
        riscv::register::sie::clear_sext();
    }

    // Now do the fun part of printing shit
    let hart_id = unsafe { (*CpuContext::fetch()).cpu_id };
    dbcn!("{}", ansi::RED);
    dbcnln!("PANIC on hart {}: {}", hart_id, info);
    dbcnln!("halting...");
    dbcn!("{}", ansi::RESET);
    unsafe { ebreak() }

    // Halt this core
    loop {
        wfi()
    }
}
