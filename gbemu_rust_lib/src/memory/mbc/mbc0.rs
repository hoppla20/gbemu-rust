use super::Mbc;

const ROM_SIZE: usize = 0x8000;
const ALLOWED_RAM_SIZES: usize = 0x2000;

pub struct Mbc0 {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Mbc0 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Result<Self, String> {
        Self::new_from_buffer(vec![0; ROM_SIZE])
    }

    pub fn new_from_buffer(buffer: Vec<u8>) -> Result<Self, String> {
        if buffer.len() != ROM_SIZE {
            return Err(format!(
                "The ROM buffer has to be {} bytes big. Got: {}",
                ROM_SIZE,
                buffer.len()
            ));
        }

        Ok(Mbc0 {
            rom: buffer,
            ram: vec![],
        })
    }
}

impl Mbc for Mbc0 {
    fn read_rom(&self, address: u16) -> u8 {
        assert!((address as usize) < ROM_SIZE);

        self.rom[address as usize]
    }
    fn write_rom(&mut self, address: u16, _value: u8) {
        assert!((address as usize) < ROM_SIZE);
    }

    fn read_ram(&self, address: u16) -> u8 {
        assert!((address as usize) < ALLOWED_RAM_SIZES);

        if (address as usize) < self.ram.len() {
            self.ram[address as usize]
        } else {
            0xFF
        }
    }

    fn write_ram(&mut self, address: u16, val: u8) {
        assert!((address as usize) < ALLOWED_RAM_SIZES);

        if (address as usize) < self.ram.len() {
            self.ram[address as usize] = val;
        }
    }
}
