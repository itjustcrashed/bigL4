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

//! Device drivers.

use fdt_raw::Fdt;

pub(crate) mod serial;

/// A universal interface for device drivers to be managed.
pub(crate) trait Driver<Config>: Sized {
    /// Initializes the driver.
    unsafe fn init(config: Config) -> Self;
    /// Deinitializes the driver.
    unsafe fn deinit();
    /// Returns the name of the driver.
    ///
    /// This function should be defined with `#[inline(always)]` wherever possible.
    ///
    /// A driver's name should follow a simple formula: `<vendor> <device> [<model>]`. The formula
    /// may be adjusted as necessary. For example, a driver for the GPUs found in some Apple silicon
    /// processors would be named 'Apple AGX.'
    fn name() -> &'static str;
    /// Returns a driver initialized with data from the FDT.
    unsafe fn from_fdt(fdt: &Fdt) -> Option<Self>;
}
