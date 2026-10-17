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

//! Architecture-independent CPU state management.

use fdt_raw::Fdt;

use crate::arch::frame::TrapFrame;

/// The context of a logical CPU.
///
/// This structure is unique across every logical CPU core. It stores globally needed information,
/// such as a pointer to the flattened device tree and the CPU's identifier.
#[repr(C)]
pub(crate) struct CpuContext<'a> {
    pub(crate) trap_frame: TrapFrame,
    pub(crate) cpu_id: usize,
    pub(crate) fdt: Fdt<'a>,
}

impl CpuContext<'static> {
    pub(crate) fn new(trap_frame: TrapFrame, cpu_id: usize, fdt: Fdt<'static>) -> Self {
        Self {
            trap_frame,
            cpu_id,
            fdt,
        }
    }
}
