mod alu;
pub mod instructions;
pub mod interrupts;
pub mod registers;

use self::instructions::Instruction;
use self::registers::Registers;
use crate::emulator::ExecutionError;
use crate::memory::mmu::Mmu;

use std::fmt::Debug;
use tracing::{debug, trace};

pub struct Cpu {
    pub registers: Registers,
    pub halted: bool,

    current_instruction: Instruction,
    current_instruction_cycle: u8,

    interrupt_enabled: bool,
    interrupt_enable_pending: bool,
}

impl Cpu {
    pub fn new_from_registers(mmu: &mut Mmu, registers: Registers) -> Self {
        let mut result = Cpu {
            registers,

            current_instruction: Instruction::nop,
            current_instruction_cycle: 0,

            interrupt_enabled: false,
            interrupt_enable_pending: false,

            halted: false,
        };

        result.trace_state(mmu);

        result.current_instruction =
            Instruction::decode_instruction(mmu.read_byte(result.registers.pc));
        (result.registers.pc, _) = result.registers.pc.overflowing_add(1);

        result
    }

    pub fn new_zeroed(mmu: &mut Mmu) -> Self {
        Cpu::new_from_registers(mmu, Registers::default())
    }

    pub fn new(mmu: &mut Mmu) -> Self {
        Cpu::new_from_registers(
            mmu,
            Registers {
                a: 0x01,
                f: if mmu.read_byte(0x14D) == 0x00 {
                    0b10000000
                } else {
                    0b10110000
                },
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                w: 0x00,
                z: 0x00,
                pc: 0x0100,
                sp: 0xfffe,
                cc: false,
            },
        )
    }

    pub fn read_byte_pc(&mut self, mmu: &mut Mmu) -> u8 {
        let byte = mmu.read_byte(self.registers.pc);
        (self.registers.pc, _) = self.registers.pc.overflowing_add(1);

        byte
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> Result<bool, ExecutionError> {
        if self.interrupt_enable_pending && !self.interrupt_enabled {
            self.interrupt_enabled = true;
            self.interrupt_enable_pending = false;
        }

        if !self.halted {
            if self.instruction_step(mmu)? {
                self.current_instruction_cycle = 0;
                Ok(true)
            } else {
                self.current_instruction_cycle += 1;
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub fn generic_fetch(&mut self, mmu: &mut Mmu) -> Result<(), ExecutionError> {
        match self.current_instruction {
            Instruction::isr { .. } => {},
            _ => self.trace_state(mmu),
        }

        if let Some(interrupt) = self.interrupt_check(mmu) {
            debug!(
                "Executing interrupt service routing for interrupt {:?}",
                interrupt
            );

            self.current_instruction = Instruction::isr { interrupt };
        } else {
            let opcode = self.read_byte_pc(mmu);

            debug!("Decoding opcode 0x{:02X}", opcode);

            match self.current_instruction {
                Instruction::prefix => {
                    self.current_instruction = Instruction::decode_prefix_instruction(opcode);
                },
                _ => self.current_instruction = Instruction::decode_instruction(opcode),
            }
        }

        debug!("Decoded instruction {:02X?}", self.current_instruction);

        self.current_instruction_cycle = 0;

        Ok(())
    }

    pub fn trace_state(&self, mmu: &Mmu) {
        match self.current_instruction {
            Instruction::prefix => {},
            _ => trace!(
                "{:?} PCMEM:{:02X},{:02X},{:02X},{:02X} SC:{:04X} IE:{:02X} IF:{:02X} {:?}",
                self,
                mmu.read_byte(self.registers.pc),
                mmu.read_byte(self.registers.pc.wrapping_add(1)),
                mmu.read_byte(self.registers.pc.wrapping_add(2)),
                mmu.read_byte(self.registers.pc.wrapping_add(3)),
                mmu.io.timer.system_counter,
                mmu.io.interrupt_enable,
                mmu.io.interrupt_flags,
                mmu.io.timer,
            ),
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X}",
                self.registers.a,
                self.registers.f,
                self.registers.b,
                self.registers.c,
                self.registers.d,
                self.registers.e,
                self.registers.h,
                self.registers.l,
                self.registers.sp,
                self.registers.pc,
            )
        )
    }
}

#[cfg(test)]
mod tests;
