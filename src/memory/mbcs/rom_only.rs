use super::super::Memory;

use super::Mbc;

const ROM_SIZE_ALLOWED: usize = 0x8000;
const E_RAM_SIZE_ALLOWED: usize = 0x2000;

pub struct MbcRomOnly {
    pub memory: Memory,
}

impl Mbc for MbcRomOnly {
    fn new(_: usize, e_ram_size: usize, is_cgb: bool) -> Self {
        Self {
            memory: Memory::new(
                ROM_SIZE_ALLOWED,
                if e_ram_size != 0 {
                    E_RAM_SIZE_ALLOWED
                } else {
                    0
                },
                is_cgb,
            ),
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address < 0x8000 {
            return self.memory.rom[address as usize];
        }

        if address < 0xA000 {
            if self.memory.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            return self.memory.v_ram[(address - 0xA000) as usize];
        }

        unimplemented!("Memory read to address 0x{:x} is not implemented!", address);
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            // ROM is read-only, do nothing
            return;
        }

        if address < 0xA000 {
            if self.memory.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            self.memory.v_ram[(address - 0xA000) as usize] = value;
            return;
        }

        unimplemented!(
            "Memory write to address 0x{:x} is not implemented!",
            address
        );
    }
}
