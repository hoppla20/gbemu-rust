use std::{cell::RefCell, fmt::Debug};

use tracing::{instrument, trace};

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

    instruction_counter: RefCell<usize>,
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

            instruction_counter: RefCell::new(0),
        };

        trace!(
            name: "cpu::state",
            "{:?} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            result.cpu,
            result.mmu.read_byte(result.cpu.registers.pc),
            result.mmu.read_byte(result.cpu.registers.pc.wrapping_add(1)),
            result.mmu.read_byte(result.cpu.registers.pc.wrapping_add(2)),
            result.mmu.read_byte(result.cpu.registers.pc.wrapping_add(3)),
        );

        result.cpu.current_instruction =
            Instruction::decode_instruction(result.mmu.read_byte(result.cpu.registers.pc));
        (result.cpu.registers.pc, _) = result.cpu.registers.pc.overflowing_add(1);

        result
    }

    #[instrument(level = "trace", skip_all, fields(counter = *self.instruction_counter.borrow()))]
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
                    *self.instruction_counter.borrow_mut() += 1;

                    trace!(
                        name: "cpu::state",
                        "{:?} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                        self.cpu,
                        self.mmu.read_byte(self.cpu.registers.pc),
                        self.mmu.read_byte(self.cpu.registers.pc.wrapping_add(1)),
                        self.mmu.read_byte(self.cpu.registers.pc.wrapping_add(2)),
                        self.mmu.read_byte(self.cpu.registers.pc.wrapping_add(3)),
                    );
                },
            }

            self.cpu.generic_fetch(&mut self.mmu)?;
        }

        Ok(())
    }

    pub fn instruction_counter(&self) -> usize {
        *self.instruction_counter.borrow()
    }
}

impl Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.cpu))
    }
}
