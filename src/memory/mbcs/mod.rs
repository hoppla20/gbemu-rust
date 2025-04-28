mod rom_only;

pub use rom_only::Mbc0;

pub trait Mbc {
    fn read_rom(&self, address: u16) -> u8;

    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
}
