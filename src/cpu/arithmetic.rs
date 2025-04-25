use super::{
    Cpu,
    registers::{DoubleRegister, Register, Registers},
};

fn half_carry_add_8(a: u8, b: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F)) > 0x0F
}

fn half_carry_add_8_3(a: u8, b: u8, c: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F) + (c & 0x0F)) > 0x0F
}

fn half_carry_sub_8(a: u8, b: u8) -> bool {
    ((a & 0x0F) as i8) - ((b & 0x0F) as i8) < 0
}

fn half_carry_sub_8_3(a: u8, b: u8, c: u8) -> bool {
    ((a & 0x0F) as i8) - ((b & 0x0F) as i8) - ((c & 0x0F) as i8) < 0
}

fn half_carry_add_16(a: u16, b: u16) -> bool {
    ((a & 0x00FF) + (b & 0x00FF)) > 0x00FF
}

impl Registers {
    // 8-bit arithmetics

    pub(super) fn alu_add(&mut self, source: Register) {
        let (temp, overflow) = self.a.overflowing_add(self.get_register(source));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_8(self.a, self.get_register(source)));
        self.set_flag_carry(overflow);
        self.a = temp;
    }

    pub(super) fn alu_adc(&mut self, source: Register) {
        let carry = if self.get_flag_carry() { 1 } else { 0 };
        let (temp, overflow) = self.a.overflowing_add(self.get_register(source));
        let (temp, overflow_c) = temp.overflowing_add(carry);
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_8_3(self.a, self.get_register(source), carry));
        self.set_flag_carry(overflow || overflow_c);
        self.a = temp;
    }

    pub(super) fn alu_sub(&mut self, source: Register) {
        let (temp, overflow) = self.a.overflowing_sub(self.get_register(source));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_8(self.a, self.get_register(source)));
        self.set_flag_carry(overflow);
        self.a = temp;
    }

    pub(super) fn alu_cp(&mut self, source: Register) {
        let (temp, overflow) = self.a.overflowing_sub(self.get_register(source));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_8(self.a, self.get_register(source)));
        self.set_flag_carry(overflow);
    }

    pub(super) fn alu_sbc(&mut self, source: Register) {
        let carry = if self.get_flag_carry() { 1 } else { 0 };
        let (temp, overflow) = self.a.overflowing_sub(self.get_register(source));
        let (temp, overflow_c) = temp.overflowing_sub(carry);
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_8_3(self.a, self.get_register(source), carry));
        self.set_flag_carry(overflow || overflow_c);
        self.a = temp;
    }

    pub(super) fn alu_and(&mut self, source: Register) {
        self.a &= self.get_register(source);
        self.set_flag_zero(self.a == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(true);
        self.set_flag_carry(false);
    }

    pub(super) fn alu_or(&mut self, source: Register) {
        self.a |= self.get_register(source);
        self.set_flag_zero(self.a == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(false);
    }

    pub(super) fn alu_xor(&mut self, source: Register) {
        self.a ^= self.get_register(source);
        self.set_flag_zero(self.a == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(false);
    }

    pub(super) fn alu_inc(&mut self, target: Register) {
        let (temp, _) = self.get_register(target).overflowing_add(1);
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_8(self.get_register(target), 1));
        self.set_register(target, temp);
    }

    pub(super) fn alu_dec(&mut self, target: Register) {
        let (temp, _) = self.get_register(target).overflowing_sub(1);
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(half_carry_sub_8(self.get_register(target), 1));
        self.set_register(target, temp);
    }

    pub(super) fn alu_swap(&mut self, target: Register) {
        self.set_register(
            target,
            (self.get_register(target) >> 4) | (self.get_register(target) << 4),
        );
        self.set_flag_zero(self.get_register(target) == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(false);
    }

    // 8-bit rotation

    pub(super) fn alu_rra(&mut self) {
        self.alu_rr(Register::A);
    }

    pub(super) fn alu_rrca(&mut self) {
        self.alu_rrc(Register::A);
    }

    pub(super) fn alu_srl(&mut self) {
        self.alu_sra(Register::A);
    }

    pub(super) fn alu_rla(&mut self) {
        self.alu_rl(Register::A);
    }

    pub(super) fn alu_rlca(&mut self) {
        self.alu_rlc(Register::A);
    }

    pub(super) fn alu_cpl(&mut self) {
        self.set_flag_subtraction(true);
        self.set_flag_half_carry(true);
        self.a = !self.a;
    }

    pub(super) fn alu_rr(&mut self, target: Register) {
        let temp =
            (if self.get_flag_carry() { 1 } else { 0 } << 7) | (self.get_register(target) >> 1);
        self.set_flag_zero(false);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_register(target) & 0x01 == 0x01);
        self.set_register(target, temp);
    }

    pub(super) fn alu_rrc(&mut self, target: Register) {
        self.set_flag_zero(false);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_register(target) & 0x01 == 0x01);
        self.set_register(target, self.get_register(target).rotate_right(1));
    }

    pub(super) fn alu_sra(&mut self, target: Register) {
        self.set_flag_zero(false);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_register(target) & 0x01 == 0x01);
        self.set_register(target, self.get_register(target) >> 1);
    }

    pub(super) fn alu_rl(&mut self, target: Register) {
        let temp = (if self.get_flag_carry() { 1 } else { 0 }) | (self.get_register(target) << 1);
        self.set_flag_zero(false);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_register(target) >> 7 == 0x01);
        self.set_register(target, temp);
    }

    pub(super) fn alu_rlc(&mut self, target: Register) {
        self.set_flag_zero(false);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_register(target) >> 7 == 0x01);
        self.set_register(target, self.get_register(target).rotate_left(1));
    }

    pub(super) fn alu_sla(&mut self, target: Register) {
        self.set_flag_zero(false);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(self.get_register(target) >> 7 == 0x01);
        self.set_register(target, self.get_register(target) << 1);
    }

    // 8-bit bit operations

    pub(super) fn alu_bit(&mut self, target: Register, index: u8) {
        if index >= 8 {
            panic!("Bit index has to between [0, 7]!");
        }

        self.set_flag_zero((self.get_register(target) >> index) & 0x01 == 0x01);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(true);
    }

    pub(super) fn alu_res(&mut self, target: Register, index: u8) {
        if index >= 8 {
            panic!("Bit index has to between [0, 7]!");
        }

        self.set_register(target, self.get_register(target) & !(1 << index));
    }

    pub(super) fn alu_set(&mut self, target: Register, index: u8) {
        if index >= 8 {
            panic!("Bit index has to between [0, 7]!");
        }

        self.set_register(target, self.get_register(target) | (1 << index));
    }

    // 16-bit arithmetics

    pub(super) fn alu_add_16(&mut self, source: DoubleRegister) {
        let (temp, overflow) = self
            .get_hl()
            .overflowing_add(self.get_double_register(source));
        self.set_flag_zero(temp == 0);
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(half_carry_add_16(
            self.get_hl(),
            self.get_double_register(source),
        ));
        self.set_flag_carry(overflow);
        self.set_hl(temp);
    }

    // carry flag

    pub(super) fn alu_ccf(&mut self) {
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(!self.get_flag_carry());
    }

    pub(super) fn alu_scf(&mut self) {
        self.set_flag_subtraction(false);
        self.set_flag_half_carry(false);
        self.set_flag_carry(true);
    }
}

impl Cpu {
    pub(super) fn instr_add_a_r8(&mut self, source: Register) {
        match source {
            Register::F => {
                panic!("Instruction ADD A, F does not exist!")
            },
            _ => {
                self.registers.alu_add(source);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_add_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x01,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_add(Register::B);
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
        registers.alu_add(Register::B);
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
        registers.alu_add(Register::B);
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
    fn test_alu_adc_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x00,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_adc(Register::B);
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
        registers.alu_adc(Register::B);
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
        registers.alu_adc(Register::B);
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
    fn test_alu_sub_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x01,
            b: 0x01,
            ..Registers::default()
        };
        registers.alu_sub(Register::B);
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
        registers.alu_sub(Register::B);
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
        registers.alu_sub(Register::B);
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
    fn test_alu_sbc_overflow() {
        // non-overflow

        let mut registers = Registers {
            a: 0x01,
            b: 0x00,
            ..Registers::default()
        };
        registers.set_flag_carry(true);
        registers.alu_sbc(Register::B);
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
        registers.alu_sbc(Register::B);
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
        registers.alu_sbc(Register::B);
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
    fn test_alu_add_overflow_16() {
        // non-overflow

        let mut registers = Registers::default();
        registers.set_hl(0x0001);
        registers.set_bc(0x0001);
        registers.alu_add_16(DoubleRegister::BC);
        assert_eq!(
            registers.get_hl(),
            0x0002,
            "Register A's value is incorrect!"
        );
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

        let mut registers = Registers::default();
        registers.set_hl(0xFFFF);
        registers.set_bc(0x0001);
        registers.alu_add_16(DoubleRegister::BC);
        assert_eq!(
            registers.get_hl(),
            0x0000,
            "Register A's value is incorrect!"
        );
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

        registers = Registers::default();
        registers.set_hl(0x00FF);
        registers.set_bc(0x0001);
        registers.alu_add_16(DoubleRegister::BC);
        assert_eq!(
            registers.get_hl(),
            0x0100,
            "Register A's value is incorrect!"
        );
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
        registers.alu_sra(Register::A);
        assert_eq!(
            registers.a, 0b00000001,
            "Register A's values is not correct!"
        );
        assert!(
            !registers.get_flag_carry(),
            "The carry flag should not be set!"
        );
        registers.alu_sra(Register::A);
        assert_eq!(registers.a, 0x00, "Register A's values is not correct!");
        assert!(registers.get_flag_carry(), "The carry flag should be set!");

        registers = Registers {
            a: 0b01000000,
            ..Registers::default()
        };
        registers.alu_sla(Register::A);
        assert_eq!(
            registers.a, 0b10000000,
            "Register A's values is not correct!"
        );
        assert!(
            !registers.get_flag_carry(),
            "The carry flag should not be set!"
        );
        registers.alu_sla(Register::A);
        assert_eq!(registers.a, 0x00, "Register A's values is not correct!");
        assert!(registers.get_flag_carry(), "The carry flag should be set!");
    }
}
