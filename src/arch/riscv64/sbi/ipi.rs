//! The SBI's inter-process interrupts extension.

use core::arch::asm;

use spin::RwLock;

use crate::arch::riscv64::sbi;

const EID: usize = 0x735049;
const FID_SEND: usize = 0;

#[unsafe(link_section = ".text.trap")] // Makes trap_vec slightly quicker
#[unsafe(no_mangle)]
pub(crate) static IPI_MESSAGE: RwLock<Option<IpiMessage>> = RwLock::new(None);

#[repr(usize)]
pub(crate) enum IpiMessage {
    Panic = 0,
}

pub(crate) fn send(mask: usize, base: usize) -> Result<(), sbi::Error> {
    let error: isize;

    unsafe {
        asm!(
            "ecall",
            inout("a0") mask => error,
            in("a1") base,
            in("a6") FID_SEND,
            in("a7") EID,
        );
    }

    match error {
        0 => Ok(()),
        _ => Err(unsafe { sbi::Error::from_isize(error) }),
    }
}
