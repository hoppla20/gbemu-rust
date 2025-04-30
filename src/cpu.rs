mod alu;
mod instructions;

use lazy_static::lazy_static;

pub mod registers;

use instructions::Instruction;
use registers::Registers;

use crate::memory::mmu::Mmu;

#[derive(Debug)]
pub enum ExecutionError {
    NoImpl { instruction: Instruction },
    MemoryWrite { address: u16 },
    MemoryRead { address: u16 },
}

pub struct Cpu {
    pub registers: Registers,

    current_instruction: Instruction,
    current_instruction_cycle: u8,
}

lazy_static! {
    static ref INSTRUCTION_LOOKUP: [Instruction; 0x100] = {
        let mut instrs = [Instruction::unknown_opcode { opcode: 0 }; 0x100];

        for (i, instr) in instrs.iter_mut().enumerate() {
            *instr = Instruction::decode_instruction(i as u8);
        }

        instrs
    };
    static ref PREFIX_INSTRUCTION_LOOKUP: [Instruction; 0x100] = {
        let mut instrs = [Instruction::unknown_opcode { opcode: 0 }; 0x100];

        for (i, instr) in instrs.iter_mut().enumerate() {
            *instr = Instruction::decode_prefix_instruction(i as u8);
        }

        instrs
    };
}

impl Cpu {
    pub fn new_from_registers(mmu: &Mmu, registers: Registers) -> Self {
        let mut result = Cpu {
            registers,

            current_instruction: Instruction::nop,
            current_instruction_cycle: 0,
        };

        result.current_instruction =
            Instruction::decode_instruction(mmu.read_byte(result.registers.pc));

        result
    }

    pub fn new(mmu: &Mmu) -> Self {
        Cpu::new_from_registers(mmu, Registers::default())
    }

    pub fn new_dmg(mmu: &Mmu, carry_flags: bool) -> Self {
        let mut result = Cpu::new_from_registers(
            mmu,
            Registers {
                a: 0x01,
                f: 0x00,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                z: 0x00,
                pc: 0x0100,
                sp: 0xfffe,
            },
        );

        result.registers.set_flag_zero(true);
        result.registers.set_flag_subtraction(false);
        result.registers.set_flag_half_carry(carry_flags);
        result.registers.set_flag_carry(carry_flags);

        result
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> Result<bool, ExecutionError> {
        let completed = self.instruction_step(mmu)?;

        if completed {
            self.registers.pc += 1;
            let opcode = mmu.read_byte(self.registers.pc);

            #[cfg(test)]
            println!("Decoding opcode 0x{:02X}", opcode);

            match self.current_instruction {
                Instruction::prefix => {
                    self.current_instruction = PREFIX_INSTRUCTION_LOOKUP[opcode as usize];
                },
                _ => self.current_instruction = INSTRUCTION_LOOKUP[opcode as usize],
            }

            #[cfg(test)]
            println!("Decoded instruction {:02X?}", self.current_instruction);

            self.current_instruction_cycle = 0;
        } else {
            self.current_instruction_cycle += 1;

            return Ok(false);
        }

        Ok(completed)
    }
}

#[cfg(test)]
mod tests;
