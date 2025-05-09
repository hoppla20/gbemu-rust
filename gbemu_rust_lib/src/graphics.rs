pub mod object;
pub mod registers;
pub mod tile;

use crate::memory::V_RAM_BANK_SIZE;

use registers::GraphicsRegisters;

pub struct Ppu {
    pub v_ram: Vec<u8>,
    pub registers: GraphicsRegisters,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            v_ram: vec![0; V_RAM_BANK_SIZE],
            registers: GraphicsRegisters::new(),
        }
    }
}
