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
}

#[derive(Clone, Copy)]
pub enum DoubleRegister {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Default, Debug)]
pub struct Registers {
    pub(super) a: u8,
    pub(super) f: u8,
    pub(super) b: u8,
    pub(super) c: u8,
    pub(super) d: u8,
    pub(super) e: u8,
    pub(super) h: u8,
    pub(super) l: u8,
}

impl Registers {
    pub(super) fn get_register(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::F => self.f,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }
    pub(super) fn set_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::F => self.f = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    pub(super) fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }
    pub(super) fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    pub(super) fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub(super) fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub(super) fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub(super) fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub(super) fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub(super) fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub(super) fn get_double_register(&self, register: DoubleRegister) -> u16 {
        match register {
            DoubleRegister::AF => self.get_af(),
            DoubleRegister::BC => self.get_bc(),
            DoubleRegister::DE => self.get_de(),
            DoubleRegister::HL => self.get_hl(),
        }
    }
    pub(super) fn set_double_register(&mut self, register: DoubleRegister, value: u16) {
        match register {
            DoubleRegister::AF => self.set_af(value),
            DoubleRegister::BC => self.set_bc(value),
            DoubleRegister::DE => self.set_de(value),
            DoubleRegister::HL => self.set_hl(value),
        }
    }

    pub(super) fn get_flag_zero(&self) -> bool {
        (self.f >> FLAG_ZERO_BYTE_POS) & 1 != 0
    }
    pub(super) fn set_flag_zero(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_ZERO_BYTE_POS;
        }
    }
    pub(super) fn get_flag_subtraction(&self) -> bool {
        (self.f >> FLAG_SUBTRACTION_BYTE_POS) & 1 != 0
    }
    pub(super) fn set_flag_subtraction(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_SUBTRACTION_BYTE_POS;
        }
    }
    pub(super) fn get_flag_half_carry(&self) -> bool {
        (self.f >> FLAG_HALF_CARRY_BYTE_POS) & 1 != 0
    }
    pub(super) fn set_flag_half_carry(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_HALF_CARRY_BYTE_POS;
        }
    }
    pub(super) fn get_flag_carry(&self) -> bool {
        (self.f >> FLAG_CARRY_BYTE_POS) & 1 != 0
    }
    pub(super) fn set_flag_carry(&mut self, on: bool) {
        if on {
            self.f |= 1 << FLAG_CARRY_BYTE_POS;
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

        regs.set_double_register(DoubleRegister::AF, 1);
        assert_eq!(regs.a, 0, "Single register value not correct!");
        assert_eq!(regs.f, 1, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(DoubleRegister::AF),
            1,
            "Read value is different than written value!"
        );

        // Register BC

        regs.set_double_register(DoubleRegister::BC, 1 << 1);
        assert_eq!(regs.b, 0, "Single register value not correct!");
        assert_eq!(regs.c, 1 << 1, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(DoubleRegister::BC),
            2,
            "Read value is different than written value!"
        );

        // Register DE

        regs.set_double_register(DoubleRegister::DE, 1 << 14);
        assert_eq!(regs.d, 1 << 6, "Single register value not correct!");
        assert_eq!(regs.e, 0, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(DoubleRegister::DE),
            1 << 14,
            "Read value is different than written value!"
        );

        // Register HL

        regs.set_double_register(DoubleRegister::HL, 1 << 15);
        assert_eq!(regs.h, 1 << 7, "Single register value not correct!");
        assert_eq!(regs.l, 0, "Single register value not correct!");
        assert_eq!(
            regs.get_double_register(DoubleRegister::HL),
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
