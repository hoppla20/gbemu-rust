use crate::memory::mmu::Mmu;

use super::Cpu;

const VBLANK_INTERRUPT_BIT: usize = 0;
const LCD_INTERRUPT_BIT: usize = 1;
const TIMER_INTERRUPT_BIT: usize = 2;
const SERIAL_INTERRUPT_BIT: usize = 3;
const JOYPAD_INTERRUPT_BIT: usize = 4;

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

impl Cpu {
    pub fn interrupt_check(&mut self, mmu: &mut Mmu) -> Option<Interrupt> {
        if (mmu.io.interrupt_enable & (1 << VBLANK_INTERRUPT_BIT)) > 0
            && (mmu.io.interrupt_flags & (1 << VBLANK_INTERRUPT_BIT)) > 0
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags &= !(1 << VBLANK_INTERRUPT_BIT);
            return Some(Interrupt::VBlank);
        }

        if (mmu.io.interrupt_enable & (1 << LCD_INTERRUPT_BIT)) > 0
            && (mmu.io.interrupt_flags & (1 << LCD_INTERRUPT_BIT)) > 0
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags &= !(1 << LCD_INTERRUPT_BIT);
            return Some(Interrupt::Lcd);
        }

        if (mmu.io.interrupt_enable & (1 << TIMER_INTERRUPT_BIT)) > 0
            && (mmu.io.interrupt_flags & (1 << TIMER_INTERRUPT_BIT)) > 0
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags &= !(1 << TIMER_INTERRUPT_BIT);
            return Some(Interrupt::Timer);
        }

        if (mmu.io.interrupt_enable & (1 << SERIAL_INTERRUPT_BIT)) > 0
            && (mmu.io.interrupt_flags & (1 << SERIAL_INTERRUPT_BIT)) > 0
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags &= !(1 << SERIAL_INTERRUPT_BIT);
            return Some(Interrupt::Serial);
        }

        if (mmu.io.interrupt_enable & (1 << JOYPAD_INTERRUPT_BIT)) > 0
            && (mmu.io.interrupt_flags & (1 << JOYPAD_INTERRUPT_BIT)) > 0
        {
            self.interrupt_enabled = false;
            mmu.io.interrupt_flags &= !(1 << JOYPAD_INTERRUPT_BIT);
            return Some(Interrupt::Joypad);
        }

        None
    }
}
