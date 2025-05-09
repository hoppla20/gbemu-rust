pub mod object;
pub mod registers;
pub mod tile;

use crate::memory::V_RAM_BANK_SIZE;

use registers::GraphicsRegisters;
use registers::PpuMode;
use tracing::debug;
use tracing::instrument;

pub struct Ppu {
    pub v_ram: Vec<u8>,
    pub registers: GraphicsRegisters,

    scanline_cycle: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            v_ram: vec![0; V_RAM_BANK_SIZE],
            registers: GraphicsRegisters::new(),

            scanline_cycle: 0,
        }
    }
}

impl Ppu {
    #[instrument(skip_all, fields(
        ppu_mode = format!("{:?}", self.registers.lcd_status.ppu_mode),
        scanline_cycle = self.scanline_cycle
    ))]
    pub fn step(&mut self, enabled: bool) {
        if enabled {
            match self.registers.lcd_status.ppu_mode {
                PpuMode::HBlank => {
                    // TODO: render scanline to framebuffer

                    if self.scanline_cycle == 252 {
                        debug!(
                            "Rendering scanline {} to framebuffer",
                            self.registers.get_lcd_ly()
                        );
                    }

                    self.scanline_cycle += 1;
                    if self.scanline_cycle == 456 {
                        self.registers.lcd_status.ppu_mode = PpuMode::VBlank;
                    }
                },
                PpuMode::VBlank => {
                    self.scanline_cycle += 1;
                    if self.scanline_cycle == 456 {
                        if self.registers.get_lcd_ly() == 153 {
                            self.registers.set_lcd_ly(0);
                            self.registers.lcd_status.ppu_mode = PpuMode::OamScan;
                        } else {
                            self.registers
                                .set_lcd_ly(self.registers.get_lcd_ly().wrapping_add(1));
                        }
                    }
                },
                PpuMode::OamScan => {
                    self.scanline_cycle += 1;
                    if self.scanline_cycle == 80 {
                        self.registers.lcd_status.ppu_mode = PpuMode::Drawing;
                    }
                },
                PpuMode::Drawing => {
                    self.scanline_cycle += 1;
                    if self.scanline_cycle == 252 {
                        self.registers.lcd_status.ppu_mode = PpuMode::HBlank;
                    }
                },
            }
        }
    }
}
