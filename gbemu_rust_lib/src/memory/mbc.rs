mod mbc0;
mod mbc1;

pub use mbc0::Mbc0;
pub use mbc1::Mbc1;

use crate::cartridge::CartridgeHeader;

#[derive(Debug)]
pub enum MbcType {
    Mbc0,
    Mbc1 { ram: bool, battery: bool },
}

impl From<u8> for MbcType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => MbcType::Mbc0,
            0x01 => MbcType::Mbc1 {
                ram: false,
                battery: false,
            },
            0x02 => MbcType::Mbc1 {
                ram: true,
                battery: false,
            },
            0x03 => MbcType::Mbc1 {
                ram: false,
                battery: true,
            },
            _ => panic!("Unknown MBC type {}", value),
        }
    }
}

pub trait Mbc {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);

    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
}

pub struct CreateError;

pub fn new_mbc_from_buffer(buffer: Vec<u8>) -> Result<Box<dyn Mbc>, String> {
    let header = Into::<CartridgeHeader>::into(&buffer);

    Ok(match header.mbc_type {
        MbcType::Mbc0 => Box::new(Mbc0::new_from_buffer(buffer)?),
        MbcType::Mbc1 { battery, .. } => {
            Box::new(Mbc1::new_from_buffer(buffer, header.ram_banks, battery)?)
        },
    })
}
