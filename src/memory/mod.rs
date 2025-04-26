pub mod mbcs;

pub(super) const ROM_BANK_SIZE: usize = 0x4000;
pub(super) const V_RAM_BANK_SIZE: usize = 0x2000;
pub(super) const E_RAM_BANK_SIZE: usize = 0x2000;
pub(super) const W_RAM_BANK_SIZE: usize = 0x1000;
pub(super) const ECHO_RAM_SIZE: usize = 0x1E00;
pub(super) const OAM_SIZE: usize = 0x00A0;
pub(super) const UNUSABLE_SIZE: usize = 0x0060;
pub(super) const IO_REGISTERS_SIZE: usize = 0x0080;
pub(super) const H_RAM_SIZE: usize = 0x007F;

pub(super) const ROM_BANK_0_ADDR: u16 = 0x0000;
pub(super) const ROM_BANK_X_ADDR: u16 = ROM_BANK_0_ADDR + (ROM_BANK_SIZE as u16);
pub(super) const V_RAM_ADDR: u16 = ROM_BANK_X_ADDR + (ROM_BANK_SIZE as u16);
pub(super) const E_RAM_BANK_ADDR: u16 = V_RAM_ADDR + (V_RAM_BANK_SIZE as u16);
pub(super) const W_RAM_BANK_0_ADDR: u16 = E_RAM_BANK_ADDR + (E_RAM_BANK_SIZE as u16);
pub(super) const W_RAM_BANK_X_ADDR: u16 = W_RAM_BANK_0_ADDR + (W_RAM_BANK_SIZE as u16);
pub(super) const ECHO_RAM_ADDR: u16 = W_RAM_BANK_X_ADDR + (W_RAM_BANK_SIZE as u16);
pub(super) const OAM_ADDR: u16 = ECHO_RAM_ADDR + (ECHO_RAM_SIZE as u16);
pub(super) const UNUSABLE_ADDR: u16 = OAM_ADDR + (OAM_SIZE as u16);
pub(super) const IO_REGISTERS_ADDR: u16 = UNUSABLE_ADDR + (UNUSABLE_SIZE as u16);
pub(super) const H_RAM_ADDR: u16 = IO_REGISTERS_ADDR + (IO_REGISTERS_SIZE as u16);
pub(super) const IE_REGISTER_ADDR: u16 = H_RAM_ADDR + (H_RAM_SIZE as u16);

pub struct Memory {
    pub rom: Vec<u8>,
    pub e_ram: Vec<u8>,
    pub w_ram: Vec<u8>,
    pub h_ram: [u8; H_RAM_SIZE],
    pub v_ram: Vec<u8>,
    pub oam: [u8; OAM_SIZE],

    is_cgb: bool,
}

impl Memory {
    fn new(rom_size: usize, e_ram_size: usize, is_cgb: bool) -> Self {
        if rom_size % ROM_BANK_SIZE != 0 {
            panic!(
                "The rom size has to be a multiple of 0x{:x}! Got: 0x{:x}",
                ROM_BANK_SIZE, rom_size
            );
        }

        if e_ram_size % E_RAM_BANK_SIZE != 0 {
            panic!(
                "The external ram size has to be a multiple of 0x{:x}! Got: 0x{:x}",
                E_RAM_BANK_SIZE, e_ram_size
            );
        }

        let w_ram_size = if is_cgb {
            W_RAM_BANK_SIZE * 8
        } else {
            W_RAM_BANK_SIZE * 2
        };

        let v_ram_size = if is_cgb {
            V_RAM_BANK_SIZE * 2
        } else {
            V_RAM_BANK_SIZE
        };

        Memory {
            rom: vec![0; rom_size],
            e_ram: vec![0; e_ram_size],
            w_ram: vec![0; w_ram_size],
            h_ram: [0; H_RAM_SIZE],
            v_ram: vec![0; v_ram_size],
            oam: [0; OAM_SIZE],

            is_cgb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_mapping() {
        assert_eq!(ROM_BANK_0_ADDR, 0x0000);
        assert_eq!(ROM_BANK_X_ADDR, 0x4000);
        assert_eq!(V_RAM_ADDR, 0x8000);
        assert_eq!(E_RAM_BANK_ADDR, 0xA000);
        assert_eq!(W_RAM_BANK_0_ADDR, 0xC000);
        assert_eq!(W_RAM_BANK_X_ADDR, 0xD000);
        assert_eq!(ECHO_RAM_ADDR, 0xE000);
        assert_eq!(OAM_ADDR, 0xFE00);
        assert_eq!(UNUSABLE_ADDR, 0xFEA0);
        assert_eq!(IO_REGISTERS_ADDR, 0xFF00);
        assert_eq!(H_RAM_ADDR, 0xFF80);
        assert_eq!(IE_REGISTER_ADDR, 0xFFFF);
    }
}
