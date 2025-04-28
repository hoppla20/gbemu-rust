use super::Mbc;

const ROM_SIZE: usize = 0x8000;
const E_RAM_SIZE_ALLOWED: usize = 0x2000;

pub struct Mbc0 {
    rom: [u8; ROM_SIZE],
    ram: Vec<u8>,
}

impl Mbc0 {
    pub fn new(with_ram: bool) -> Self {
        Mbc0 {
            rom: [0; ROM_SIZE],
            ram: if with_ram {
                vec![0; E_RAM_SIZE_ALLOWED]
            } else {
                vec![]
            },
        }
    }

    pub fn new_from_buffer(buffer: &[u8], with_ram: bool) -> Self {
        if buffer.len() > ROM_SIZE {
            panic!("Buffer size is not allowed to exceed {} bytes!", ROM_SIZE);
        }

        let mut rom = [0_u8; ROM_SIZE];
        rom[0..buffer.len()].copy_from_slice(buffer);

        Mbc0 {
            rom,
            ram: if with_ram {
                vec![0; E_RAM_SIZE_ALLOWED]
            } else {
                vec![]
            },
        }
    }
}

impl Mbc for Mbc0 {
    fn read_rom(&self, addr: u16) -> u8 {
        assert!((addr as usize) < ROM_SIZE);

        self.rom[addr as usize]
    }

    fn read_ram(&self, addr: u16) -> u8 {
        assert!((addr as usize) < E_RAM_SIZE_ALLOWED);

        if (addr as usize) < self.ram.len() {
            self.ram[addr as usize]
        } else {
            0xFF
        }
    }

    fn write_ram(&mut self, addr: u16, val: u8) {
        assert!((addr as usize) < E_RAM_SIZE_ALLOWED);

        if (addr as usize) < self.ram.len() {
            self.ram[addr as usize] = val;
        }
    }
}
