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

//! I/O devices that transport data bit-by-bit.

use crate::dev::Driver;

pub(crate) mod uart;

/// A device that trasports data bit-by-bit.
pub(crate) trait Serial<Config>: Driver<Config> {
    /// Sends a byte to the serial device.
    fn send_byte(&mut self, b: u8);
    /// Receives a byte from the serial device.
    fn receive_byte(&mut self) -> Option<u8>;
}
