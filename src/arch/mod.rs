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

//! Architecture-specific code.

#[cfg(target_arch = "riscv64")]
pub(crate) mod riscv64;

pub(crate) mod linker_symbols {
    #[cfg(target_arch = "riscv64")]
    pub(crate) use super::riscv64::linker_symbols::*;
}

pub(crate) mod frame {
    #[cfg(target_arch = "riscv64")]
    pub(crate) use super::riscv64::frame::*;
}

#[cfg(target_arch = "riscv64")]
pub(crate) use riscv64::panic_handler;
