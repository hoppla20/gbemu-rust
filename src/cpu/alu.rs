use crate::utils::half_carry::{
    half_carry_add_r8, half_carry_add_r8_3, half_carry_sub_r8, half_carry_sub_r8_3,
};

use super::{
    instructions::ArithmeticOperand, instructions::ArithmeticOperand16, registers::Registers,
};

impl Registers {
    // arithmetics

    pub fn get_arithmetic_target_r8(&self, operand: ArithmeticOperand) -> u8 {
        match operand {
            ArithmeticOperand::B => self.b,
            ArithmeticOperand::C => self.c,
            ArithmeticOperand::D => self.d,
            ArithmeticOperand::E => self.e,
            ArithmeticOperand::H => self.h,
            ArithmeticOperand::L => self.l,
            ArithmeticOperand::IND_HL => self.z,
            ArithmeticOperand::A => self.a,
        }
    }

    pub fn set_arithmetic_target_r8(&mut self, operand: ArithmeticOperand, value: u8) {
        match operand {
            ArithmeticOperand::B => self.b = value,
            ArithmeticOperand::C => self.c = value,
            ArithmeticOperand::D => self.d = value,
            ArithmeticOperand::E => self.e = value,
            ArithmeticOperand::H => self.h = value,
            ArithmeticOperand::L => self.l = value,
            ArithmeticOperand::IND_HL => self.z = value,
            ArithmeticOperand::A => self.a = value,
        }
    }

    pub fn get_arithmetic_target_r16(&self, operand: ArithmeticOperand16) -> u16 {
        match operand {
            ArithmeticOperand16::BC => self.get_bc(),
            ArithmeticOperand16::DE => self.get_de(),
            ArithmeticOperand16::HL => self.get_hl(),
            ArithmeticOperand16::SP => self.sp,
        }
    }

    pub fn set_arithmetic_target_r16(&mut self, operand: ArithmeticOperand16, value: u16) {
        match operand {
            ArithmeticOperand16::BC => self.set_bc(value),
            ArithmeticOperand16::DE => self.set_de(value),
            ArithmeticOperand16::HL => self.set_hl(value),
            ArithmeticOperand16::SP => self.sp = value,
        }
    }

    pub fn alu_add_a_r8(&mut self, operand: ArithmeticOperand) {
        let (temp, overflow) = self
            .a
            .overflowing_add(self.get_arithmetic_target_r8(operand));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_r8(
            self.a,
            self.get_arithmetic_target_r8(operand),
        ));
        self.set_flag_carry(overflow);
        self.a = temp;
    }

    pub fn alu_adc_a_r8(&mut self, operand: ArithmeticOperand) {
        let carry = if self.get_flag_carry() { 1 } else { 0 };
        let (temp, overflow) = self
            .a
            .overflowing_add(self.get_arithmetic_target_r8(operand));
        let (temp, overflow_c) = temp.overflowing_add(carry);
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_r8_3(
            self.a,
            self.get_arithmetic_target_r8(operand),
            carry,
        ));
        self.set_flag_carry(overflow || overflow_c);
        self.a = temp;
    }

    pub fn alu_sub_a_r8(&mut self, operand: ArithmeticOperand) {
        let (temp, overflow) = self
            .a
            .overflowing_sub(self.get_arithmetic_target_r8(operand));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_r8(
            self.a,
            self.get_arithmetic_target_r8(operand),
        ));
        self.set_flag_carry(overflow);
        self.a = temp;
    }

    pub fn alu_cp_a_r8(&mut self, operand: ArithmeticOperand) {
        let (temp, overflow) = self
            .a
            .overflowing_sub(self.get_arithmetic_target_r8(operand));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_r8(
            self.a,
            self.get_arithmetic_target_r8(operand),
        ));
        self.set_flag_carry(overflow);
    }

    pub fn alu_sbc_a_r8(&mut self, operand: ArithmeticOperand) {
        let carry = if self.get_flag_carry() { 1 } else { 0 };
        let (temp, overflow) = self
            .a
            .overflowing_sub(self.get_arithmetic_target_r8(operand));
        let (temp, overflow_c) = temp.overflowing_sub(carry);
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_r8_3(
            self.a,
            self.get_arithmetic_target_r8(operand),
            carry,
        ));
        self.set_flag_carry(overflow || overflow_c);
        self.a = temp;
    }

    pub fn alu_and_a_r8(&mut self, operand: ArithmeticOperand) {
        self.a &= self.get_arithmetic_target_r8(operand);
        self.set_flag_zero(self.a == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(true);
        self.set_flag_carry(false);
    }

    pub fn alu_or_a_r8(&mut self, operand: ArithmeticOperand) {
        self.a |= self.get_arithmetic_target_r8(operand);
        self.set_flag_zero(self.a == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(false);
    }

    pub fn alu_xor_a_r8(&mut self, operand: ArithmeticOperand) {
        self.a ^= self.get_arithmetic_target_r8(operand);
        self.set_flag_zero(self.a == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(false);
    }

    pub fn alu_inc_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let (result, _) = self.get_arithmetic_target_r8(operand).overflowing_add(1);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_r8(self.get_arithmetic_target_r8(operand), 1));

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_dec_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let (result, _) = self.get_arithmetic_target_r8(operand).overflowing_sub(1);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_r8(self.get_arithmetic_target_r8(operand), 1));

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_swap_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = (self.get_arithmetic_target_r8(operand) >> 4)
            | (self.get_arithmetic_target_r8(operand) << 4);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(false);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_cpl_a(&mut self) {
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(true);
        self.a = !self.a;
    }

    pub fn alu_daa(&mut self) {
        let mut overflow_1 = false;
        let mut overflow_2 = false;
        let mut temp = self.a as u16;
        if self.get_flag_subtraction() {
            if self.get_flag_half_carry() {
                temp = temp.wrapping_sub(0x06);
            }
            if self.get_flag_carry() {
                (temp, overflow_2) = temp.overflowing_sub(0x60);
            }
        } else {
            if self.get_flag_half_carry() || (temp & 0x0F) > 0x09 {
                temp += 0x06;
                overflow_1 = temp >= 0x100;
            }
            if self.get_flag_carry() || temp > 0x9F {
                temp += 0x60;
                overflow_2 = temp >= 0x100;
            }
        }
        self.set_flag_zero(temp as u8 == 0x00);
        self.set_flag_half_carry(false);
        if overflow_1 || overflow_2 {
            self.set_flag_carry(true);
        }
        self.a = temp as u8;
    }

    // rotation

    pub fn alu_rra(&mut self) {
        self.alu_rr_r8(ArithmeticOperand::A);
        self.set_flag_zero(false);
    }

    pub fn alu_rrca(&mut self) {
        self.alu_rrc_r8(ArithmeticOperand::A);
        self.set_flag_zero(false);
    }

    pub fn alu_srl_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = self.get_arithmetic_target_r8(operand) >> 1;
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) & 0x01 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_rla(&mut self) {
        self.alu_rl_r8(ArithmeticOperand::A);
        self.set_flag_zero(false);
    }

    pub fn alu_rlca(&mut self) {
        self.alu_rlc_r8(ArithmeticOperand::A);
        self.set_flag_zero(false);
    }

    pub fn alu_rr_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = (if self.get_flag_carry() { 1 } else { 0 } << 7)
            | (self.get_arithmetic_target_r8(operand) >> 1);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) & 0x01 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_rrc_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = self.get_arithmetic_target_r8(operand).rotate_right(1);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) & 0x01 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_sra_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = (self.get_arithmetic_target_r8(operand) >> 1)
            | (self.get_arithmetic_target_r8(operand) & 0b10000000);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) & 0x01 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_rl_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = (if self.get_flag_carry() { 1 } else { 0 })
            | (self.get_arithmetic_target_r8(operand) << 1);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) >> 7 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_rlc_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = self.get_arithmetic_target_r8(operand).rotate_left(1);
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) >> 7 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_sla_r8(&mut self, operand: ArithmeticOperand) -> u8 {
        let result = self.get_arithmetic_target_r8(operand) << 1;
        self.set_flag_zero(result == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_arithmetic_target_r8(operand) >> 7 == 0x01);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    // bit operations

    pub fn alu_bit_b3_r8(&mut self, index: u8, operand: ArithmeticOperand) {
        if index >= 8 {
            panic!("Bit index has to between [0, 7]!");
        }

        self.set_flag_zero((self.get_arithmetic_target_r8(operand) >> index) & 0x01 == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(true);
    }

    pub fn alu_res_b3_r8(&mut self, index: u8, operand: ArithmeticOperand) -> u8 {
        if index >= 8 {
            panic!("Bit index has to between [0, 7]!");
        }

        let result = self.get_arithmetic_target_r8(operand) & !(1 << index);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    pub fn alu_set_b3_r8(&mut self, index: u8, operand: ArithmeticOperand) -> u8 {
        if index >= 8 {
            panic!("Bit index has to between [0, 7]!");
        }

        let result = self.get_arithmetic_target_r8(operand) | (1 << index);

        match operand {
            ArithmeticOperand::IND_HL => {},
            _ => self.set_arithmetic_target_r8(operand, result),
        }

        result
    }

    // carry flag

    pub fn alu_ccf(&mut self) {
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(!self.get_flag_carry());
    }

    pub fn alu_scf(&mut self) {
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_add_a_r8_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x01,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_add_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x02, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            !registers.get_flag_subtraction(),
            "Subtraction flag should not be set!"
        );
        assert!(
            !registers.get_flag_half_carry(),
            "Half carry flag should not be set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");

        // overflow

        let mut registers = Registers {
            a: 0xFF,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_add_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x00, "Register A's value is incorrect!");
        assert!(registers.get_flag_zero(), "Zero flag is not set!");
        assert!(
            !registers.get_flag_subtraction(),
            "Subtraction flag should not be set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(registers.get_flag_carry(), "Carry flag is not set!");

        // half carry

        registers = Registers {
            a: 0x0F,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_add_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x10, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            !registers.get_flag_subtraction(),
            "Subtraction flag should not be set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");
    }

    #[test]
    fn test_alu_adc_a_r8_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x00,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_adc_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x01, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            !registers.get_flag_subtraction(),
            "Subtraction flag should not be set!"
        );
        assert!(
            !registers.get_flag_half_carry(),
            "Half carry flag should not be set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");

        // overflow

        let mut registers = Registers {
            a: 0xFF,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_adc_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x00, "Register A's value is incorrect!");
        assert!(registers.get_flag_zero(), "Zero flag is not set!");
        assert!(
            !registers.get_flag_subtraction(),
            "Subtraction flag should not be set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(registers.get_flag_carry(), "Carry flag is not set!");

        // half carry

        registers = Registers {
            a: 0x0F,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_adc_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x10, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            !registers.get_flag_subtraction(),
            "Subtraction flag should not be set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");
    }

    #[test]
    fn test_alu_sub_a_r8_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x01,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_sub_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x00, "Register A's value is incorrect!");
        assert!(registers.get_flag_zero(), "Zero flag is not set!");
        assert!(
            registers.get_flag_subtraction(),
            "Subtraction flag is not set!"
        );
        assert!(
            !registers.get_flag_half_carry(),
            "Half carry flag should not be set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");

        // overflow

        let mut registers = Registers {
            a: 0x00,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_sub_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0xFF, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            registers.get_flag_subtraction(),
            "Subtraction flag is not set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(registers.get_flag_carry(), "Carry flag is not set!");

        // half carry

        registers = Registers {
            a: 0x10,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_sub_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x0F, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            registers.get_flag_subtraction(),
            "Subtraction flag is not set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");
    }

    #[test]
    fn test_alu_sbc_a_r8_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x01,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_sbc_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x00, "Register A's value is incorrect!");
        assert!(registers.get_flag_zero(), "Zero flag is not set!");
        assert!(
            registers.get_flag_subtraction(),
            "Subtraction flag is not set!"
        );
        assert!(
            !registers.get_flag_half_carry(),
            "Half carry flag should not be set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");

        // overflow

        let mut registers = Registers {
            a: 0x00,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_sbc_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0xFF, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            registers.get_flag_subtraction(),
            "Subtraction flag is not set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(registers.get_flag_carry(), "Carry flag is not set!");

        // half carry

        registers = Registers {
            a: 0x10,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_sbc_a_r8(ArithmeticOperand::B);
        assert_eq!(registers.a, 0x0F, "Register A's value is incorrect!");
        assert!(!registers.get_flag_zero(), "Zero flag should not be set!");
        assert!(
            registers.get_flag_subtraction(),
            "Subtraction flag is not set!"
        );
        assert!(
            registers.get_flag_half_carry(),
            "Half carry flag is not set!"
        );
        assert!(!registers.get_flag_carry(), "Carry flag should not be set!");
    }

    #[test]
    fn test_rotate_a() {
        // rotate through c

        let mut registers = Registers {
            a: 0xFF,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_rra();
        assert_eq!(
            registers.a, 0xFF,
            "Register A's values should still be 0xFF!"
        );
        assert!(
            registers.get_flag_carry(),
            "The carry flag should still be set!"
        );
        registers.alu_rla();
        assert_eq!(
            registers.a, 0xFF,
            "Register A's values should still be 0xFF!"
        );
        assert!(
            registers.get_flag_carry(),
            "The carry flag should still be set!"
        );

        registers = Registers {
            a: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_rra();
        assert_eq!(
            registers.a, 0b10000000,
            "Register A's values is not correct!"
        );
        assert!(!registers.get_flag_carry(), "The carry flag is still set!");
        registers.alu_rla();
        assert_eq!(registers.a, 0x00, "Register A's values is not correct!");
        assert!(registers.get_flag_carry(), "The carry flag should be set!");
        registers.alu_rla();
        assert_eq!(
            registers.a, 0b00000001,
            "Register A's values is not correct!"
        );
        assert!(!registers.get_flag_carry(), "The carry flag is still set!");
        registers.alu_rra();
        assert_eq!(registers.a, 0x00, "Register A's values is not correct!");
        assert!(registers.get_flag_carry(), "The carry flag should be set!");

        // rotate with c

        registers = Registers {
            a: 0b00000010,
            ..Registers::default()
        };
        registers.alu_rrca();
        assert_eq!(
            registers.a, 0b00000001,
            "Register A's values is not correct!"
        );
        assert!(
            !registers.get_flag_carry(),
            "The carry flag should not be set!"
        );
        registers.alu_rrca();
        assert_eq!(
            registers.a, 0b10000000,
            "Register A's values is not correct!"
        );
        assert!(registers.get_flag_carry(), "The carry flag should be set!");
        registers.alu_rlca();
        assert_eq!(
            registers.a, 0b00000001,
            "Register A's values is not correct!"
        );
        assert!(registers.get_flag_carry(), "The carry flag should be set!");
        registers.alu_rlca();
        assert_eq!(
            registers.a, 0b00000010,
            "Register A's values is not correct!"
        );
        assert!(
            !registers.get_flag_carry(),
            "The carry flag should not be set!"
        );

        // arithmetic shifts

        registers = Registers {
            a: 0b00000010,
            ..Registers::default()
        };
        registers.alu_sra_r8(ArithmeticOperand::A);
        assert_eq!(
            registers.a, 0b00000001,
            "Register A's values is not correct!"
        );
        assert!(
            !registers.get_flag_carry(),
            "The carry flag should not be set!"
        );
        registers.alu_sra_r8(ArithmeticOperand::A);
        assert_eq!(registers.a, 0x00, "Register A's values is not correct!");
        assert!(registers.get_flag_carry(), "The carry flag should be set!");

        registers = Registers {
            a: 0b01000000,
            ..Registers::default()
        };
        registers.alu_sla_r8(ArithmeticOperand::A);
        assert_eq!(
            registers.a, 0b10000000,
            "Register A's values is not correct!"
        );
        assert!(
            !registers.get_flag_carry(),
            "The carry flag should not be set!"
        );
        registers.alu_sla_r8(ArithmeticOperand::A);
        assert_eq!(registers.a, 0x00, "Register A's values is not correct!");
        assert!(registers.get_flag_carry(), "The carry flag should be set!");
    }
}
