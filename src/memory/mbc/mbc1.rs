use std::process::exit;
use tracing::error;

use crate::memory::{E_RAM_BANK_SIZE, ROM_BANK_SIZE};

use super::Mbc;

const ALLOWED_ROM_BANKS: [usize; 7] = [2, 4, 8, 16, 32, 64, 128];
const MAX_ROM_BANKS: usize = ALLOWED_RAM_BANKS[ALLOWED_RAM_BANKS.len() - 1];
const ALLOWED_RAM_BANKS: [usize; 3] = [0, 1, 4];

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    num_rom_banks: usize,
    num_ram_banks: usize,

    banking_mode_advanced: bool,
    ram_enabled: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
}

impl Mbc1 {
    pub fn new(num_rom_banks: usize, num_ram_banks: usize, _has_battery: bool) -> Self {
        Self::new_from_buffer(
            vec![0; num_rom_banks * ROM_BANK_SIZE],
            num_ram_banks,
            _has_battery,
        )
    }

    pub fn new_from_buffer(buffer: Vec<u8>, num_ram_banks: usize, _has_battery: bool) -> Self {
        if buffer.len() % ROM_BANK_SIZE != 0 {
            error!("The ROM buffer is not a multiple of the ROM bank size");
            exit(1);
        }

        let rom_banks = buffer.len() / ROM_BANK_SIZE;

        if !ALLOWED_ROM_BANKS.contains(&rom_banks) {
            error!(
                "Mbc1 does not support ROM buffers with size {} bytes ({} banks)",
                buffer.len(),
                rom_banks
            );
            error!("Allowed number of banks: {:?}", ALLOWED_ROM_BANKS);
            exit(1);
        }

        if !ALLOWED_RAM_BANKS.contains(&num_ram_banks) {
            error!(
                "Mbc1 does not support RAM with size {} bytes",
                num_ram_banks
            );
            exit(1);
        }

        let extra_large_rom = rom_banks > 32;
        if extra_large_rom && num_ram_banks != 1 {
            panic!("Mbc1 with ROM size >512 KiB only supports RAM with size 8 KiB");
        }

        Mbc1 {
            rom: buffer,
            ram: vec![0; num_ram_banks * E_RAM_BANK_SIZE],

            num_rom_banks: rom_banks,
            num_ram_banks,

            banking_mode_advanced: false,
            ram_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        assert!((address as usize) < 2 * ROM_BANK_SIZE);

        if (address as usize) < ROM_BANK_SIZE {
            // ROM bank 0
            self.rom[address as usize]
        } else {
            // ROM bank 1-127
            let bit_mask = (self.num_rom_banks - 1) & 0b11111;
            let mut bank = (self.rom_bank_number as usize) & bit_mask;
            if bank == 0 {
                bank = 1
            }
            let real_address = (bank * ROM_BANK_SIZE) + ((address as usize) % ROM_BANK_SIZE);

            if self.num_rom_banks > 32 {
                todo!("Extra large Mbc1 ROM");
            }

            self.rom[real_address]
        }
    }
    fn write_rom(&mut self, address: u16, value: u8) {
        assert!((address as usize) < 2 * ROM_BANK_SIZE);

        if address < 0x2000 {
            self.ram_enabled = value & 0x0F == 0x0A;
        } else if address < 0x6000 {
            if !self.num_rom_banks > 32 {
                if address < 0x4000 {
                    self.rom_bank_number = value;
                }
                if address < 0x6000 {
                    self.ram_bank_number = value;
                }
            } else {
                todo!("Extra large Mbc1 ROM");
            }
        } else if address < 0x8000 {
            self.banking_mode_advanced = (value & 0x01) > 0;
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        assert!((address as usize) < E_RAM_BANK_SIZE);

        if !self.ram_enabled {
            return 0xFF;
        }

        let real_address =
            (((self.ram_bank_number & 0b11) as usize) * E_RAM_BANK_SIZE) + (address as usize);
        if self.num_rom_banks > 32 {
            todo!("Extra larg Mbc1 ROM");
        }

        self.ram[real_address]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        assert!((address as usize) < E_RAM_BANK_SIZE);

        if !self.ram_enabled {
            return;
        }

        let real_address =
            (((self.ram_bank_number & 0b11) as usize) * E_RAM_BANK_SIZE) + (address as usize);
        if self.num_rom_banks > 32 {
            todo!("Extra larg Mbc1 ROM");
        }

        self.ram[real_address] = value;
    }
}

#[cfg(test)]
mod test {
    use crate::tests::setup_default_logger;

    use super::*;

    #[test]
    fn test_rom_simple() {
        let _guard = setup_default_logger();

        let mut mbc = Mbc1::new(4, 0, false);

        // bank 0
        mbc.rom[0x0100] = 1;
        // bank 1
        mbc.rom[0x4000] = 1;
        mbc.rom[0x4001] = 2;
        // bank 2
        mbc.rom[0x8000] = 2;
        mbc.rom[0x8001] = 1;
        // bank 3
        mbc.rom[0xC000] = 2;
        mbc.rom[0xC001] = 2;

        // bank 0
        assert_eq!(mbc.read_rom(0x0100), 1);
        // bank 1
        assert_eq!(mbc.read_rom(0x4000), 1);
        assert_eq!(mbc.read_rom(0x4001), 2);
        // bank 2
        mbc.write_rom(0x2000, 0x02);
        assert_eq!(mbc.read_rom(0x2000), 0x00);
        assert_eq!(mbc.read_rom(0x4000), 2);
        assert_eq!(mbc.read_rom(0x4001), 1);
        // bank 3
        mbc.write_rom(0x3FFF, 0x03);
        assert_eq!(mbc.read_rom(0x3FFF), 0x00);
        assert_eq!(mbc.read_rom(0x4000), 2);
        assert_eq!(mbc.read_rom(0x4001), 2);
    }

    #[test]
    fn test_ram_simple() {
        let _guard = setup_default_logger();

        let mut mbc = Mbc1::new(2, 4, false);

        // bank 0
        mbc.ram[0x0000] = 1;
        // bank 1
        mbc.ram[0x2000] = 1;
        mbc.ram[0x2001] = 2;
        // bank 2
        mbc.ram[0x4000] = 2;
        mbc.ram[0x4001] = 1;
        // bank 3
        mbc.ram[0x6000] = 2;
        mbc.ram[0x6001] = 2;

        // ram disabled
        assert_eq!(mbc.read_ram(0x0000), 0xFF);
        // enable ram
        mbc.write_rom(0x0000, 0x0A);

        // bank 0
        assert_eq!(mbc.read_ram(0x0000), 1);
        // bank 1
        mbc.write_rom(0x4000, 0x01);
        assert_eq!(mbc.read_ram(0x0000), 1);
        assert_eq!(mbc.read_ram(0x0001), 2);
        // bank 2
        mbc.write_rom(0x5000, 0x02);
        assert_eq!(mbc.read_rom(0x5000), 0x00);
        assert_eq!(mbc.read_ram(0x0000), 2);
        assert_eq!(mbc.read_ram(0x0001), 1);
        // bank 3
        mbc.write_rom(0x5FFF, 0x03);
        assert_eq!(mbc.read_rom(0x5FFF), 0x00);
        assert_eq!(mbc.read_ram(0x0000), 2);
        assert_eq!(mbc.read_ram(0x0001), 2);
    }
}
