use crate::memory::V_RAM_BANK_SIZE;

pub struct GraphicsRegisters {
    pub lcd_control: u8,
    pub lcd_y: u8,
    pub lcd_y_compare: u8,
    pub lcd_status: u8,
    pub screen_y: u8,
    pub screen_x: u8,
    pub window_y: u8,
    pub window_x: u8,

    // non-cgb mode only
    pub background_palette: u8,
    pub obj_palette: [u8; 2],
}

pub struct GraphicsState {
    pub v_ram: Vec<u8>,
    pub registers: GraphicsRegisters,
}

impl Default for GraphicsState {
    fn default() -> Self {
        Self {
            v_ram: vec![0; V_RAM_BANK_SIZE],
            registers: GraphicsRegisters {
                lcd_control: 0x91,
                lcd_y: 0x00,
                lcd_y_compare: 0x00,
                lcd_status: 0x86,
                screen_y: 0x00,
                screen_x: 0x00,
                window_y: 0x00,
                window_x: 0x00,
                background_palette: 0xFC,
                obj_palette: [0x00, 0x00],
            },
        }
    }
}
