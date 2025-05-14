pub mod object;
pub mod registers;
pub mod tile;

use crate::memory::V_RAM_BANK_SIZE;

use registers::GraphicsRegisters;
use registers::PpuMode;
use tracing::debug;
use tracing::instrument;

const LCD_WIDTH: u8 = 160;
const LCD_HEIGHT: u8 = 144;

const MODE_OAM_SCAN_CYCLES: u16 = 80 / 4;
const MODE_DRAWING_CYCLES: u16 = 172 / 4;
const MODE_H_BLANK_CYCLES: u16 = 204 / 4;

const SCANLINE_CYCLES: u16 = MODE_OAM_SCAN_CYCLES + MODE_DRAWING_CYCLES + MODE_H_BLANK_CYCLES;

const NUM_LINES: u8 = 153;

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
    #[cfg(not(feature = "nogfx"))]
    #[instrument(skip_all, fields(
        ppu_mode = format!("{:?}", self.registers.lcd_status.ppu_mode),
        scanline_cycle = self.scanline_cycle
    ))]
    pub fn step(&mut self) {
        match self.registers.lcd_status.ppu_mode {
            PpuMode::HBlank => {
                // TODO: render scanline to framebuffer

                if self.scanline_cycle == MODE_OAM_SCAN_CYCLES + MODE_DRAWING_CYCLES {
                    debug!(
                        "Rendering scanline {} to framebuffer",
                        self.registers.get_lcd_ly()
                    );
                }

                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle == SCANLINE_CYCLES {
                    self.scanline_cycle = 0;
                    self.registers.inc_lcd_ly();
                    if self.registers.get_lcd_ly() < LCD_HEIGHT {
                        self.registers.lcd_status.ppu_mode = PpuMode::OamScan;
                    } else {
                        self.registers.lcd_status.ppu_mode = PpuMode::VBlank;
                    }
                }
            },
            PpuMode::VBlank => {
                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle == SCANLINE_CYCLES {
                    self.scanline_cycle = 0;
                    if self.registers.get_lcd_ly() == NUM_LINES {
                        self.registers.set_lcd_ly(0);
                        self.registers.lcd_status.ppu_mode = PpuMode::OamScan;
                    } else {
                        self.registers
                            .set_lcd_ly(self.registers.get_lcd_ly().wrapping_add(1));
                    }
                }
            },
            PpuMode::OamScan => {
                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle == MODE_OAM_SCAN_CYCLES {
                    self.registers.lcd_status.ppu_mode = PpuMode::Drawing;
                }
            },
            PpuMode::Drawing => {
                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle == MODE_OAM_SCAN_CYCLES + MODE_DRAWING_CYCLES {
                    self.registers.lcd_status.ppu_mode = PpuMode::HBlank;
                }
            },
        }
    }

    #[cfg(feature = "nogfx")]
    pub fn step(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let mut ppu = Ppu::default();

        for line in 0..LCD_HEIGHT {
            println!("Line {:03}", line);
            assert_eq!(ppu.registers.get_lcd_ly(), line);

            for _ in 0..MODE_OAM_SCAN_CYCLES {
                println!("Cycle {:03}", ppu.scanline_cycle);
                assert_eq!(ppu.registers.lcd_status.ppu_mode, PpuMode::OamScan);
                ppu.step();
            }

            for _ in 0..MODE_DRAWING_CYCLES {
                println!("Cycle {:03}", ppu.scanline_cycle);
                assert_eq!(ppu.registers.lcd_status.ppu_mode, PpuMode::Drawing);
                ppu.step();
            }

            for _ in 0..MODE_H_BLANK_CYCLES {
                println!("Cycle {:03}", ppu.scanline_cycle);
                assert_eq!(ppu.registers.lcd_status.ppu_mode, PpuMode::HBlank);
                ppu.step();
            }
        }

        for v_blank_line in 0..10 {
            println!("VBlank line {:03}", v_blank_line);
            for _ in 0..SCANLINE_CYCLES {
                println!("Cycle {:03}", ppu.scanline_cycle);
                assert_eq!(ppu.registers.get_lcd_ly(), LCD_HEIGHT + v_blank_line);
                assert_eq!(ppu.registers.lcd_status.ppu_mode, PpuMode::VBlank);
                ppu.step();
            }
        }
    }
}
