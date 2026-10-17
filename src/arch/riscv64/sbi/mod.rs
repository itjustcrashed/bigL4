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

//! Handling of the SBI, from version 3.0.
//!
//! Reference
//! _[RISC-V Supervisor Binary Interface Specification](https://github.com/riscv-non-isa/riscv-sbi-doc/releases/download/v3.0/riscv-sbi.pdf)_.

use core::mem::transmute;

use thiserror::Error;

#[macro_use]
pub(crate) mod dbcn;
pub(crate) mod ipi;

/// An error returned by a call to the SBI.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[repr(isize)]
pub(crate) enum Error {
    #[error("failed")]
    Failed = -1,
    #[error("not supported")]
    NotSupported = -2,
    #[error("invalid parameter(s)")]
    InvalidParam = -3,
    #[error("denied or not allowed")]
    Denied = -4,
    #[error("invalid address(es)")]
    InvalidAddr = -5,
    #[error("already available")]
    AlreadyAvailable = -6,
    #[error("already started")]
    AlreadyStarted = -7,
    #[error("already stopped")]
    AlreadyStopped = -8,
    #[error("shared memory not available")]
    NoSharedMem = -9,
    #[error("invalid state")]
    InvalidState = -10,
    #[error("bad or invalid range")]
    BadRange = -11,
    #[error("timeout")]
    Timeout = -12,
    #[error("io error")]
    Io = -13,
    #[error("denied or not allowed due to lock status")]
    DeniedLocked = -14,
}

impl Error {
    /// Creates an error from an `isize`.
    #[inline(always)]
    unsafe fn from_isize(value: isize) -> Self {
        unsafe { transmute(value) }
    }
}
