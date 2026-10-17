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

//! NS16550A-compatible UART device.

use fdt_raw::Fdt;
use volatile_register::RO;
use volatile_register::RW;

use crate::dev::Driver;
use crate::dev::serial::Serial;
use crate::dev::serial::uart::UART_TARGET_BAUD_RATE;

/// A UART driver for NS16550A-compatible devices.
pub(crate) struct Ns16550a {
    mmio: *mut Ns16550aMmio,
}

#[repr(C)]
struct Ns16550aMmio {
    /// Receive buffer (on read) / transmit holding (on write) / divisor latch low (when DLAB is 1),
    /// located at offset `0`.
    rbr_thr_dll: RW<u8>,

    /// Interrupt enable register (when DLAB is 0) / divisor latch high (when DLAB is 1), located at
    /// offset `1`.
    ier_dlm: RW<u8>,

    /// FIFO control register (on write) / interrupt status register (on read), located at offset
    /// `2`.
    fcr_isr: RW<u8>,

    /// Line control register (LCR), located at offset `3`, with DLAB at bit 7.
    lcr: RW<u8>,

    /// Modem control register (MCR), located at offset `4`.
    mcr: RW<u8>,

    /// Line status register (LSR), located at offset `5`, with DR at bit 0 and THRE at bit 5.
    lsr: RO<u8>,

    /// Modem status register (MSR), located at offset `6`.
    msr: RO<u8>,

    /// Scratchpad register (SPR), located at offset `7`.
    spr: RW<u8>,
}

/// Configuration for the [`Ns16550a`] driver (which is just a physical base address).
pub(crate) struct Ns16550aConfig {
    base_address: usize,
    clock_frequency: u32,
}

impl Ns16550aConfig {
    /// Creates a new configuration.
    pub(crate) fn new(base_address: usize, clock_frequency: u32) -> Self {
        Self {
            base_address,
            clock_frequency,
        }
    }
}

impl<'a> Driver<Ns16550aConfig> for Ns16550a {
    // TODO: Safety
    unsafe fn init(config: Ns16550aConfig) -> Self {
        let mmio = config.base_address as *mut Ns16550aMmio;

        // Dynamically compute the 16-bit divisor
        let divisor = config.clock_frequency / (UART_TARGET_BAUD_RATE * 16);

        // SAFETY: All NS16550A-compatible devices support basic setup routines.
        unsafe {
            // Disable all interrupts while configuring
            (*mmio).ier_dlm.write(0x00);

            // Enable DLAB to set the divisor
            (*mmio).lcr.write(0x80);

            // Split the computed divisor into low and high bytes
            (*mmio).rbr_thr_dll.write((divisor & 0xFF) as u8);
            (*mmio).ier_dlm.write(((divisor >> 8) & 0xFF) as u8);

            // Clear DLAB and setup with: word length of 8 bits, no parity, and 1 stop bit
            (*mmio).lcr.write(0x03);

            // Enable and clear FIFOs
            (*mmio).fcr_isr.write(0x07);
        }

        Self { mmio }
    }

    // not applicable
    unsafe fn deinit() {}

    unsafe fn from_fdt(fdt: &Fdt) -> Option<Self> {
        for node in fdt.all_nodes() {
            let is_ns16550a = node.compatibles().any(|c| c.contains("ns16550a"));

            if is_ns16550a {
                if let Some(mut reg_iter) = node.reg() {
                    if let Some(reg_info) = reg_iter.next() {
                        let base_address = reg_info.address as usize;
                        let clock_frequency = node
                            .find_property("clock-frequency")
                            .and_then(|prop| prop.as_u32())
                            .unwrap_or(3_686_400);

                        let config = Ns16550aConfig::new(base_address, clock_frequency);
                        return Some(unsafe { Self::init(config) });
                    }
                }
            }
        }
        None
    }

    #[inline(always)]
    fn name() -> &'static str {
        "National Semiconductor UART NS16550A"
    }
}

impl Serial<Ns16550aConfig> for Ns16550a {
    // SAFETY: The raw pointer stored in 'mmio' was verified by Ns16550a::init.

    fn send_byte(&mut self, b: u8) {
        unsafe {
            // Wait until the THRE bit is true
            while ((*self.mmio).lsr.read() & 0x20) == 0 {
                core::hint::spin_loop();
            }
            // Write the byte to the THR
            (*self.mmio).rbr_thr_dll.write(b);
        }
    }

    fn receive_byte(&mut self) -> Option<u8> {
        unsafe {
            // Check if DR bit is true
            if (*self.mmio).lsr.read() & 0x01 != 0 {
                Some((*self.mmio).rbr_thr_dll.read())
            } else {
                None
            }
        }
    }
}
