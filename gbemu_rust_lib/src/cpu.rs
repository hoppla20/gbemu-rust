mod alu;

pub mod instructions;
pub mod interrupts;
pub mod registers;

use crate::emulator::ExecutionError;
use crate::system::System;

use self::instructions::Instruction;
use self::registers::Registers;

use std::fmt::Debug;
use tracing::{debug, instrument};

pub struct Cpu {
    pub registers: Registers,
    pub halted: bool,

    pub current_instruction: Instruction,
    pub current_instruction_cycle: u8,

    pub interrupt_enabled: bool,
    pub interrupt_enable_pending: bool,

    pub z_sign: bool,
}

impl Cpu {
    pub fn new_from_registers(registers: Registers) -> Self {
        Cpu {
            registers,
            halted: false,

            current_instruction: Instruction::nop,
            current_instruction_cycle: 0,

            interrupt_enabled: false,
            interrupt_enable_pending: false,

            z_sign: false,
        }
    }

    pub fn new_zeroed() -> Self {
        Cpu::new_from_registers(Registers::default())
    }

    pub fn new(mmu: &mut System) -> Self {
        Cpu::new_from_registers(Registers {
            a: 0x01,
            f: if mmu.read_byte(0x14D) == 0x00 {
                0b1000_0000
            } else {
                0b1011_0000
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
        })
    }

    pub fn read_byte_pc(&mut self, mmu: &mut System) -> u8 {
        let byte = mmu.read_byte(self.registers.pc);
        (self.registers.pc, _) = self.registers.pc.overflowing_add(1);

        byte
    }

    pub fn step(&mut self, mmu: &mut System) -> Result<bool, ExecutionError> {
        if self.interrupt_enable_pending && !self.interrupt_enabled {
            self.interrupt_enabled = true;
            self.interrupt_enable_pending = false;
        }

        if !self.halted {
            if self.instruction_step(mmu)? {
                self.current_instruction_cycle = 0;
                Ok(true)
            } else {
                self.current_instruction_cycle = self.current_instruction_cycle.wrapping_add(1);
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    #[instrument(skip_all)]
    pub fn generic_fetch(&mut self, mmu: &mut System) -> Result<(), ExecutionError> {
        if let Some(interrupt) = self.interrupt_check(mmu) {
            debug!(
                "Executing interrupt service routing for interrupt {:?}",
                interrupt
            );

            self.current_instruction = Instruction::isr { interrupt };
        } else {
            let opcode = self.read_byte_pc(mmu);

            self.current_instruction = Instruction::decode_instruction(opcode);
        }

        self.current_instruction_cycle = 0;

        Ok(())
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
