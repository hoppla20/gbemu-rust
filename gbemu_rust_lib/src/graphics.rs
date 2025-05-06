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

pub struct Ppu {
    pub v_ram: Vec<u8>,
    pub registers: GraphicsRegisters,
}

impl Default for Ppu {
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

impl Ppu {
    pub(crate) fn lcd_control(&self) -> u8 {
        todo!()
    }

    pub(crate) fn lcd_status(&self) -> u8 {
        todo!()
    }

    pub(crate) fn screen_y(&self) -> u8 {
        todo!()
    }

    pub(crate) fn screen_x(&self) -> u8 {
        todo!()
    }

    pub(crate) fn lcd_y(&self) -> u8 {
        todo!()
    }

    pub(crate) fn lcd_y_compare(&self) -> u8 {
        todo!()
    }

    pub(crate) fn background_palette(&self) -> u8 {
        todo!()
    }

    pub(crate) fn obj_palette(&self, arg: i32) -> u8 {
        todo!()
    }

    pub(crate) fn window_y(&self) -> u8 {
        todo!()
    }

    pub(crate) fn window_x(&self) -> u8 {
        todo!()
    }

    pub(crate) fn write_lcd_control(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_lcd_status(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_screen_y(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_screen_x(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_lcd_y(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_lcd_y_compare(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_background_palette(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_obj_palette(&mut self, arg: i32, value: u8) {
        todo!()
    }

    pub(crate) fn write_window_y(&mut self, value: u8) {
        todo!()
    }

    pub(crate) fn write_window_x(&mut self, value: u8) {
        todo!()
    }
}
