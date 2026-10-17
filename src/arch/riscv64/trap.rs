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

//! Trap handling.

use core::arch::naked_asm;
use core::mem::transmute;

use riscv::asm::wfi;
use riscv::register::scause::Scause;

use crate::arch::riscv64::sbi::dbcn::dbcn;
use crate::arch::riscv64::sbi::dbcn::dbcnln;
use crate::arch::riscv64::sbi::ipi::IPI_MESSAGE;
use crate::arch::riscv64::sbi::ipi::IpiMessage;
use crate::cpu::CpuContext;
use crate::util::ansi;

#[unsafe(link_section = ".text.trap")]
#[unsafe(naked)]
pub(crate) unsafe extern "C" fn trap_vec() -> ! {
    naked_asm!(
        // Swap and sscratch to get pointer to TrapFrame/CpuContext
        "csrrw   t6, sscratch, t6",
        // Save initial integer registers needed for checks
        "sd      x1, 8(t6)",  // ra
        "sd      x2, 16(t6)", // sp
        "sd      x3, 24(t6)", // gp
        "sd      x4, 32(t6)", // tp
        "sd      x5, 40(t6)", // t0
        // Check if the message is a panic message (IPI_MESSAGE = 0)
        "la      t0, IPI_MESSAGE",
        "ld      t0, 0(t0)",
        "bnez    t0, 3f", // If IPI_MESSAGE != 0, then a panic didn't happen
        // If IPI_MESSAGE == 0, then HOLY SHIT PANIC
        "ld      t0, 560(t6)", // (*context).cpu_id is directly after TrapFrame struct
        "beqz    t0, 3f",      // If the current hart is hart 0, then don't halt (yet)
        // If not on hart 0, halt in an infinite wfi loop
        "1:",
        "wfi",
        "j       1b",
        // Continue saving registers
        "3:",
        "sd      x6, 48(t6)",   // t1
        "sd      x7, 56(t6)",   // t2
        "sd      x8, 64(t6)",   // s0/fp
        "sd      x9, 72(t6)",   // s1
        "sd      x10, 80(t6)",  // a0
        "sd      x11, 88(t6)",  // a1
        "sd      x12, 96(t6)",  // a2
        "sd      x13, 104(t6)", // a3
        "sd      x14, 112(t6)", // a4
        "sd      x15, 120(t6)", // a5
        "sd      x16, 128(t6)", // a6
        "sd      x17, 136(t6)", // a7
        "sd      x18, 144(t6)", // s2
        "sd      x19, 152(t6)", // s3
        "sd      x20, 160(t6)", // s4
        "sd      x21, 168(t6)", // s5
        "sd      x22, 176(t6)", // s6
        "sd      x23, 184(t6)", // s7
        "sd      x24, 192(t6)", // s8
        "sd      x25, 200(t6)", // s9
        "sd      x26, 208(t6)", // s10
        "sd      x27, 216(t6)", // s11
        "sd      x28, 224(t6)", // t3
        "sd      x29, 232(t6)", // t4
        "sd      x30, 240(t6)", // t5
        // Save original t6 value
        "csrr    t5, sscratch",
        "sd      t5, 248(t6)",
        // FPRs
        // These registers are only saved if sstatus.FS is NOT 0b00 (off)
        "csrr    t0, sstatus",
        "sd      t0, 528(t6)", // Save sstatus now since its already loaded
        // Check if FP registers are off (sstatus.FS is bits 13:14)
        "li      t1, 0x6000",
        "and     t2, t0, t1",
        "beqz    t2, 4f", // Skip ONLY if FS == 0b00 (off)
        "fsd     f0,  256(t6)",
        "fsd     f1,  264(t6)",
        "fsd     f2,  272(t6)",
        "fsd     f3,  280(t6)",
        "fsd     f4,  288(t6)",
        "fsd     f5,  296(t6)",
        "fsd     f6,  304(t6)",
        "fsd     f7,  312(t6)",
        "fsd     f8,  320(t6)",
        "fsd     f9,  328(t6)",
        "fsd     f10, 336(t6)",
        "fsd     f11, 344(t6)",
        "fsd     f12, 352(t6)",
        "fsd     f13, 360(t6)",
        "fsd     f14, 368(t6)",
        "fsd     f15, 376(t6)",
        "fsd     f16, 384(t6)",
        "fsd     f17, 392(t6)",
        "fsd     f18, 400(t6)",
        "fsd     f19, 408(t6)",
        "fsd     f20, 416(t6)",
        "fsd     f21, 424(t6)",
        "fsd     f22, 432(t6)",
        "fsd     f23, 440(t6)",
        "fsd     f24, 448(t6)",
        "fsd     f25, 456(t6)",
        "fsd     f26, 464(t6)",
        "fsd     f27, 472(t6)",
        "fsd     f28, 480(t6)",
        "fsd     f29, 488(t6)",
        "fsd     f30, 496(t6)",
        "fsd     f31, 504(t6)",
        "csrr    t1, fcsr",
        "sd      t1, 512(t6)",
        "4:", // Both paths join here
        // CSRs (except sstatus which was saved earlier)
        "csrr    t0, sepc",
        "sd      t0, 520(t6)",
        "csrr    t0, scause",
        "sd      t0, 536(t6)",
        "csrr    t0, stval",
        "sd      t0, 544(t6)",
        "csrr    t0, satp",
        "sd      t0, 552(t6)",
        // Call the trap handler
        "mv      a0, t6",
        "call    kernel_trap_handler",
    )
}

/// Handles a trap for the current hart.
///
/// # Safety
///
/// This function is fundamentally very unsafe because traps don't guarantee any specific
/// environment. Most of the body of the function is wrapped in an `unsafe` block.
#[unsafe(link_section = ".text.trap")]
#[unsafe(no_mangle)]
unsafe extern "C" fn kernel_trap_handler(context: *mut CpuContext) -> ! {
    unsafe {
        if transmute::<u64, Scause>((*context).trap_frame.scause).is_interrupt() {
            dbcnln!("TRAPPED (interrupt) on hart {}", (*context).cpu_id);
        } else {
            dbcn!("{}", ansi::RED);

            dbcnln!("TRAPPED (exception) on hart {}", (*context).cpu_id);

            // GPRs
            for i in 0..=31 {
                dbcnln!(
                    "[hart {}] x{:<7}{:#018x}",
                    (*context).cpu_id,
                    i,
                    (*context).trap_frame.x[i]
                );
            }
            // FPRs
            for i in 0..=31 {
                dbcnln!(
                    "[hart {}] f{:<7}{}",
                    (*context).cpu_id,
                    i,
                    (*context).trap_frame.f[i]
                );
            }
            // CSRs
            dbcnln!(
                "[hart {}] fcsr    {:#018x}",
                (*context).cpu_id,
                (*context).trap_frame.fcsr
            );
            dbcnln!(
                "[hart {}] sepc    {:#018x}",
                (*context).cpu_id,
                (*context).trap_frame.sepc
            );
            dbcnln!(
                "[hart {}] sstatus {:#018x}",
                (*context).cpu_id,
                (*context).trap_frame.sstatus
            );
            dbcnln!(
                "[hart {}] scause  {:#018x}",
                (*context).cpu_id,
                (*context).trap_frame.scause
            );
            dbcnln!(
                "[hart {}] stval   {:#018x}",
                (*context).cpu_id,
                (*context).trap_frame.stval
            );
            dbcnln!(
                "[hart {}] satp    {:#018x}",
                (*context).cpu_id,
                (*context).trap_frame.satp
            );

            dbcn!("{}", ansi::RESET);
        }

        // Check the IPI message
        match *IPI_MESSAGE.read() {
            None => {},
            Some(IpiMessage::Panic) => dbcnln!("all harts will now halt"),
        }
    }

    // Halt the current hart if the function finishes
    loop {
        wfi();
    }
}
