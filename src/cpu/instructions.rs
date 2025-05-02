use std::fmt::Debug;

use crate::memory::mmu::Mmu;
use crate::utils::bit_operations::extract_bits;
use crate::utils::half_carry::half_carry_add_r8;

use super::{Cpu, ExecutionError};

macro_rules! panic_execuction {
    () => {
        panic!("During execution the CPU ran into an undefined state!")
    };
}

#[derive(Debug)]
pub enum DecodeError {
    UnknownOpcode { opcode: u8 },
    UnknownOperand { operand: u8 },
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
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

impl From<u8> for ArithmeticOperand {
    fn from(value: u8) -> Self {
        match value {
            x if x == ArithmeticOperand::B as u8 => ArithmeticOperand::B,
            x if x == ArithmeticOperand::C as u8 => ArithmeticOperand::C,
            x if x == ArithmeticOperand::D as u8 => ArithmeticOperand::D,
            x if x == ArithmeticOperand::E as u8 => ArithmeticOperand::E,
            x if x == ArithmeticOperand::H as u8 => ArithmeticOperand::H,
            x if x == ArithmeticOperand::L as u8 => ArithmeticOperand::L,
            x if x == ArithmeticOperand::IND_HL as u8 => ArithmeticOperand::IND_HL,
            x if x == ArithmeticOperand::A as u8 => ArithmeticOperand::A,
            _ => panic!(
                "Unknown arithmetic operand '{:02X}' while decoding instruction!",
                value
            ),
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

impl From<u8> for ArithmeticOperand16 {
    fn from(value: u8) -> Self {
        match value {
            x if x == ArithmeticOperand16::BC as u8 => ArithmeticOperand16::BC,
            x if x == ArithmeticOperand16::DE as u8 => ArithmeticOperand16::DE,
            x if x == ArithmeticOperand16::HL as u8 => ArithmeticOperand16::HL,
            x if x == ArithmeticOperand16::SP as u8 => ArithmeticOperand16::SP,
            _ => panic!(
                "Unknown 16-bit arithmetic operand '{:02X}' while decoding instruction!",
                value
            ),
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum MemoryOperand16 {
    BC,
    DE,
    HLI,
    HLD,
}

impl From<u8> for MemoryOperand16 {
    fn from(value: u8) -> Self {
        match value {
            x if x == MemoryOperand16::BC as u8 => MemoryOperand16::BC,
            x if x == MemoryOperand16::DE as u8 => MemoryOperand16::DE,
            x if x == MemoryOperand16::HLI as u8 => MemoryOperand16::HLI,
            x if x == MemoryOperand16::HLD as u8 => MemoryOperand16::HLD,
            _ => panic!(
                "Unknown memory operand '{:02X}' while decoding instruction!",
                value
            ),
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum StackOperand16 {
    BC,
    DE,
    HL,
    AF,
}

impl From<u8> for StackOperand16 {
    fn from(value: u8) -> Self {
        match value {
            x if x == StackOperand16::BC as u8 => StackOperand16::BC,
            x if x == StackOperand16::DE as u8 => StackOperand16::DE,
            x if x == StackOperand16::HL as u8 => StackOperand16::HL,
            x if x == StackOperand16::AF as u8 => StackOperand16::AF,
            _ => panic!(
                "Unknown condition '{:02X}' while decoding instruction!",
                value
            ),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

impl From<u8> for Condition {
    fn from(value: u8) -> Self {
        match value {
            x if x == Condition::NZ as u8 => Condition::NZ,
            x if x == Condition::Z as u8 => Condition::Z,
            x if x == Condition::NC as u8 => Condition::NC,
            x if x == Condition::C as u8 => Condition::C,
            _ => panic!(
                "Unknown condition '{:02X}' while decoding instruction!",
                value
            ),
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // special
    nop,
    stop,
    halt,
    di,
    ei,

    // 8-bit arithmetics
    add_a_r8 {
        operand: ArithmeticOperand,
    },
    adc_a_r8 {
        operand: ArithmeticOperand,
    },
    sub_a_r8 {
        operand: ArithmeticOperand,
    },
    sbc_a_r8 {
        operand: ArithmeticOperand,
    },
    and_a_r8 {
        operand: ArithmeticOperand,
    },
    xor_a_r8 {
        operand: ArithmeticOperand,
    },
    or_a_r8 {
        operand: ArithmeticOperand,
    },
    cp_a_r8 {
        operand: ArithmeticOperand,
    },
    inc_r8 {
        operand: ArithmeticOperand,
    },
    dec_r8 {
        operand: ArithmeticOperand,
    },
    add_a_n8,
    adc_a_n8,
    sub_a_n8,
    sbc_a_n8,
    and_a_n8,
    xor_a_n8,
    or_a_n8,
    cp_a_n8,
    add_sp_n8,
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
    inc_r16 {
        operand: ArithmeticOperand16,
    },
    dec_r16 {
        operand: ArithmeticOperand16,
    },
    add_hl_r16 {
        operand: ArithmeticOperand16,
    },

    // prefix
    prefix,
    rlc_r8 {
        operand: ArithmeticOperand,
    },
    rrc_r8 {
        operand: ArithmeticOperand,
    },
    rl_r8 {
        operand: ArithmeticOperand,
    },
    rr_r8 {
        operand: ArithmeticOperand,
    },
    sla_r8 {
        operand: ArithmeticOperand,
    },
    sra_r8 {
        operand: ArithmeticOperand,
    },
    swap_r8 {
        operand: ArithmeticOperand,
    },
    srl_r8 {
        operand: ArithmeticOperand,
    },
    bit_b3_r8 {
        index: u8,
        operand: ArithmeticOperand,
    },
    res_b3_r8 {
        index: u8,
        operand: ArithmeticOperand,
    },
    set_b3_r8 {
        index: u8,
        operand: ArithmeticOperand,
    },

    // 8-bit load operations
    ld_r8_n8 {
        operand: ArithmeticOperand,
    },
    ld_r8_r8 {
        operand_a: ArithmeticOperand,
        operand_b: ArithmeticOperand,
    },
    ld_ind_n16_a,
    ld_a_ind_n16,
    ld_ind_r16mem_a {
        operand: MemoryOperand16,
    },
    ld_a_ind_r16mem {
        operand: MemoryOperand16,
    },
    ldh_ind_c_a,
    ldh_a_ind_c,
    ldh_ind_n8_a,
    ldh_a_ind_n8,

    // 16-bit load operations
    ld_r16_n16 {
        operand: ArithmeticOperand16,
    },
    ld_ind_n16_sp,
    ld_sp_hl,
    ld_hl_sp_n8,

    // jumps
    jr_i8,
    jr_cond_i8 {
        condition: Condition,
    },
    jp_n16,
    jp_hl,
    jp_cond_n16 {
        condition: Condition,
    },
    call_n16,
    call_cond_n16 {
        condition: Condition,
    },
    rst_tgt3 {
        target_address: u16,
    },
    ret,
    reti,
    ret_cond {
        condition: Condition,
    },

    // stack
    pop_r16stk {
        operand: StackOperand16,
    },
    push_r16stk {
        operand: StackOperand16,
    },

    // errors
    unknown_opcode {
        opcode: u8,
    },
    unknown_prefix_opcode {
        opcode: u8,
    },
}

impl Instruction {
    pub(super) fn decode_prefix_instruction(opcode: u8) -> Self {
        match extract_bits!(opcode: u8, 6, 7) {
            0b00 => match extract_bits!(opcode: u8, 3, 5) {
                0b000 => Self::rlc_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b001 => Self::rrc_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b010 => Self::rl_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b011 => Self::rr_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b100 => Self::sla_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b101 => Self::sra_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b110 => Self::swap_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                0b111 => Self::srl_r8 {
                    operand: extract_bits!(opcode:u8,0,2).into(),
                },
                _ => panic!(
                    "Something went wrong while decoding prefix instruction {:02X}!",
                    opcode
                ),
            },
            0b01 => Self::bit_b3_r8 {
                operand: extract_bits!(opcode:u8,0,2).into(),
                index: extract_bits!(opcode: u8, 3, 5),
            },
            0b10 => Self::res_b3_r8 {
                operand: extract_bits!(opcode:u8,0,2).into(),
                index: extract_bits!(opcode: u8, 3, 5),
            },
            0b11 => Self::set_b3_r8 {
                operand: extract_bits!(opcode:u8,0,2).into(),
                index: extract_bits!(opcode: u8, 3, 5),
            },
            _ => panic!(
                "Something went wrong while decoding prefix instruction {:02X}!",
                opcode
            ),
        }
    }

    pub(super) fn decode_instruction(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Self::nop,
            0x10 => Self::stop,
            0x76 => Self::halt,
            0xCB => Self::prefix,

            0x18 => Self::jr_i8,
            0x20 | 0x28 | 0x30 | 0x38 => Self::jr_cond_i8 {
                condition: extract_bits!(opcode: u8, 3, 4).into(),
            },

            0x01 | 0x11 | 0x21 | 0x31 => Self::ld_r16_n16 {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0x09 | 0x19 | 0x29 | 0x39 => Self::add_hl_r16 {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0x02 | 0x12 | 0x22 | 0x32 => Self::ld_ind_r16mem_a {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0x0A | 0x1A | 0x2A | 0x3A => Self::ld_a_ind_r16mem {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0x03 | 0x13 | 0x23 | 0x33 => Self::inc_r16 {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0x0B | 0x1B | 0x2B | 0x3B => Self::dec_r16 {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => Self::inc_r8 {
                operand: extract_bits!(opcode: u8, 3, 5).into(),
            },

            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => Self::dec_r8 {
                operand: extract_bits!(opcode: u8, 3, 5).into(),
            },

            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => Self::ld_r8_n8 {
                operand: extract_bits!(opcode: u8, 3, 5).into(),
            },

            0x07 => Self::rlca,
            0x0F => Self::rrca,
            0x17 => Self::rla,
            0x1F => Self::rra,

            0x27 => Self::daa,
            0x2F => Self::cpl,
            0x37 => Self::scf,
            0x3F => Self::ccf,

            0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 | 0x41 | 0x49 | 0x51 | 0x59
            | 0x61 | 0x69 | 0x71 | 0x79 | 0x42 | 0x4A | 0x52 | 0x5A | 0x62 | 0x6A | 0x72 | 0x7A
            | 0x43 | 0x4B | 0x53 | 0x5B | 0x63 | 0x6B | 0x73 | 0x7B | 0x44 | 0x4C | 0x54 | 0x5C
            | 0x64 | 0x6C | 0x74 | 0x7C | 0x45 | 0x4D | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D
            | 0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E | 0x47 | 0x4F | 0x57 | 0x5F | 0x67
            | 0x6F | 0x77 | 0x7F => Self::ld_r8_r8 {
                operand_a: extract_bits!(opcode: u8, 3, 5).into(),
                operand_b: extract_bits!(opcode: u8, 0, 2).into(),
            },

            0x80..=0x87 => Self::add_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0x88..=0x8F => Self::adc_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0x90..=0x97 => Self::sub_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0x98..=0x9F => Self::sbc_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0xA0..=0xA7 => Self::and_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0xA8..=0xAF => Self::xor_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0xB0..=0xB7 => Self::or_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },
            0xB8..=0xBF => Self::cp_a_r8 {
                operand: extract_bits!(opcode: u8, 0, 2).into(),
            },

            0xC0 | 0xC8 | 0xD0 | 0xD8 => Self::ret_cond {
                condition: extract_bits!(opcode: u8, 3, 4).into(),
            },

            0xE0 => Self::ldh_ind_n8_a,
            0xF0 => Self::ldh_a_ind_n8,

            0xE8 => Self::add_sp_n8,
            0xF8 => Self::ld_hl_sp_n8,

            0xC1 | 0xD1 | 0xE1 | 0xF1 => Self::pop_r16stk {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0xC9 => Self::ret,
            0xD9 => Self::reti,
            0xE9 => Self::jp_hl,
            0xF9 => Self::ld_sp_hl,

            0xC2 | 0xCA | 0xD2 | 0xDA => Self::jp_cond_n16 {
                condition: extract_bits!(opcode: u8, 3, 4).into(),
            },

            0xE2 => Self::ldh_ind_c_a,
            0xEA => Self::ld_ind_n16_a,
            0xF2 => Self::ldh_a_ind_c,
            0xFA => Self::ld_a_ind_n16,

            0xC3 => Self::jp_n16,
            0xF3 => Self::di,
            0xFB => Self::ei,

            0xC4 | 0xCC | 0xD4 | 0xDC => Self::call_cond_n16 {
                condition: extract_bits!(opcode: u8, 3, 4).into(),
            },

            0xC5 | 0xD5 | 0xE5 | 0xF5 => Self::push_r16stk {
                operand: extract_bits!(opcode: u8, 4, 5).into(),
            },

            0xCD => Self::call_n16,

            0xC6 => Self::add_a_n8,
            0xCE => Self::adc_a_n8,
            0xD6 => Self::sub_a_n8,
            0xDE => Self::sbc_a_n8,
            0xE6 => Self::and_a_n8,
            0xEE => Self::xor_a_n8,
            0xF6 => Self::or_a_n8,
            0xFE => Self::cp_a_n8,

            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => Self::rst_tgt3 {
                target_address: (extract_bits!(opcode: u8, 3, 4) as u16) * 8,
            },

            _ => Instruction::unknown_opcode { opcode },
        }
    }
}

macro_rules! instr_a_r8_match {
    ($self:ident, $operand:ident, $instr:ident, $mmc:ident) => {
        match ($operand, $self.current_instruction_cycle) {
            (ArithmeticOperand::IND_HL, 0) => {
                $self.registers.z = $mmc.read_byte($self.registers.get_hl());
                Ok(false)
            },
            (ArithmeticOperand::IND_HL, 1) | (_, 0) => {
                $self.registers.$instr($operand);
                Ok(true)
            },
            _ => panic_execuction!(),
        }
    };
}

macro_rules! instr_a_n8_match {
    ($self:ident, $instr:ident, $mmu:ident) => {
        match $self.current_instruction_cycle {
            0 => {
                $self.registers.z = $mmu.read_byte($self.registers.pc);
                Ok(())
            },
            1 => {
                $self.registers.$instr(ArithmeticOperand::IND_HL);
                Ok(())
            },
            _ => panic_execuction!(),
        }
    };
}

macro_rules! instr_r8_match {
    ($self:ident, $operand:ident, $instr:ident, $mmu:ident) => {
        match ($operand, $self.current_instruction_cycle) {
            (ArithmeticOperand::IND_HL, 0) => {
                $self.registers.z = $mmu.read_byte($self.registers.get_hl());
                Ok(false)
            },
            (ArithmeticOperand::IND_HL, 1) => {
                $mmu.write_byte($self.registers.get_hl(), $self.registers.$instr($operand));
                Ok(false)
            },
            (ArithmeticOperand::IND_HL, 2) => Ok(true),
            (_, 0) => {
                $self.registers.$instr($operand);
                Ok(true)
            },
            _ => panic_execuction!(),
        }
    };
}

impl Cpu {
    fn jump_relative(&mut self) {
        let z_sign = self.registers.z >> 7;
        let (result, overflow) = self
            .registers
            .z
            .overflowing_add((self.registers.pc & 0x00FF) as u8);
        self.registers.z = result;
        let adj = if overflow && (z_sign != 1) {
            1
        } else if !overflow && (z_sign == 1) {
            -1
        } else {
            0
        };
        self.registers.w = ((self.registers.pc >> 8) as i16 + adj) as u8;
    }

    pub(super) fn instruction_step(&mut self, mmu: &mut Mmu) -> Result<bool, ExecutionError> {
        match self.current_instruction {
            //special
            Instruction::nop => Ok(true),
            Instruction::di => {
                self.interrupt_enabled = false;
                Ok(true)
            },
            Instruction::ei => {
                self.interrupt_enabled = true;
                Ok(true)
            },

            // 8-bit arithmetics
            Instruction::add_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_add_a_r8, mmu)
            },
            Instruction::adc_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_adc_a_r8, mmu)
            },
            Instruction::sub_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_sub_a_r8, mmu)
            },
            Instruction::sbc_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_sbc_a_r8, mmu)
            },
            Instruction::and_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_and_a_r8, mmu)
            },
            Instruction::xor_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_xor_a_r8, mmu)
            },
            Instruction::or_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_or_a_r8, mmu)
            },
            Instruction::cp_a_r8 { operand } => {
                instr_a_r8_match!(self, operand, alu_cp_a_r8, mmu)
            },
            Instruction::inc_r8 { operand } => {
                instr_r8_match!(self, operand, alu_inc_r8, mmu)
            },
            Instruction::dec_r8 { operand } => {
                instr_r8_match!(self, operand, alu_dec_r8, mmu)
            },

            // 16-bit arithmetics
            Instruction::inc_r16 { operand } => match self.current_instruction_cycle {
                0 => {
                    let (temp, _) = self
                        .registers
                        .get_arithmetic_target_r16(operand)
                        .overflowing_add(1);
                    self.registers.set_arithmetic_target_r16(operand, temp);
                    Ok(false)
                },
                1 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::dec_r16 { operand } => match self.current_instruction_cycle {
                0 => {
                    let (temp, _) = self
                        .registers
                        .get_arithmetic_target_r16(operand)
                        .overflowing_sub(1);
                    self.registers.set_arithmetic_target_r16(operand, temp);
                    Ok(false)
                },
                1 => Ok(true),
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
                    Ok(false)
                },
                1 => {
                    let b = (self.registers.get_arithmetic_target_r16(operand) >> 8) as u8;
                    let (temp, overflow) = self.registers.h.overflowing_add(b);
                    self.registers.set_flag_subtraction(false);
                    self.registers
                        .set_flag_half_carry(half_carry_add_r8(self.registers.h, b));
                    self.registers.set_flag_carry(overflow);
                    self.registers.h = temp;
                    Ok(true)
                },
                _ => panic_execuction!(),
            },

            // prefix
            Instruction::prefix => Ok(true),
            Instruction::rlc_r8 { operand } => instr_r8_match!(self, operand, alu_rlc_r8, mmu),
            Instruction::rrc_r8 { operand } => instr_r8_match!(self, operand, alu_rrc_r8, mmu),
            Instruction::sla_r8 { operand } => instr_r8_match!(self, operand, alu_sla_r8, mmu),
            Instruction::sra_r8 { operand } => instr_r8_match!(self, operand, alu_sra_r8, mmu),
            Instruction::swap_r8 { operand } => instr_r8_match!(self, operand, alu_swap_r8, mmu),
            Instruction::srl_r8 { operand } => instr_r8_match!(self, operand, alu_srl_r8, mmu),
            Instruction::bit_b3_r8 { index, operand } => {
                match (operand, self.current_instruction_cycle) {
                    (ArithmeticOperand::IND_HL, 0) => {
                        self.registers.z = mmu.read_byte(self.registers.get_hl());
                        Ok(false)
                    },
                    (ArithmeticOperand::IND_HL, 1) => {
                        self.registers.alu_bit_b3_r8(index, operand);
                        Ok(true)
                    },
                    (_, 0) => {
                        self.registers.alu_bit_b3_r8(index, operand);
                        Ok(true)
                    },
                    _ => panic_execuction!(),
                }
            },
            Instruction::res_b3_r8 { index, operand } => {
                match (operand, self.current_instruction_cycle) {
                    (ArithmeticOperand::IND_HL, 0) => {
                        self.registers.z = mmu.read_byte(self.registers.get_hl());
                        Ok(false)
                    },
                    (ArithmeticOperand::IND_HL, 1) => {
                        mmu.write_byte(
                            self.registers.get_hl(),
                            self.registers.alu_res_b3_r8(index, operand),
                        );
                        Ok(false)
                    },
                    (ArithmeticOperand::IND_HL, 2) => Ok(true),
                    (_, 0) => {
                        self.registers.alu_res_b3_r8(index, operand);
                        Ok(true)
                    },
                    _ => panic_execuction!(),
                }
            },
            Instruction::set_b3_r8 { index, operand } => {
                match (operand, self.current_instruction_cycle) {
                    (ArithmeticOperand::IND_HL, 0) => {
                        self.registers.z = mmu.read_byte(self.registers.get_hl());
                        Ok(false)
                    },
                    (ArithmeticOperand::IND_HL, 1) => {
                        mmu.write_byte(
                            self.registers.get_hl(),
                            self.registers.alu_set_b3_r8(index, operand),
                        );
                        Ok(false)
                    },
                    (ArithmeticOperand::IND_HL, 2) => Ok(true),
                    (_, 0) => {
                        self.registers.alu_set_b3_r8(index, operand);
                        Ok(true)
                    },
                    _ => panic_execuction!(),
                }
            },

            // 8-bit load operations
            Instruction::ld_r8_n8 { operand } => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.registers
                        .set_arithmetic_target_r8(operand, self.registers.z);
                    Ok(true)
                },
                _ => panic_execuction!(),
            },
            Instruction::ld_r8_r8 {
                operand_a,
                operand_b,
            } => match self.current_instruction_cycle {
                0 => {
                    self.registers.set_arithmetic_target_r8(
                        operand_a,
                        self.registers.get_arithmetic_target_r8(operand_b),
                    );
                    Ok(true)
                },
                _ => panic_execuction!(),
            },
            Instruction::ld_ind_n16_a => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.registers.w = self.read_byte_pc(mmu);
                    Ok(false)
                },
                2 => {
                    mmu.write_byte(self.registers.get_wz(), self.registers.a);
                    Ok(false)
                },
                3 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::ld_ind_r16mem_a { operand } => match self.current_instruction_cycle {
                0 => {
                    mmu.write_byte(self.registers.get_memory_operand(operand), self.registers.a);
                    Ok(false)
                },
                1 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::ld_a_ind_r16mem { operand } => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = mmu.read_byte(self.registers.get_memory_operand(operand));
                    Ok(false)
                },
                1 => {
                    self.registers.a = self.registers.z;
                    Ok(true)
                },
                _ => panic_execuction!(),
            },
            Instruction::ldh_ind_n8_a => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    mmu.write_byte(self.registers.z as u16 + 0xFF00, self.registers.a);
                    Ok(false)
                },
                2 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::ldh_a_ind_n8 => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.registers.z = mmu.read_byte(self.registers.z as u16 + 0xFF00);
                    Ok(false)
                },
                2 => {
                    self.registers.a = self.registers.z;
                    Ok(true)
                },
                _ => panic_execuction!(),
            },

            // 16-bit load operations
            Instruction::ld_r16_n16 { operand } => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.registers.w = self.read_byte_pc(mmu);
                    Ok(false)
                },
                2 => {
                    self.registers
                        .set_arithmetic_target_r16(operand, self.registers.get_wz());
                    Ok(true)
                },
                _ => panic_execuction!(),
            },

            // jumps
            Instruction::jr_i8 => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.jump_relative();
                    Ok(false)
                },
                2 => {
                    self.registers.pc = self.registers.get_wz();
                    Ok(true)
                },
                _ => panic_execuction!(),
            },
            Instruction::jr_cond_i8 { condition } => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    self.registers.check_condition(condition);
                    Ok(false)
                },
                1 => {
                    if self.registers.cc {
                        self.jump_relative();
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                },
                2 => {
                    self.registers.pc = self.registers.get_wz();
                    Ok(true)
                },
                _ => panic_execuction!(),
            },
            Instruction::jp_n16 => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.registers.w = self.read_byte_pc(mmu);
                    Ok(false)
                },
                2 => {
                    self.registers.pc = self.registers.get_wz();
                    Ok(false)
                },
                3 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::call_n16 => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = self.read_byte_pc(mmu);
                    Ok(false)
                },
                1 => {
                    self.registers.w = self.read_byte_pc(mmu);
                    Ok(false)
                },
                2 => {
                    self.registers.sp -= 1;
                    Ok(false)
                },
                3 => {
                    mmu.write_byte(self.registers.sp, (self.registers.pc >> 8) as u8);
                    self.registers.sp -= 1;
                    Ok(false)
                },
                4 => {
                    mmu.write_byte(self.registers.sp, (self.registers.pc & 0x00FF) as u8);
                    self.registers.pc = self.registers.get_wz();
                    Ok(false)
                },
                5 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::ret => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = mmu.read_byte(self.registers.sp);
                    self.registers.sp += 1;
                    Ok(false)
                },
                1 => {
                    self.registers.w = mmu.read_byte(self.registers.sp);
                    self.registers.sp += 1;
                    Ok(false)
                },
                2 => {
                    self.registers.pc = self.registers.get_wz();
                    Ok(false)
                },
                3 => Ok(true),
                _ => panic_execuction!(),
            },

            // stack
            Instruction::push_r16stk { operand } => match self.current_instruction_cycle {
                0 => {
                    self.registers.sp -= 1;
                    Ok(false)
                },
                1 => {
                    mmu.write_byte(
                        self.registers.sp,
                        (self.registers.get_stack_operand(operand) >> 8) as u8,
                    );
                    self.registers.sp -= 1;
                    Ok(false)
                },
                2 => {
                    mmu.write_byte(
                        self.registers.sp,
                        (self.registers.get_stack_operand(operand) & 0x00FF) as u8,
                    );
                    Ok(false)
                },
                3 => Ok(true),
                _ => panic_execuction!(),
            },
            Instruction::pop_r16stk { operand } => match self.current_instruction_cycle {
                0 => {
                    self.registers.z = mmu.read_byte(self.registers.sp);
                    self.registers.sp += 1;
                    Ok(false)
                },
                1 => {
                    self.registers.w = mmu.read_byte(self.registers.sp);
                    self.registers.sp += 1;
                    Ok(false)
                },
                2 => {
                    self.registers
                        .set_stack_operand(operand, self.registers.get_wz());
                    Ok(true)
                },
                _ => panic_execuction!(),
            },

            _ => Err(ExecutionError::NoImpl {
                instruction: self.current_instruction,
            }),
        }
    }
}
