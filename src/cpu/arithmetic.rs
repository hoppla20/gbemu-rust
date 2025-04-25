use super::{Cpu, registers::Register, registers::Registers};

fn half_carry_8(a: u8, b: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F)) > 0x0F
}

fn alu_add_8(registers: &mut Registers, source: Register) {
    let (temp, overflow) = registers.a.overflowing_add(registers.get_register(source));
    registers.set_flag_zero(temp == 0);
    registers.set_flag_subtraction(false);
    registers.set_flag_half_carry(half_carry_8(registers.a, registers.get_register(source)));
    registers.set_flag_carry(overflow);
    registers.a = temp;
}

impl Cpu {
    pub(super) fn instr_add_a_r8(&mut self, source: Register) {
        match source {
            Register::F => {
                panic!("Instruction ADD A, F does not exist!")
            },
            _ => {
                alu_add_8(&mut self.registers, source);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_add_overflow_8() {
        // overflow

        let mut registers = Registers {
            a: 0xFF,
            b: 0x01,
            ..Registers::default()
        };
        alu_add_8(&mut registers, Register::B);
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
        alu_add_8(&mut registers, Register::B);
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
}
