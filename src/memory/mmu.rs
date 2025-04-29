use super::mbc::Mbc;

use super::{
    E_RAM_BANK_ADDR, ECHO_RAM_ADDR, H_RAM_ADDR, H_RAM_SIZE, IE_REGISTER_ADDR, IO_REGISTERS_ADDR,
    OAM_ADDR, UNUSABLE_ADDR, V_RAM_ADDR, W_RAM_BANK_0_ADDR, W_RAM_BANK_SIZE,
};

pub struct Mmu {
    is_cgb: bool,
    oam_transfer: bool,

    mbc: Box<dyn Mbc + 'static>,
    w_ram: Vec<u8>,
    h_ram: [u8; H_RAM_SIZE],
}

impl Mmu {
    pub fn new(mbc: Box<dyn Mbc + 'static>, is_cgb: bool) -> Self {
        Mmu {
            is_cgb,
            oam_transfer: false,

            mbc,
            w_ram: if is_cgb {
                vec![0; W_RAM_BANK_SIZE * 8]
            } else {
                vec![0; W_RAM_BANK_SIZE * 2]
            },
            h_ram: [0; H_RAM_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address < V_RAM_ADDR {
            // rom

            return self.mbc.read_rom(address);
        }

        if address < E_RAM_BANK_ADDR {
            // vram

            if self.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            unimplemented!("VRAM is not implemented!");
        }

        if address < W_RAM_BANK_0_ADDR {
            // eram

            return self.mbc.read_ram(address - E_RAM_BANK_ADDR);
        }

        if address < ECHO_RAM_ADDR {
            // wram

            if self.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            return self.w_ram[(address - W_RAM_BANK_0_ADDR) as usize];
        }

        if address < OAM_ADDR {
            // echo ram

            return self.read_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR);
        }

        if address < UNUSABLE_ADDR {
            // oam

            unimplemented!("OAM not implemented!");
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

            return self.h_ram[(address - H_RAM_ADDR) as usize];
        }

        if address == IE_REGISTER_ADDR {
            // interrupt enable register

            todo!("Implement IE register")
        }

        panic!("Memory read to address 0x{:x} is not implemented!", address);
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address < V_RAM_ADDR {
            // rom is read-only, do nothing

            return;
        }

        if address < E_RAM_BANK_ADDR {
            // vram

            if self.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            unimplemented!("VRAM not implemented!");
        }

        if address < W_RAM_BANK_0_ADDR {
            // eram

            self.mbc.write_ram(address - E_RAM_BANK_ADDR, value);
            return;
        }

        if address < ECHO_RAM_ADDR {
            // wram

            if self.is_cgb {
                unimplemented!("VRAM Banking for CGB not implemented!");
            }

            self.w_ram[(address - W_RAM_BANK_0_ADDR) as usize] = value;
            return;
        }

        if address < OAM_ADDR {
            // echo ram

            self.write_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR, value);
            return;
        }

        if address < UNUSABLE_ADDR {
            // oam

            unimplemented!("OAM not implemented!");
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

            self.h_ram[(address - H_RAM_ADDR) as usize] = value;
            return;
        }

        if address == IE_REGISTER_ADDR {
            // interrupt enable register

            todo!("Implement IE register")
        }

        panic!(
            "Memory write to address 0x{:x} is not implemented!",
            address
        );
    }
}
