mod alu;
mod instructions;

pub mod registers;

use instructions::Instruction;
use registers::Registers;

use crate::memory::mbcs::Mbc;

pub struct Cpu {
    pub registers: Registers,

    current_instruction: Instruction,
    current_instruction_completed: bool,
    current_instruction_cycle: u8,
}

impl Cpu {
    pub fn new_dmg(mbc: &impl Mbc, carry_flags: bool) -> Self {
        let mut result = Cpu {
            registers: Registers {
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

            current_instruction: Instruction::nop,
            current_instruction_completed: false,
            current_instruction_cycle: 0,
        };

        match result.decode_instruction(mbc) {
            Ok(instr) => result.current_instruction = instr,
            Err(err) => panic!("{:?}", err),
        }

        result.registers.set_flag_zero(true);
        result.registers.set_flag_subtraction(false);
        result.registers.set_flag_half_carry(carry_flags);
        result.registers.set_flag_carry(carry_flags);

        result
    }
}
