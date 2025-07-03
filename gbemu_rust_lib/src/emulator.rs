use std::fmt::Debug;
use tracing::instrument;
use tracing::trace;

use crate::cpu::Cpu;
use crate::cpu::instructions::Instruction;
use crate::cpu::interrupts::Interrupt;
use crate::cpu::interrupts::InterruptFlags;
use crate::memory::mbc::new_mbc_from_buffer;
use crate::serial::LogSerial;
use crate::serial::Serial;
use crate::system::System;

macro_rules! trace_cpu_state {
    ($self:ident) => {
        trace!(
            name: "cpu::state",
            "{:?} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            $self.cpu,
            $self.system.read_byte($self.cpu.registers.pc),
            $self.system.read_byte($self.cpu.registers.pc.wrapping_add(1)),
            $self.system.read_byte($self.cpu.registers.pc.wrapping_add(2)),
            $self.system.read_byte($self.cpu.registers.pc.wrapping_add(3)),
        )
    };
}

#[derive(Debug)]
pub enum ExecutionError {
    NoImpl { instruction: Instruction },
    MemoryWrite { address: u16 },
    MemoryRead { address: u16 },
}

pub struct Emulator {
    pub cpu: Cpu,
    pub system: System,

    graphics_enabled: bool,
}

impl Emulator {
    pub fn new() -> Result<Self, String> {
        Self::new_from_buffer(vec![0; 32 * 1024], true, None, None)
    }

    pub fn new_from_buffer(
        rom: Vec<u8>,
        graphics_enabled: bool,
        cpu_option: Option<Cpu>,
        serial_option: Option<Box<dyn Serial>>,
    ) -> Result<Self, String> {
        let serial = if let Some(s) = serial_option {
            s
        } else {
            Box::new(LogSerial::default())
        };
        let mut mmu = System::new(new_mbc_from_buffer(rom)?, serial);

        let mut result = Self {
            cpu: if let Some(cpu) = cpu_option {
                cpu
            } else {
                Cpu::new(&mut mmu)
            },
            system: mmu,

            graphics_enabled,
        };

        result.init();

        Ok(result)
    }

    #[instrument(skip_all)]
    fn init(&mut self) {
        trace_cpu_state!(self);

        self.cpu.current_instruction =
            Instruction::decode_instruction(self.system.read_byte(self.cpu.registers.pc));
        (self.cpu.registers.pc, _) = self.cpu.registers.pc.overflowing_add(1);
    }

    #[instrument(skip_all, fields(
        instruction = format!("{:?}", self.cpu.current_instruction)
    ))]
    pub fn step(&mut self) -> Result<(), ExecutionError> {
        if self.system.oam_transfer {
            self.system.oam_transfer_step();
        }

        let mut cpu_completed = false;
        if !self.cpu.halted {
            cpu_completed = self.cpu.step(&mut self.system)?
        }

        let mut v_blank_interrupt = false;
        if self.graphics_enabled {
            v_blank_interrupt = self.system.graphics.step();
            v_blank_interrupt |= self.system.graphics.step();
        }

        let timer_interrupt = self.system.io.timer.step()?;
        let joypad_interrupt = self.system.io.joypad.interrupt();

        if v_blank_interrupt {
            self.cpu
                .request_interrupt(&mut self.system, Interrupt::VBlank);
        } else if timer_interrupt {
            self.cpu
                .request_interrupt(&mut self.system, Interrupt::Timer);
        } else if joypad_interrupt {
            self.cpu
                .request_interrupt(&mut self.system, Interrupt::Joypad);
        }

        if self.system.io.interrupt_enable
            & <InterruptFlags as Into<u8>>::into(self.system.io.interrupt_flags)
            != 0
        {
            self.cpu.halted = false;
        }

        if !self.cpu.halted && cpu_completed {
            match self.cpu.current_instruction {
                Instruction::isr { .. } => {},
                _ => {
                    trace_cpu_state!(self);
                },
            }

            self.cpu.generic_fetch(&mut self.system)?;
        }

        Ok(())
    }
}

impl Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.cpu))
    }
}
