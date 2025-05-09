use crate::utils::bit_operations::{bit, extract_bits};

#[derive(Debug, Clone, Copy)]
pub struct LcdControlFlags {
    pub enabled: bool,
    pub window_tile_map: bool,
    pub window_enabled: bool,
    pub tile_data_select: bool,
    pub background_tile_map: bool,
    pub sprite_double_size: bool,
    pub sprite_enabled: bool,
    pub background_window_enabled: bool,
}

impl From<u8> for LcdControlFlags {
    fn from(value: u8) -> Self {
        LcdControlFlags {
            enabled: bit!(value: u8, 7),
            window_tile_map: bit!(value: u8, 6),
            window_enabled: bit!(value: u8, 5),
            tile_data_select: bit!(value: u8, 4),
            background_tile_map: bit!(value: u8, 3),
            sprite_double_size: bit!(value: u8, 2),
            sprite_enabled: bit!(value: u8, 1),
            background_window_enabled: bit!(value: u8, 0),
        }
    }
}

impl From<LcdControlFlags> for u8 {
    fn from(value: LcdControlFlags) -> Self {
        let mut result = 0;
        result |= if value.enabled { 1 << 7 } else { 0 };
        result |= if value.window_tile_map { 1 << 6 } else { 0 };
        result |= if value.window_enabled { 1 << 5 } else { 0 };
        result |= if value.tile_data_select { 1 << 4 } else { 0 };
        result |= if value.background_tile_map { 1 << 3 } else { 0 };
        result |= if value.sprite_double_size { 1 << 2 } else { 0 };
        result |= if value.sprite_enabled { 1 << 1 } else { 0 };
        result |= if value.background_window_enabled {
            1
        } else {
            0
        };
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PpuMode {
    HBlank,
    VBlank,
    OamScan,
    Drawing,
}

impl From<u8> for PpuMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::HBlank,
            1 => Self::VBlank,
            2 => Self::OamScan,
            3 => Self::Drawing,
            _ => panic!("Unknown ppu mode {}", value),
        }
    }
}

impl From<PpuMode> for u8 {
    fn from(value: PpuMode) -> Self {
        match value {
            PpuMode::HBlank => 0,
            PpuMode::VBlank => 1,
            PpuMode::OamScan => 2,
            PpuMode::Drawing => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LcdStatusFlags {
    pub int_lyc_enabled: bool,
    pub int_mode_2_enabled: bool,
    pub int_mode_1_enabled: bool,
    pub int_mode_0_enabled: bool,
    pub ppu_mode: PpuMode,
}

impl From<u8> for LcdStatusFlags {
    fn from(value: u8) -> Self {
        LcdStatusFlags {
            int_lyc_enabled: bit!(value: u8, 6),
            int_mode_2_enabled: bit!(value: u8, 5),
            int_mode_1_enabled: bit!(value: u8, 4),
            int_mode_0_enabled: bit!(value: u8, 3),
            ppu_mode: extract_bits!(value: u8, 0, 1).into(),
        }
    }
}

impl From<LcdStatusFlags> for u8 {
    fn from(value: LcdStatusFlags) -> Self {
        // first bit is unused and always set
        let mut result = 0b1000_0000;
        result |= if value.int_lyc_enabled { 1 << 6 } else { 0 };
        result |= if value.int_mode_2_enabled { 1 << 5 } else { 0 };
        result |= if value.int_mode_1_enabled { 1 << 4 } else { 0 };
        result |= if value.int_mode_0_enabled { 1 << 3 } else { 0 };
        result |= <PpuMode as Into<u8>>::into(value.ppu_mode);
        result
    }
}

pub struct GraphicsRegisters {
    lcd_control: LcdControlFlags,
    lcd_status: LcdStatusFlags,
    lcd_ly: u8,
    lcd_lyc: u8,
    screen_y: u8,
    screen_x: u8,
    window_y: u8,
    window_x: u8,

    // non-cgb mode only
    background_palette: u8,
    obj_palette: [u8; 2],
}

impl GraphicsRegisters {
    pub fn new() -> Self {
        Self {
            lcd_control: 0x91.into(),
            lcd_status: 0x86.into(),
            lcd_ly: 0x00,
            lcd_lyc: 0x00,
            screen_y: 0x00,
            screen_x: 0x00,
            window_y: 0x00,
            window_x: 0x00,
            background_palette: 0xFC,
            obj_palette: [0x00, 0x00],
        }
    }

    pub fn get_lcd_control(&self) -> u8 {
        self.lcd_control.into()
    }

    pub fn get_lcd_status(&self) -> u8 {
        self.lcd_status.into()
    }

    pub fn get_lcd_ly(&self) -> u8 {
        self.lcd_ly
    }

    pub fn get_lcd_lyc(&self) -> u8 {
        self.lcd_lyc
    }

    pub fn get_screen_y(&self) -> u8 {
        self.screen_y
    }

    pub fn get_screen_x(&self) -> u8 {
        self.screen_x
    }

    pub fn get_window_y(&self) -> u8 {
        self.window_y
    }

    pub fn get_window_x(&self) -> u8 {
        self.window_x
    }

    pub fn get_background_palette(&self) -> u8 {
        self.background_palette
    }

    pub fn get_obj_palette(&self, index: usize) -> u8 {
        self.obj_palette[index]
    }

    pub fn set_lcd_control(&mut self, value: u8) {
        self.lcd_control = value.into()
    }

    pub fn set_lcd_status(&mut self, value: u8) {
        self.lcd_status = value.into()
    }

    pub fn set_lcd_ly(&mut self, value: u8) {
        self.lcd_ly = value;
    }

    pub fn set_lcd_lyc(&mut self, value: u8) {
        self.lcd_lyc = value;
    }

    pub fn set_screen_y(&mut self, value: u8) {
        self.screen_y = value;
    }

    pub fn set_screen_x(&mut self, value: u8) {
        self.screen_x = value;
    }

    pub fn set_window_y(&mut self, value: u8) {
        self.window_y = value;
    }

    pub fn set_window_x(&mut self, value: u8) {
        self.window_x = value;
    }

    pub fn set_background_palette(&mut self, value: u8) {
        self.background_palette = value;
    }

    pub fn set_obj_palette(&mut self, index: usize, value: u8) {
        self.obj_palette[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcd_control_flags() {
        let lcdc: LcdControlFlags = 0x00.into();

        assert!(!lcdc.enabled);
        assert!(!lcdc.window_tile_map);
        assert!(!lcdc.window_enabled);
        assert!(!lcdc.tile_data_select);
        assert!(!lcdc.background_tile_map);
        assert!(!lcdc.sprite_double_size);
        assert!(!lcdc.sprite_enabled);
        assert!(!lcdc.background_window_enabled);

        let lcdc: LcdControlFlags = 0xFF.into();

        assert!(lcdc.enabled);
        assert!(lcdc.window_tile_map);
        assert!(lcdc.window_enabled);
        assert!(lcdc.tile_data_select);
        assert!(lcdc.background_tile_map);
        assert!(lcdc.sprite_double_size);
        assert!(lcdc.sprite_enabled);
        assert!(lcdc.background_window_enabled);

        let lcdc: LcdControlFlags = 0b1000_0000.into();

        assert!(lcdc.enabled);
        assert!(!lcdc.window_tile_map);
        assert!(!lcdc.window_enabled);
        assert!(!lcdc.tile_data_select);
        assert!(!lcdc.background_tile_map);
        assert!(!lcdc.sprite_double_size);
        assert!(!lcdc.sprite_enabled);
        assert!(!lcdc.background_window_enabled);

        let lcdc: LcdControlFlags = 0b0000_0001.into();

        assert!(!lcdc.enabled);
        assert!(!lcdc.window_tile_map);
        assert!(!lcdc.window_enabled);
        assert!(!lcdc.tile_data_select);
        assert!(!lcdc.background_tile_map);
        assert!(!lcdc.sprite_double_size);
        assert!(!lcdc.sprite_enabled);
        assert!(lcdc.background_window_enabled);
    }

    #[test]
    fn test_lcd_status_flags() {
        let lcds: LcdStatusFlags = 0x00.into();

        let byte: u8 = lcds.into();
        assert!(bit!(byte: u8, 7));

        assert!(!lcds.int_lyc_enabled);
        assert!(!lcds.int_mode_2_enabled);
        assert!(!lcds.int_mode_1_enabled);
        assert!(!lcds.int_mode_0_enabled);
        assert_eq!(lcds.ppu_mode, PpuMode::HBlank);

        let lcds: LcdStatusFlags = 0xFF.into();

        assert!(lcds.int_lyc_enabled);
        assert!(lcds.int_mode_2_enabled);
        assert!(lcds.int_mode_1_enabled);
        assert!(lcds.int_mode_0_enabled);
        assert_eq!(lcds.ppu_mode, PpuMode::Drawing);

        let lcds: LcdStatusFlags = 0b0100_0000.into();

        assert!(lcds.int_lyc_enabled);
        assert!(!lcds.int_mode_2_enabled);
        assert!(!lcds.int_mode_1_enabled);
        assert!(!lcds.int_mode_0_enabled);
        assert_eq!(lcds.ppu_mode, PpuMode::HBlank);

        let lcds: LcdStatusFlags = 0x0000_0001.into();

        assert!(!lcds.int_lyc_enabled);
        assert!(!lcds.int_mode_2_enabled);
        assert!(!lcds.int_mode_1_enabled);
        assert!(!lcds.int_mode_0_enabled);
        assert_eq!(lcds.ppu_mode, PpuMode::VBlank);
    }
}
