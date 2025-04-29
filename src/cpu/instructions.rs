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

impl TryFrom<u8> for ArithmeticOperand {
    type Error = DecodeError;

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
            _ => Err(DecodeError::UnknownOperand { operand: value }),
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
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == ArithmeticOperand16::BC as u8 => Ok(ArithmeticOperand16::BC),
            x if x == ArithmeticOperand16::DE as u8 => Ok(ArithmeticOperand16::DE),
            x if x == ArithmeticOperand16::HL as u8 => Ok(ArithmeticOperand16::HL),
            x if x == ArithmeticOperand16::SP as u8 => Ok(ArithmeticOperand16::SP),
            _ => Err(DecodeError::UnknownOperand { operand: value }),
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

impl TryFrom<u8> for MemoryOperand16 {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == MemoryOperand16::BC as u8 => Ok(MemoryOperand16::BC),
            x if x == MemoryOperand16::DE as u8 => Ok(MemoryOperand16::DE),
            x if x == MemoryOperand16::HLI as u8 => Ok(MemoryOperand16::HLI),
            x if x == MemoryOperand16::HLD as u8 => Ok(MemoryOperand16::HLD),
            _ => Err(DecodeError::UnknownOperand { operand: value }),
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

impl TryFrom<u8> for StackOperand16 {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == StackOperand16::BC as u8 => Ok(StackOperand16::BC),
            x if x == StackOperand16::DE as u8 => Ok(StackOperand16::DE),
            x if x == StackOperand16::HL as u8 => Ok(StackOperand16::HL),
            x if x == StackOperand16::AF as u8 => Ok(StackOperand16::AF),
            _ => Err(DecodeError::UnknownOperand { operand: value }),
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

impl TryFrom<u8> for Condition {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Condition::NZ as u8 => Ok(Condition::NZ),
            x if x == Condition::Z as u8 => Ok(Condition::Z),
            x if x == Condition::NC as u8 => Ok(Condition::NC),
            x if x == Condition::C as u8 => Ok(Condition::C),
            _ => Err(DecodeError::UnknownOperand { operand: value }),
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    nop,

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
    add_a_n8 {
        immediate: u8,
    },
    adc_a_n8 {
        immediate: u8,
    },
    sub_a_n8 {
        immediate: u8,
    },
    sbc_a_n8 {
        immediate: u8,
    },
    and_a_n8 {
        immediate: u8,
    },
    xor_a_n8 {
        immediate: u8,
    },
    or_a_n8 {
        immediate: u8,
    },
    cp_a_n8 {
        operand: ArithmeticOperand,
        immediate: u8,
    },
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
        immediate: u8,
    },
    ld_r8_r8 {
        operand_a: ArithmeticOperand,
        operand_b: ArithmeticOperand,
    },

    // 16-bit load operations
    ld_r16_n16 {
        operand: ArithmeticOperand16,
        immediate: u16,
    },
    ld_ind_r16mem_a {
        operand: MemoryOperand16,
    },
    ld_a_ind_r16mem {
        operand: MemoryOperand16,
    },

    // jumps
    jr_n8 {
        immediate: u8,
    },
    jr_cond_n8 {
        condition: Condition,
        immediate: u8,
    },
    jp_n16 {
        immediate: u16,
    },
    jp_hl {
        immediate: u16,
    },
    jp_cond_n16 {
        condition: Condition,
        immediate: u16,
    },
    call_n16 {
        immediate: u16,
    },
    call_cond_n16 {
        condition: Condition,
        immediate: u16,
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

    // special
    halt,
    stop,

    // errors
    unknown_opcode {
        opcode: u8,
    },
    unknown_prefix_opcode {
        opcode: u8,
    },
}

macro_rules! arithmetic_operand_0_2 {
    ($opcode:ident) => {
        extract_bits!($opcode: u8, 0, 2).try_into().unwrap()
    };
}

macro_rules! arithmetic_operand_3_5 {
    ($opcode:ident) => {
        extract_bits!($opcode: u8, 3, 5).try_into().unwrap()
    };
}

macro_rules! arithmetic_operand_16_4_5 {
    ($opcode:ident) => {
        extract_bits!($opcode: u8, 4, 5).try_into().unwrap()
    };
}

impl Instruction {
    pub(super) fn decode_prefix_instruction(opcode: u8) -> Self {
        let mut result = Self::unknown_prefix_opcode { opcode };

        match extract_bits!(opcode: u8, 6, 7) {
            0b00 => match extract_bits!(opcode: u8, 3, 5) {
                0b000 => {
                    result = Instruction::rlc_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b001 => {
                    result = Instruction::rrc_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b010 => {
                    result = Instruction::rl_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b011 => {
                    result = Instruction::rr_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b100 => {
                    result = Instruction::sla_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b101 => {
                    result = Instruction::sra_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b110 => {
                    result = Instruction::swap_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                0b111 => {
                    result = Instruction::srl_r8 {
                        operand: arithmetic_operand_0_2!(opcode),
                    }
                },
                _ => {},
            },
            0b01 => {
                result = Instruction::bit_b3_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                    index: extract_bits!(opcode: u8, 3, 5),
                }
            },
            0b10 => {
                result = Instruction::res_b3_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                    index: extract_bits!(opcode: u8, 3, 5),
                }
            },
            0b11 => {
                result = Instruction::set_b3_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                    index: extract_bits!(opcode: u8, 3, 5),
                }
            },
            _ => {},
        }

        result
    }

    pub(super) fn decode_instruction(opcode: u8) -> Instruction {
        if opcode == 0x00 {
            return Instruction::nop;
        }

        if opcode == 0b11001011 {
            return Instruction::prefix;
        }

        match extract_bits!(opcode: u8, 6, 7) {
            0b00 => match extract_bits!(opcode: u8, 0, 3) {
                0b0011 => Instruction::inc_r16 {
                    operand: arithmetic_operand_16_4_5!(opcode),
                },
                0b1011 => Instruction::dec_r16 {
                    operand: arithmetic_operand_16_4_5!(opcode),
                },
                0b1001 => Instruction::add_hl_r16 {
                    operand: arithmetic_operand_16_4_5!(opcode),
                },
                _ => match extract_bits!(opcode: u8, 0, 2) {
                    0b100 => Instruction::inc_r8 {
                        operand: arithmetic_operand_3_5!(opcode),
                    },
                    0b101 => Instruction::dec_r8 {
                        operand: arithmetic_operand_3_5!(opcode),
                    },
                    0b111 => match extract_bits!(opcode: u8, 3, 5) {
                        0b000 => Instruction::rla,
                        0b001 => Instruction::rrca,
                        0b010 => Instruction::rla,
                        0b011 => Instruction::rra,
                        0b100 => Instruction::daa,
                        0b101 => Instruction::cpl,
                        0b110 => Instruction::scf,
                        0b111 => Instruction::ccf,
                        _ => Instruction::unknown_opcode { opcode },
                    },
                    _ => Instruction::unknown_opcode { opcode },
                },
            },
            0b01 => Instruction::unknown_opcode { opcode },
            0b10 => match extract_bits!(opcode: u8, 3, 5) {
                0b000 => Instruction::add_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b001 => Instruction::adc_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b010 => Instruction::sub_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b011 => Instruction::sbc_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b100 => Instruction::and_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b101 => Instruction::xor_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b110 => Instruction::or_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                0b111 => Instruction::cp_a_r8 {
                    operand: arithmetic_operand_0_2!(opcode),
                },
                _ => Self::unknown_opcode { opcode },
            },
            _ => Self::unknown_opcode { opcode },
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
    pub(super) fn instruction_step(&mut self, mmu: &mut Mmu) -> Result<bool, ExecutionError> {
        match self.current_instruction {
            Instruction::nop => Ok(true),

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

            _ => Err(ExecutionError::NoImpl {
                instruction: self.current_instruction,
            }),
        }
    }
}
