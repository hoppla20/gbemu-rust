use core::panic;
use std::fmt::Debug;

use tracing::debug;

use crate::emulator::ExecutionError;

const TAC_ENABLE_BIT: usize = 2;
const TAC_CYCLES_256_BIT: usize = 9;
const TAC_CYCLES_4_BIT: usize = 3;
const TAC_CYCLES_16_BIT: usize = 5;
const TAC_CYCLES_64_BIT: usize = 7;

pub enum TimerFrequency {
    Cycles256,
    Cycles4,
    Cycles16,
    Cycles64,
}

impl From<u8> for TimerFrequency {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::Cycles256,
            0b01 => Self::Cycles4,
            0b10 => Self::Cycles16,
            0b11 => Self::Cycles64,
            _ => panic!("Unknown timer frequency '{}'", value),
        }
    }
}

pub struct TimerRegisters {
    pub system_counter: u16,
    pub counter: u8,
    pub modulo: u8,
    pub control: u8,

    pending_overflow: bool,
    counter_written: bool,
}

impl Default for TimerRegisters {
    fn default() -> Self {
        Self {
            system_counter: 0xAB00,
            counter: 0x00,
            modulo: 0x00,
            control: 0xF8,
            pending_overflow: false,
            counter_written: false,
        }
    }
}

impl Debug for TimerRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "DIV:{:02X} TIMA:{:02X} TMA:{:02X} TAC:{:02X}",
            self.divider(),
            self.counter,
            self.modulo,
            self.control
        ))
    }
}

impl TimerRegisters {
    pub fn divider(&self) -> u8 {
        (self.system_counter >> 8) as u8
    }

    pub fn reset_divider(&mut self) {
        self.system_counter = 0;
    }

    pub fn write_counter(&mut self, value: u8) {
        self.counter = value;
        self.counter_written = true;
    }

    pub fn step(&mut self) -> Result<bool, ExecutionError> {
        let mut request_interrupt = false;
        if self.pending_overflow {
            debug!(
                name: "timer::interrupt",
                "Requesting timer interrupt and resetting timer counter to timer modulo {}",
                self.modulo
            );
            self.pending_overflow = false;
            self.counter = self.modulo;
            request_interrupt = true;
        }

        let old_system_counter = self.system_counter;
        self.system_counter = self.system_counter.wrapping_add(4);

        // TODO: audio DIV-APU event

        if self.control & (1 << TAC_ENABLE_BIT) > 0 {
            let tick = match Into::<TimerFrequency>::into(self.control) {
                TimerFrequency::Cycles256 => {
                    (old_system_counter & (1 << TAC_CYCLES_256_BIT) > 0)
                        && (self.system_counter & (1 << TAC_CYCLES_256_BIT) == 0)
                },
                TimerFrequency::Cycles4 => {
                    (old_system_counter & (1 << TAC_CYCLES_4_BIT) > 0)
                        && (self.system_counter & (1 << TAC_CYCLES_4_BIT) == 0)
                },
                TimerFrequency::Cycles16 => {
                    (old_system_counter & (1 << TAC_CYCLES_16_BIT) > 0)
                        && (self.system_counter & (1 << TAC_CYCLES_16_BIT) == 0)
                },
                TimerFrequency::Cycles64 => {
                    (old_system_counter & (1 << TAC_CYCLES_64_BIT) > 0)
                        && (self.system_counter & (1 << TAC_CYCLES_64_BIT) == 0)
                },
            };

            if !self.counter_written && tick {
                let (temp, overflow) = self.counter.overflowing_add(1);
                self.counter = temp;

                if overflow {
                    debug!(
                        name: "timer::overflow",
                        "Timer counter overflowed. Delayed interrupt request and counter reset after the next cycle"
                    );
                    self.pending_overflow = true;
                } else {
                    debug!(name: "timer::increment", "Incremented timer counter to {}", self.counter);
                }
            }
        }

        if self.counter_written {
            self.counter_written = false;
        }

        Ok(request_interrupt)
    }
}
