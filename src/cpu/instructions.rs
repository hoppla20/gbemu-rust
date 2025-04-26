use std::fmt::Debug;

use crate::memory::mbcs::Mbc;
use crate::utils::bit_operations::extract_bits;
use crate::utils::half_carry::half_carry_add_r8;

use super::Cpu;

macro_rules! panic_execuction {
    () => {
        panic!("During execution the CPU ran into an undefined state!")
    };
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOperand {
    B,
    C,
    D,
    E,
    H,
    L,
    IND_HL,
    A,
}

impl TryFrom<u8> for ArithmeticOperand {
    type Error = ExecutionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == ArithmeticOperand::B as u8 => Ok(ArithmeticOperand::B),
            x if x == ArithmeticOperand::C as u8 => Ok(ArithmeticOperand::C),
            x if x == ArithmeticOperand::D as u8 => Ok(ArithmeticOperand::D),
            x if x == ArithmeticOperand::E as u8 => Ok(ArithmeticOperand::E),
            x if x == ArithmeticOperand::H as u8 => Ok(ArithmeticOperand::H),
            x if x == ArithmeticOperand::L as u8 => Ok(ArithmeticOperand::L),
            x if x == ArithmeticOperand::IND_HL as u8 => Ok(ArithmeticOperand::IND_HL),
            x if x == ArithmeticOperand::A as u8 => Ok(ArithmeticOperand::A),
            _ => Err(ExecutionError::DecodeOperand { operand: value }),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOperand16 {
    BC,
    DE,
    HL,
    SP,
}

impl TryFrom<u8> for ArithmeticOperand16 {
    type Error = ExecutionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == ArithmeticOperand16::BC as u8 => Ok(ArithmeticOperand16::BC),
            x if x == ArithmeticOperand16::DE as u8 => Ok(ArithmeticOperand16::DE),
            x if x == ArithmeticOperand16::HL as u8 => Ok(ArithmeticOperand16::HL),
            x if x == ArithmeticOperand16::SP as u8 => Ok(ArithmeticOperand16::SP),
            _ => Err(ExecutionError::DecodeOperand { operand: value }),
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NOP,

    // 8-bit arithmetics
    add_a_r8 { operand: ArithmeticOperand },
    adc_a_r8 { operand: ArithmeticOperand },
    sub_a_r8 { operand: ArithmeticOperand },
    sbc_a_r8 { operand: ArithmeticOperand },
    and_a_r8 { operand: ArithmeticOperand },
    xor_a_r8 { operand: ArithmeticOperand },
    or_a_r8 { operand: ArithmeticOperand },
    cp_a_r8 { operand: ArithmeticOperand },
    inc_r8 { operand: ArithmeticOperand },
    dec_r8 { operand: ArithmeticOperand },
    inc_r16 { operand: ArithmeticOperand16 },
    dec_r16 { operand: ArithmeticOperand16 },
    cpl,
    daa,

    // 8-bit rotation
    rlca,
    rrca,
    rla,
    rra,

    // carry flag
    scf,
    ccf,

    // 16-bit arithmetics
    add_hl_r16 { operand: ArithmeticOperand16 },
}

#[derive(Debug)]
pub enum ExecutionError {
    NoImpl { instruction: Instruction },
    UnknownOpcode { opcode: u8 },
    MemoryWrite { address: u16 },
    MemoryRead { address: u16 },
    DecodeOperand { operand: u8 },
}

macro_rules! instr_a_r8_match {
    ($self:ident, $operand:ident, $instr:ident, $mbc:ident) => {
        match ($operand, $self.current_instruction_cycle) {
            (ArithmeticOperand::IND_HL, 0) => {
                $self.registers.z = $mbc.read_byte($self.registers.get_hl());
                Ok(())
            },
            (ArithmeticOperand::IND_HL, 1) | (_, 0) => {
                $self.registers.$instr($operand);
                $self.current_instruction_completed = true;
                Ok(())
            },
            _ => panic_execuction!(),
        }
    };
}

macro_rules! arithmetic_operand_0_2 {
    ($opcode:ident) => {
        extract_bits!($opcode: u8, 0, 2).try_into()?
    };
}

macro_rules! arithmetic_operand_3_5 {
    ($opcode:ident) => {
        extract_bits!($opcode: u8, 3, 5).try_into()?
    };
}

macro_rules! arithmetic_operand_16_4_5 {
    ($opcode:ident) => {
        extract_bits!($opcode: u8, 4, 5).try_into()?
    };
}

impl Cpu {
    pub fn decode_instruction(&self, mbc: &impl Mbc) -> Result<Instruction, ExecutionError> {
        let opcode = mbc.read_byte(self.registers.pc);

        if opcode == 0x00 {
            return Ok(Instruction::NOP);
        }

        match extract_bits!(opcode: u8, 6, 7) {
            0b00 => match extract_bits!(opcode: u8, 0, 3) {
                0b0011 => Ok(Instruction::inc_r16 {
                    operand: arithmetic_operand_16_4_5!(opcode),
                }),
                0b1011 => Ok(Instruction::dec_r16 {
                    operand: arithmetic_operand_16_4_5!(opcode),
                }),
                0b1001 => Ok(Instruction::add_hl_r16 {
                    operand: arithmetic_operand_16_4_5!(opcode),
                }),
                _ => match extract_bits!(opcode: u8, 0, 2) {
                    0b100 => Ok(Instruction::inc_r8 {
                        operand: arithmetic_operand_3_5!(opcode),
                    }),
                    0b101 => Ok(Instruction::dec_r8 {
                        operand: arithmetic_operand_3_5!(opcode),
                    }),
                    0b111 => match extract_bits!(opcode: u8, 3, 5) {
                        0b000 => Ok(Instruction::rla),
                        0b001 => Ok(Instruction::rrca),
                        0b010 => Ok(Instruction::rla),
                        0b011 => Ok(Instruction::rra),
                        0b100 => Ok(Instruction::daa),
                        0b101 => Ok(Instruction::cpl),
                        0b110 => Ok(Instruction::scf),
                        0b111 => Ok(Instruction::ccf),
                        _ => Err(ExecutionError::UnknownOpcode { opcode }),
                    },
                    _ => Err(ExecutionError::UnknownOpcode { opcode }),
                },
            },
            0b01 => Err(ExecutionError::UnknownOpcode { opcode }),
            0b10 => match extract_bits!(opcode: u8, 3, 5) {
                0b000 => Ok(Instruction::add_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b001 => Ok(Instruction::adc_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b010 => Ok(Instruction::sub_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b011 => Ok(Instruction::sbc_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b100 => Ok(Instruction::and_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b101 => Ok(Instruction::xor_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b110 => Ok(Instruction::or_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                0b111 => Ok(Instruction::cp_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                }),
                _ => Err(ExecutionError::UnknownOpcode { opcode }),
            },
            _ => Err(ExecutionError::UnknownOpcode { opcode }),
        }
    }

    fn instruction_step(&mut self, mbc: &impl Mbc) -> Result<(), ExecutionError> {
        match self.current_instruction {
            Instruction::NOP => {
                self.current_instruction_completed = true;
                Ok(())
            },

            // 8-bit arithmetics
            Instruction::add_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_add_a_r8, mbc)
            },
            Instruction::adc_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_adc_a_r8, mbc)
            },
            Instruction::sub_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_sub_a_r8, mbc)
            },
            Instruction::sbc_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_sbc_a_r8, mbc)
            },
            Instruction::and_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_and_a_r8, mbc)
            },
            Instruction::xor_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_xor_a_r8, mbc)
            },
            Instruction::or_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_or_a_r8, mbc)
            },
            Instruction::cp_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_cp_a_r8, mbc)
            },
            Instruction::inc_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_inc_r8, mbc)
            },
            Instruction::dec_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_dec_r8, mbc)
            },

            // 16-bit arithmetics
            Instruction::inc_r16 { operand } => match self.current_instruction_cycle {
                0 => {
                    let (temp, _) = self
                        .registers
                        .get_arithmetic_target_r16(operand)
                        .overflowing_add(1);
                    self.registers.set_arithmetic_target_r16(operand, temp);
                    Ok(())
                },
                1 => {
                    self.current_instruction_completed = true;
                    Ok(())
                },
                _ => panic_execuction!(),
            },
            Instruction::dec_r16 { operand } => match self.current_instruction_cycle {
                0 => {
                    let (temp, _) = self
                        .registers
                        .get_arithmetic_target_r16(operand)
                        .overflowing_sub(1);
                    self.registers.set_arithmetic_target_r16(operand, temp);
                    Ok(())
                },
                1 => {
                    self.current_instruction_completed = true;
                    Ok(())
                },
                _ => panic_execuction!(),
            },
            Instruction::add_hl_r16 { operand } => match self.current_instruction_cycle {
                0 => {
                    let b = (self.registers.get_arithmetic_target_r16(operand) & 0x00FF) as u8;
                    let (temp, overflow) = self.registers.l.overflowing_add(b);
                    self.registers.set_flag_subtraction(false);
                    self.registers
                        .set_flag_half_carry(half_carry_add_r8(self.registers.l, b));
                    self.registers.set_flag_carry(overflow);
                    self.registers.l = temp;
                    Ok(())
                },
                1 => {
                    let b = (self.registers.get_arithmetic_target_r16(operand) >> 8) as u8;
                    let (temp, overflow) = self.registers.h.overflowing_add(b);
                    self.registers.set_flag_subtraction(false);
                    self.registers
                        .set_flag_half_carry(half_carry_add_r8(self.registers.h, b));
                    self.registers.set_flag_carry(overflow);
                    self.registers.h = temp;
                    self.current_instruction_completed = true;
                    Ok(())
                },
                _ => panic_execuction!(),
            },

            _ => Err(ExecutionError::NoImpl {
                instruction: self.current_instruction,
            }),
        }
    }

    pub fn step(&mut self, mbc: &impl Mbc) -> Result<(), ExecutionError> {
        self.instruction_step(mbc)?;

        if self.current_instruction_completed {
            self.registers.pc += 1;
            self.current_instruction = self.decode_instruction(mbc)?;
            self.current_instruction_completed = false;
            self.current_instruction_cycle = 0;
        } else {
            self.current_instruction_cycle += 1;
        }

        Ok(())
    }
}
