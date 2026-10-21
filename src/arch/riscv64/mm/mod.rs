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

//! RISC-V memory management.

mod sv39;

// TODO: Implement this file

// MARK: virt --> phys

/// Resolves and returns the physical immutable pointer of a virtual immutable pointer.
///
/// This function will always be inline.
#[inline(always)]
pub(crate) fn virt_to_phys_ptr<T>(virt: *const T) -> *const T {
    virt
}

/// Resolves and returns the physical mutable pointer of a virtual mutable pointer.
///
/// This function will always be inline.
#[inline(always)]
pub(crate) fn virt_to_phys_ptr_mut<T>(virt: *mut T) -> *mut T {
    virt
}

/// Resolves and returns the physical address of a virtual address.
///
/// This function will always be inline.
#[inline(always)]
pub(crate) fn virt_to_phys_addr(virt: usize) -> usize {
    virt
}

// MARK: phys --> virt

/// Resolves and returns the virtual immutable pointer of a physical immutable pointer.
///
/// This function will always be inline.
#[inline(always)]
pub(crate) fn phys_to_virt_ptr<T>(virt: *const T) -> *const T {
    virt
}

/// Resolves and returns the virtual mutable pointer of a physical mutable pointer.
///
/// This function will always be inline.
#[inline(always)]
pub(crate) fn phys_to_virt_ptr_mut<T>(virt: *mut T) -> *mut T {
    virt
}

/// Resolves and returns the physical address of a virtual address.
///
/// This function will always be inline.
#[inline(always)]
pub(crate) fn phys_to_virt_addr(virt: usize) -> usize {
    virt
}
