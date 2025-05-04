use super::instructions::{Condition, MemoryOperand16, StackOperand16};

const FLAG_ZERO_BYTE_POS: u8 = 7;
const FLAG_SUBTRACTION_BYTE_POS: u8 = 6;
const FLAG_HALF_CARRY_BYTE_POS: u8 = 5;
const FLAG_CARRY_BYTE_POS: u8 = 4;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    W,
    Z,
}

#[derive(Clone, Copy)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Default, Debug)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub w: u8,
    pub z: u8,
    pub sp: u16,
    pub pc: u16,
    pub cc: bool,
}

impl Registers {
    pub fn get_register(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::F => self.f,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::W => self.w,
            Register::Z => self.z,
        }
    }
    pub fn set_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::F => self.f = value & 0xF0,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::W => {
                panic!("Writing into register W is not allowed with this method!");
            },
            Register::Z => {
                panic!("Writing into register Z is not allowed with this method!");
            },
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        // lower 4 bits of f flag can't be set
        self.f = (value & 0xF0) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn get_wz(&self) -> u16 {
        (self.w as u16) << 8 | self.z as u16
    }
    pub fn set_wz(&mut self, value: u16) {
        self.w = (value >> 8) as u8;
        self.z = (value & 0xFF) as u8;
    }

    pub fn get_double_register(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => self.get_af(),
            Register16::BC => self.get_bc(),
            Register16::DE => self.get_de(),
            Register16::HL => self.get_hl(),
        }
    }
    pub fn set_double_register(&mut self, register: Register16, value: u16) {
        match register {
            Register16::AF => self.set_af(value),
            Register16::BC => self.set_bc(value),
            Register16::DE => self.set_de(value),
            Register16::HL => self.set_hl(value),
        }
    }

    pub fn get_flag_zero(&self) -> bool {
        (self.f >> FLAG_ZERO_BYTE_POS) & 1 != 0
    }
    pub fn set_flag_zero(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_ZERO_BYTE_POS;
        } else {
            self.f &= !(1 << FLAG_ZERO_BYTE_POS);
        }
    }
    pub fn get_flag_subtraction(&self) -> bool {
        (self.f >> FLAG_SUBTRACTION_BYTE_POS) & 1 != 0
    }
    pub fn set_flag_subtraction(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_SUBTRACTION_BYTE_POS;
        } else {
            self.f &= !(1 << FLAG_SUBTRACTION_BYTE_POS);
        }
    }
    pub fn get_flag_half_carry(&self) -> bool {
        (self.f >> FLAG_HALF_CARRY_BYTE_POS) & 1 != 0
    }
    pub fn set_flag_half_carry(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_HALF_CARRY_BYTE_POS;
        } else {
            self.f &= !(1 << FLAG_HALF_CARRY_BYTE_POS);
        }
    }
    pub fn get_flag_carry(&self) -> bool {
        (self.f >> FLAG_CARRY_BYTE_POS) & 1 != 0
    }
    pub fn set_flag_carry(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_CARRY_BYTE_POS;
        } else {
            self.f &= !(1 << FLAG_CARRY_BYTE_POS);
        }
    }

    pub fn check_condition(&mut self, condition: Condition) {
        match condition {
            Condition::NZ => self.cc = !self.get_flag_zero(),
            Condition::Z => self.cc = self.get_flag_zero(),
            Condition::NC => self.cc = !self.get_flag_carry(),
            Condition::C => self.cc = self.get_flag_carry(),
        }
    }

    pub fn get_memory_operand(&mut self, operand: MemoryOperand16) -> u16 {
        match operand {
            MemoryOperand16::BC => self.get_bc(),
            MemoryOperand16::DE => self.get_de(),
            MemoryOperand16::HLI => {
                let result = self.get_hl();
                self.set_hl(self.get_hl().wrapping_add(1));
                result
            },
            MemoryOperand16::HLD => {
                let result = self.get_hl();
                self.set_hl(self.get_hl().wrapping_sub(1));
                result
            },
        }
    }

    pub fn get_stack_operand(&self, operand: StackOperand16) -> u16 {
        match operand {
            StackOperand16::BC => self.get_bc(),
            StackOperand16::DE => self.get_de(),
            StackOperand16::HL => self.get_hl(),
            StackOperand16::AF => self.get_af(),
        }
    }

    pub fn set_stack_operand(&mut self, operand: StackOperand16, value: u16) {
        match operand {
            StackOperand16::BC => self.set_bc(value),
            StackOperand16::DE => self.set_de(value),
            StackOperand16::HL => self.set_hl(value),
            StackOperand16::AF => self.set_af(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_registers() {
        let mut regs = Registers::default();

        // Register AF

        regs.set_double_register(Register16::AF, 1);
        assert_eq!(regs.a, 0, "Single register value not correct!");
        assert_eq!(regs.f, 0, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(Register16::AF),
            0,
            "Read value is different than written value!"
        );

        // Register BC

        regs.set_double_register(Register16::BC, 1 << 1);
        assert_eq!(regs.b, 0, "Single register value not correct!");
        assert_eq!(regs.c, 1 << 1, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(Register16::BC),
            2,
            "Read value is different than written value!"
        );

        // Register DE

        regs.set_double_register(Register16::DE, 1 << 14);
        assert_eq!(regs.d, 1 << 6, "Single register value not correct!");
        assert_eq!(regs.e, 0, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(Register16::DE),
            1 << 14,
            "Read value is different than written value!"
        );

        // Register HL

        regs.set_double_register(Register16::HL, 1 << 15);
        assert_eq!(regs.h, 1 << 7, "Single register value not correct!");
        assert_eq!(regs.l, 0, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(Register16::HL),
            1 << 15,
            "Read value is different than written value!"
        );
    }

    #[test]
    fn test_flags() {
        // Flag zero

        let mut regs = Registers {
            f: 1 << FLAG_ZERO_BYTE_POS,
            ..Registers::default()
        };
        assert!(regs.get_flag_zero(), "Zero flag not correct!");
        assert!(
            !regs.get_flag_subtraction(),
            "Subtraction flag not correct!"
        );
        assert!(!regs.get_flag_half_carry(), "Half carry flag not correct!");
        assert!(!regs.get_flag_carry(), "Carry flag not correct!");

        // Flag subtraction

        regs = Registers {
            f: 1 << FLAG_SUBTRACTION_BYTE_POS,
            ..Registers::default()
        };
        assert!(!regs.get_flag_zero(), "Zero flag not correct!");
        assert!(regs.get_flag_subtraction(), "Subtraction flag not correct!");
        assert!(!regs.get_flag_half_carry(), "Half carry flag not correct!");
        assert!(!regs.get_flag_carry(), "Carry flag not correct!");

        // Flag half carry

        regs = Registers {
            f: 1 << FLAG_HALF_CARRY_BYTE_POS,
            ..Registers::default()
        };
        assert!(!regs.get_flag_zero(), "Zero flag not correct!");
        assert!(
            !regs.get_flag_subtraction(),
            "Subtraction flag not correct!"
        );
        assert!(regs.get_flag_half_carry(), "Half carry flag not correct!");
        assert!(!regs.get_flag_carry(), "Carry flag not correct!");

        // Flag carry

        regs = Registers {
            f: 1 << FLAG_CARRY_BYTE_POS,
            ..Registers::default()
        };
        assert!(!regs.get_flag_zero(), "Zero flag not correct!");
        assert!(
            !regs.get_flag_subtraction(),
            "Subtraction flag not correct!"
        );
        assert!(!regs.get_flag_half_carry(), "Half carry flag not correct!");
        assert!(regs.get_flag_carry(), "Carry flag not correct!");
    }
}
