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

use riscv::register::sscratch;

use crate::cpu::CpuContext;

impl CpuContext<'static> {
    /// Returns the reference to the CPU's context from `sscratch`.
    ///
    /// This function will always be inline.
    ///
    /// # Safety
    /// The function if safe **if and only if** the caller ensures that the `sscratch` register has
    /// been previously initialized.
    #[inline(always)]
    pub(crate) unsafe fn fetch() -> *mut Self {
        // SAFETY: Will never be/should never be called before sscratch is initialized.
        sscratch::read() as *mut Self
    }
}
