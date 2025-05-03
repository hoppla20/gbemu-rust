use std::fmt::Debug;

#[cfg(test)]
use std::cell::RefCell;

use tracing::trace;

use crate::{
    cpu::{Cpu, instructions::Instruction, interrupts::Interrupt},
    memory::{mbc::Mbc0, mmu::Mmu},
    serial::{LogSerial, Serial},
};

#[derive(Debug)]
pub enum ExecutionError {
    NoImpl { instruction: Instruction },
    MemoryWrite { address: u16 },
    MemoryRead { address: u16 },
}

pub struct Emulator {
    pub cpu: Cpu,
    pub mmu: Mmu,

    #[cfg(test)]
    trace_counter: RefCell<usize>,
}

impl Emulator {
    pub fn new_from_buffer(
        rom: &[u8],
        cpu_option: Option<Cpu>,
        serial_option: Option<Box<dyn Serial>>,
    ) -> Self {
        let mbc = Mbc0::new_from_buffer(rom);
        let serial = if let Some(s) = serial_option {
            s
        } else {
            Box::new(LogSerial::default())
        };
        let mut mmu = Mmu::new(Box::new(mbc), serial);

        let mut result = Self {
            cpu: if let Some(cpu) = cpu_option {
                cpu
            } else {
                Cpu::new(&mut mmu)
            },
            mmu,

            #[cfg(test)]
            trace_counter: RefCell::new(0),
        };

        trace!("{:?}", result);

        result.cpu.current_instruction =
            Instruction::decode_instruction(result.mmu.read_byte(result.cpu.registers.pc));
        (result.cpu.registers.pc, _) = result.cpu.registers.pc.overflowing_add(1);

        result
    }

    pub fn step(&mut self) -> Result<(), ExecutionError> {
        let mut cpu_completed = false;
        if !self.cpu.halted {
            cpu_completed = self.cpu.step(&mut self.mmu)?
        }

        let timer_interrupt = self.mmu.io.timer.step()?;
        if timer_interrupt {
            self.cpu.request_interrupt(&mut self.mmu, Interrupt::Timer);
        }

        if self.mmu.io.interrupt_enable & self.mmu.io.interrupt_flags != 0 {
            self.cpu.halted = false;
        }

        if !self.cpu.halted && cpu_completed {
            match self.cpu.current_instruction {
                Instruction::prefix => {},
                Instruction::isr { .. } => {},
                _ => {
                    #[cfg(test)]
                    {
                        *self.trace_counter.borrow_mut() += 1;
                    }

                    trace!("{:?}", self);
                },
            }

            self.cpu.generic_fetch(&mut self.mmu)?;
        }

        Ok(())
    }

    #[cfg(test)]
    pub fn trace_counter(&self) -> usize {
        *self.trace_counter.borrow()
    }
}

impl Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} PCMEM:{:02X},{:02X},{:02X},{:02X} SC:{:04X} IE:{:02X} IF:{:02X} {:?}",
            self.cpu,
            self.mmu.read_byte(self.cpu.registers.pc),
            self.mmu.read_byte(self.cpu.registers.pc.wrapping_add(1)),
            self.mmu.read_byte(self.cpu.registers.pc.wrapping_add(2)),
            self.mmu.read_byte(self.cpu.registers.pc.wrapping_add(3)),
            self.mmu.io.timer.system_counter,
            self.mmu.io.interrupt_enable,
            self.mmu.io.interrupt_flags,
            self.mmu.io.timer,
        ))
    }
}
