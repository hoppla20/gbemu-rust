use crate::system::System;
use crate::utils::bit_operations::bit;

use super::Cpu;

const JOYPAD_INTERRUPT_BIT: usize = 4;
const SERIAL_INTERRUPT_BIT: usize = 3;
const TIMER_INTERRUPT_BIT: usize = 2;
const LCD_INTERRUPT_BIT: usize = 1;
const V_BLANK_INTERRUPT_BIT: usize = 0;

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    VBlank,
    Lcd,
    Timer,
    Serial,
    Joypad,
}

impl From<Interrupt> for u16 {
    fn from(value: Interrupt) -> Self {
        match value {
            Interrupt::VBlank => 0x40,
            Interrupt::Lcd => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InterruptFlags {
    pub joypad: bool,
    pub serial: bool,
    pub timer: bool,
    pub lcd: bool,
    pub v_blank: bool,
}

impl From<u8> for InterruptFlags {
    fn from(value: u8) -> Self {
        Self {
            joypad: bit!(value: u8, 4),
            serial: bit!(value: u8, 3),
            timer: bit!(value: u8, 2),
            lcd: bit!(value: u8, 1),
            v_blank: bit!(value: u8, 0),
        }
    }
}

impl From<InterruptFlags> for u8 {
    fn from(value: InterruptFlags) -> Self {
        let mut result = 0;
        result |= if value.joypad {
            1 << JOYPAD_INTERRUPT_BIT
        } else {
            0
        };
        result |= if value.serial {
            1 << SERIAL_INTERRUPT_BIT
        } else {
            0
        };
        result |= if value.timer {
            1 << TIMER_INTERRUPT_BIT
        } else {
            0
        };
        result |= if value.lcd { 1 << LCD_INTERRUPT_BIT } else { 0 };
        result |= if value.v_blank {
            1 << V_BLANK_INTERRUPT_BIT
        } else {
            0
        };
        result
    }
}

impl Cpu {
    pub fn request_interrupt(&mut self, mmu: &mut System, interrupt: Interrupt) {
        match interrupt {
            Interrupt::Joypad => mmu.io.interrupt_flags.joypad = true,
            Interrupt::Serial => mmu.io.interrupt_flags.serial = true,
            Interrupt::Timer => mmu.io.interrupt_flags.timer = true,
            Interrupt::Lcd => mmu.io.interrupt_flags.lcd = true,
            Interrupt::VBlank => mmu.io.interrupt_flags.v_blank = true,
        }
    }

    pub fn interrupt_check(&mut self, mmu: &mut System) -> Option<Interrupt> {
        if !self.interrupt_enabled {
            return None;
        }

        if (mmu.io.interrupt_enable & (1 << V_BLANK_INTERRUPT_BIT)) > 0
            && mmu.io.interrupt_flags.v_blank
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags.v_blank = false;
            return Some(Interrupt::VBlank);
        }

        if (mmu.io.interrupt_enable & (1 << LCD_INTERRUPT_BIT)) > 0 && mmu.io.interrupt_flags.lcd {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags.lcd = false;
            return Some(Interrupt::Lcd);
        }

        if (mmu.io.interrupt_enable & (1 << TIMER_INTERRUPT_BIT)) > 0
            && mmu.io.interrupt_flags.timer
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags.timer = false;
            return Some(Interrupt::Timer);
        }

        if (mmu.io.interrupt_enable & (1 << SERIAL_INTERRUPT_BIT)) > 0
            && mmu.io.interrupt_flags.serial
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags.serial = false;
            return Some(Interrupt::Serial);
        }

        if (mmu.io.interrupt_enable & (1 << JOYPAD_INTERRUPT_BIT)) > 0
            && mmu.io.interrupt_flags.joypad
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags.joypad = false;
            return Some(Interrupt::Joypad);
        }

        None
    }
}
