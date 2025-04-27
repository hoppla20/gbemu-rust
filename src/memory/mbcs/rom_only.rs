use crate::memory::{
    E_RAM_BANK_ADDR, ECHO_RAM_ADDR, H_RAM_ADDR, IE_REGISTER_ADDR, IO_REGISTERS_ADDR, OAM_ADDR,
    UNUSABLE_ADDR, V_RAM_ADDR, W_RAM_BANK_0_ADDR,
};

use super::super::Memory;

use super::Mbc;

const ROM_SIZE_ALLOWED: usize = 0x8000;
const E_RAM_SIZE_ALLOWED: usize = 0x2000;

pub struct MbcRomOnly {
    pub memory: Memory,

    oam_transfer: bool,
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
            oam_transfer: false,
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address < V_RAM_ADDR {
            // rom

            return self.memory.rom[address as usize];
        }

        if address < E_RAM_BANK_ADDR {
            // vram

            if self.memory.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            return self.memory.v_ram[(address - V_RAM_ADDR) as usize];
        }

        if address < W_RAM_BANK_0_ADDR {
            // eram

            if self.memory.e_ram.is_empty() {
                return 0xFF;
            }

            return self.memory.e_ram[(address - E_RAM_BANK_ADDR) as usize];
        }

        if address < ECHO_RAM_ADDR {
            // wram

            if self.memory.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            return self.memory.w_ram[(address - W_RAM_BANK_0_ADDR) as usize];
        }

        if address < OAM_ADDR {
            // echo ram

            return self.read_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR);
        }

        if address < UNUSABLE_ADDR {
            // oam

            return self.memory.oam[(address - OAM_ADDR) as usize];
        }

        if address < IO_REGISTERS_ADDR {
            // unusable

            if self.oam_transfer {
                return 0xFF;
            } else {
                return 0x00;
            }
        }

        if address < H_RAM_ADDR {
            // io registers

            todo!("Implement i/o registers")
        }

        if address < IE_REGISTER_ADDR {
            // hram

            return self.memory.h_ram[(address - H_RAM_ADDR) as usize];
        }

        if address == IE_REGISTER_ADDR {
            // interrupt enable register

            todo!("Implement IE register")
        }

        unimplemented!("Memory read to address 0x{:x} is not implemented!", address);
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address < V_RAM_ADDR {
            // rom is read-only, do nothing

            return;
        }

        if address < E_RAM_BANK_ADDR {
            // vram

            if self.memory.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            self.memory.v_ram[(address - V_RAM_ADDR) as usize] = value;
            return;
        }

        if address < W_RAM_BANK_0_ADDR {
            // eram

            if self.memory.e_ram.is_empty() {
                return;
            }

            self.memory.e_ram[(address - E_RAM_BANK_ADDR) as usize] = value;
            return;
        }

        if address < ECHO_RAM_ADDR {
            // wram

            if self.memory.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            self.memory.w_ram[(address - W_RAM_BANK_0_ADDR) as usize] = value;
            return;
        }

        if address < OAM_ADDR {
            // echo ram

            self.write_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR, value);
            return;
        }

        if address < UNUSABLE_ADDR {
            // oam

            self.memory.oam[(address - OAM_ADDR) as usize] = value;
            return;
        }

        if address < IO_REGISTERS_ADDR {
            // unusable

            return;
        }

        if address < H_RAM_ADDR {
            // io registers

            todo!("Implement i/o registers")
        }

        if address < IE_REGISTER_ADDR {
            // hram

            self.memory.h_ram[(address - H_RAM_ADDR) as usize] = value;
            return;
        }

        if address == IE_REGISTER_ADDR {
            // interrupt enable register

            todo!("Implement IE register")
        }

        unimplemented!(
            "Memory write to address 0x{:x} is not implemented!",
            address
        );
    }
}
