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

//! Register frames.

// IMPORTANT: Register frames must ALWAYS have zero padding, otherwise the assembly that manipulates
//            them will cause undefined behavior.

/// A frame that stores registers saved during context switching.
#[repr(C, packed)]
pub(crate) struct ThreadFrame {
    /// The stack pointer register, `sp`.
    pub(crate) sp: u64,
    /// An array of the 12 callee-saved GPRs, `s0`–`s11`, not including the stack pointer (`sp`)
    /// register.
    pub(crate) s: [u64; 12],
}

// NOTE TO DEVELOPER: If this assertion fails, ThreadFrame has been modified, so the appropriate
//                    context switching logic should be reviewed (and amended as needed).
const _: () = assert!(core::mem::size_of::<ThreadFrame>() == 104);

/// A frame that stores registers saved during a trap.
#[repr(C)]
pub(crate) struct TrapFrame {
    /// An array of the 32 GPRs.
    pub(crate) x: [u64; 32],
    // TODO: The FPRs are sometimes 32-bit or 128-bit depending on extensions, but that doesn't
    //       matter for now
    /// An array of the 32 FPRs.
    pub(crate) f: [f64; 32],
    pub(crate) fcsr: u64,
    pub(crate) sepc: u64,
    pub(crate) sstatus: u64,
    pub(crate) scause: u64,
    pub(crate) stval: u64,
    pub(crate) satp: u64,
}

impl TrapFrame {
    pub(crate) fn empty() -> Self {
        Self {
            x: [0; 32],
            f: [0.0; 32],
            fcsr: 0,
            sepc: 0,
            sstatus: 0,
            scause: 0,
            stval: 0,
            satp: 0,
        }
    }
}

// NOTE TO DEVELOPER: If this assertion fails, TrapFrame has been modified, so the appropriate
//                    trap handling logic should be reviewed (and amended as needed).
const _: () = assert!(core::mem::size_of::<TrapFrame>() == 560);
