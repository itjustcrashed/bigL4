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

//! General-purpose helpers.

/// ANSI control codes.
pub(crate) mod ansi {
    pub(crate) const RED: &str = "\x1b[31m";
    pub(crate) const BLUE: &str = "\x1b[34m";
    pub(crate) const RESET: &str = "\x1b[0m";
}
