use tracing::warn;

use crate::memory::mbc::MbcType;

pub struct CartridgeHeader {
    pub title: String,
    pub mbc_type: MbcType,
    pub rom_banks: usize,
    pub ram_banks: usize,
}

const MBC_TYPE_ADDR: usize = 0x148;
const ROM_SIZE_ADDR: usize = 0x148;
const RAM_SIZE_ADDR: usize = 0x148;

impl From<&Vec<u8>> for CartridgeHeader {
    fn from(buffer: &Vec<u8>) -> Self {
        if buffer.len() < 0x014F {
            warn!("Could not parse cartridge header. Using default MBC Mbc0");

            return CartridgeHeader {
                title: "Unknown".to_owned(),
                mbc_type: MbcType::Mbc0,
                rom_banks: 32 * 1024,
                ram_banks: 0,
            };
        }

        let mut title = String::new();
        for byte in &buffer[0x0134..0x0143] {
            if *byte == 0x00 {
                break;
            }

            title.push(*byte as char);
        }

        let rom_banks = match buffer[ROM_SIZE_ADDR] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            _ => panic!("Unknown ROM size {}", buffer[ROM_SIZE_ADDR]),
        };

        let ram_banks = match buffer[RAM_SIZE_ADDR] {
            0x00 | 0x01 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => panic!("Unknown RAM size {}", buffer[RAM_SIZE_ADDR]),
        };

        CartridgeHeader {
            title,
            mbc_type: buffer[MBC_TYPE_ADDR].into(),
            rom_banks,
            ram_banks,
        }
    }
}
