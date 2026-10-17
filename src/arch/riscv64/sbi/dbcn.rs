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

//! The SBI's debug console extension.

use core::arch::asm;
use core::fmt;

use crate::arch::riscv64::mm::virt_to_phys_ptr;
use crate::arch::riscv64::sbi;

const EID: usize = 0x4442434E;
const FID_WRITE: usize = 0;

pub(crate) struct DebugConsole;

impl fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = sbi::dbcn::write(s);
        Ok(())
    }
}

/// Writes a string slice to the SBI's debug console and returns the amount of bytes written.
///
/// This function is a form of `sbi_debug_console_write`, written to be idiomatic of Rust. The
/// amount of bytes written is returned because the SBI might write only part of the string slice.
///
/// # Examples
/// ```rust
/// let result = write("Hello, World!");
/// let _ = write("The return value of this call won't be saved.")
/// ```
pub(crate) fn write(s: &str) -> Result<usize, sbi::Error> {
    let error: isize;
    let bytes_written: usize;

    let bytes = s.as_ptr();
    let len = s.len();

    unsafe {
        asm!(
            "ecall",
            inout("a0") len => error,
            inout("a1") virt_to_phys_ptr(bytes) => bytes_written,
            in("a2") 0, // base_addr_hi is always 0 on 64-bit
            in("a6") FID_WRITE,
            in("a7") EID,
        );
    }

    match error {
        0 => Ok(bytes_written),
        _ => Err(unsafe { sbi::Error::from_isize(error) }),
    }
}

/// Formats and writes arguments to the SBI Debug Console.
///
/// # Examples
/// ```
/// dbcn!(); // prints absolutely nothing
/// dbcn!("Hello, World!"); // prints "Hello, World!"
/// ```
macro_rules! dbcn {
    () => {};
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($crate::arch::riscv64::sbi::dbcn::DebugConsole, $($arg)*);
        }
    };
}
pub(crate) use dbcn;

/// Formats and writes arguments, followed by a newline character, to the SBI Debug Console.
///
/// # Examples
/// ```
/// dbcnln!(); // prints "↵"
/// dbcnln!("Hello, World!"); // prints "Hello, World!↵"
/// ```
macro_rules! dbcnln {
    () => {
        {
            use core::fmt::Write;
            let _ = writeln!($crate::arch::riscv64::sbi::dbcn::DebugConsole);
        }
    };
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = writeln!($crate::arch::riscv64::sbi::dbcn::DebugConsole, $($arg)*);
        }
    };
}
pub(crate) use dbcnln;
