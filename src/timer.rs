#[derive(Default)]
pub struct TimerRegisters {
    pub divider: u8,
    pub counter: u8,
    pub modulo: u8,
    pub control: u8,
}

impl TimerRegisters {
    fn new_dmg() -> Self {
        Self {
            divider: 0xAB,
            counter: 0x00,
            modulo: 0x00,
            control: 0xF8,
        }
    }
}
