mod rom_only;

pub use rom_only::MbcRomOnly;

pub trait Mbc {
    fn new(rom_size: usize, e_ram_size: usize, is_cgb: bool) -> Self;

    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}
