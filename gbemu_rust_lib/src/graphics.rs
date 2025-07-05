pub mod object;
pub mod registers;
pub mod renderer;
pub mod tile;

use std::array::from_fn;
use std::cmp::min;

use object::Object;
use registers::GraphicsRegisters;
use registers::PpuMode;
use renderer::Renderer;
use renderer::WGPURenderer;
use tile::TileData;
use tile::TileMap;
use tracing::instrument;
use tracing::trace;

use crate::memory::OAM_SIZE;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

const MODE_OAM_SCAN_CYCLES: usize = 80 / 4;
const MODE_DRAWING_CYCLES: usize = 172 / 4;
const MODE_H_BLANK_CYCLES: usize = 204 / 4;

const SCANLINE_CYCLES: usize = MODE_OAM_SCAN_CYCLES + MODE_DRAWING_CYCLES + MODE_H_BLANK_CYCLES;

const NUM_LINES: usize = 153;

pub struct Ppu {
    pub registers: GraphicsRegisters,
    pub tile_data: TileData,
    pub tile_maps: [TileMap; 2],

    oam: [u8; OAM_SIZE],
    object_buffer: Vec<Object>,

    pub renderer: Box<dyn Renderer>,

    scanline_cycle: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            registers: GraphicsRegisters::new(),
            tile_data: TileData::default(),
            tile_maps: from_fn(|_| TileMap::default()),

            oam: [0; OAM_SIZE],
            object_buffer: Vec::with_capacity(10),

            renderer: Box::new(WGPURenderer::default()),

            scanline_cycle: 0,
        }
    }
}

impl Ppu {
    pub fn read_oam_byte(&self, address: u16) -> u8 {
        assert!((address as usize) < OAM_SIZE);

        self.oam[address as usize]
    }

    pub fn write_oam_byte(&mut self, address: u16, value: u8) {
        assert!((address as usize) < OAM_SIZE);

        self.oam[address as usize] = value;
    }

    #[instrument(skip_all, fields(ly = self.registers.get_lcd_ly()))]
    pub fn render_background(&mut self) {
        let tile_map = &self.tile_maps[self.registers.lcd_control.background_tile_map as usize];

        let map_y = (self
            .registers
            .get_screen_y()
            .overflowing_add(self.registers.get_lcd_ly()))
        .0 as usize;
        let map_tile_y = map_y / 8;
        let mut screen_x = 0;

        for tile_x in 0..(LCD_WIDTH / 8) + 1 {
            let map_tile_x = self.registers.get_screen_x() as usize + tile_x;
            let tile_number = tile_map.tiles[map_tile_y][map_tile_x];
            let tile = self
                .tile_data
                .get_tile(self.registers.lcd_control.tile_data_select, tile_number);
            let row = &tile.rows[map_y % 8];

            let mut iter_start = 0;
            let mut iter_stop = 8;
            if tile_x == 0 {
                iter_start = self.registers.get_screen_x() % 8;
            }
            if self.registers.get_screen_x() as usize + (tile_x * 8) + 7 >= LCD_WIDTH {
                iter_stop = LCD_WIDTH as u8 - (self.registers.get_screen_x() + (tile_x as u8 * 8));
            }
            for i in iter_start..iter_stop {
                let pixel = row.get_pixel(i as usize);

                self.renderer
                    .set_pixel(pixel, self.registers.get_lcd_ly() as usize, screen_x);

                screen_x += 1;
            }
        }
    }

    pub fn render_window(&mut self) {}

    pub fn render_objects(&mut self) {
        for obj in &self.object_buffer {
            let tile = self.tile_data.get_tile(true, obj.tile_number);
            // TODO: double height mode
            let tile_row = self.registers.get_lcd_ly() - (obj.pos_y - 16);
            let pixels = tile.get_row(tile_row as usize);

            let low = min(0, obj.pos_x as isize - 8).unsigned_abs();
            let high = min(8, LCD_WIDTH - obj.pos_x as usize);

            (low..high).for_each(|x| {
                self.renderer.set_pixel(
                    pixels[x],
                    self.registers.get_lcd_ly() as usize,
                    obj.pos_x as usize - 8 + x,
                );
            });
        }
    }

    #[cfg(not(feature = "nogfx"))]
    #[instrument(skip_all, fields(
        ppu_mode = format!("{:?}", self.registers.lcd_status.ppu_mode),
        scanline_cycle = self.scanline_cycle
    ))]
    pub fn step(&mut self) -> bool {
        let mut interrupt = false;

        match self.registers.lcd_status.ppu_mode {
            PpuMode::HBlank => {
                if self.scanline_cycle as usize == MODE_OAM_SCAN_CYCLES + MODE_DRAWING_CYCLES {
                    trace!(
                        "Rendering scanline {} to framebuffer",
                        self.registers.get_lcd_ly()
                    );

                    self.render_background();
                    self.render_window();
                    self.render_objects();

                    self.renderer.h_blank();
                }

                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle as usize == SCANLINE_CYCLES {
                    self.scanline_cycle = 0;
                    self.registers.inc_lcd_ly();
                    self.object_buffer.clear();
                    if (self.registers.get_lcd_ly() as usize) < LCD_HEIGHT {
                        self.registers.lcd_status.ppu_mode = PpuMode::OamScan;
                    } else {
                        self.registers.lcd_status.ppu_mode = PpuMode::VBlank;
                    }
                }
            },
            PpuMode::VBlank => {
                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle as usize == SCANLINE_CYCLES {
                    self.scanline_cycle = 0;
                    if self.registers.get_lcd_ly() as usize == NUM_LINES {
                        self.registers.set_lcd_ly(0);
                        self.registers.lcd_status.ppu_mode = PpuMode::OamScan;
                    } else {
                        if self.registers.get_lcd_ly() as usize == LCD_HEIGHT {
                            self.renderer.v_blank();
                            interrupt = true;
                        }
                        self.registers
                            .set_lcd_ly(self.registers.get_lcd_ly().wrapping_add(1));
                    }
                }
            },
            PpuMode::OamScan => {
                if self.scanline_cycle == 0 {
                    trace!(
                        "Searching for objects on scanline {}",
                        self.registers.get_lcd_ly()
                    );

                    for i in 0..OAM_SIZE / 4 {
                        let obj: Object = self.oam[i * 4..(i + 1) * 4].into();

                        let obj_height = if self.registers.lcd_control.sprite_double_size {
                            16
                        } else {
                            8
                        };

                        if self.object_buffer.len() < 10
                            && (self.registers.get_lcd_ly() >= obj.pos_y - 16)
                            && (self.registers.get_lcd_ly() < obj.pos_y + obj_height - 16)
                        {
                            trace!("Found object {:?}", obj);
                            self.object_buffer.push(obj);
                        }
                    }
                }

                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle as usize == MODE_OAM_SCAN_CYCLES {
                    self.registers.lcd_status.ppu_mode = PpuMode::Drawing;
                }
            },
            PpuMode::Drawing => {
                self.scanline_cycle = self.scanline_cycle.wrapping_add(1);
                if self.scanline_cycle as usize == MODE_OAM_SCAN_CYCLES + MODE_DRAWING_CYCLES {
                    self.registers.lcd_status.ppu_mode = PpuMode::HBlank;
                }
            },
        }

        return interrupt;
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

        for _ in 0..2 {
            for line in 0..LCD_HEIGHT {
                println!("Line {:03}", line);
                assert_eq!(ppu.registers.get_lcd_ly() as usize, line);

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
                    assert_eq!(
                        ppu.registers.get_lcd_ly() as usize,
                        LCD_HEIGHT + v_blank_line as usize
                    );
                    assert_eq!(ppu.registers.lcd_status.ppu_mode, PpuMode::VBlank);
                    ppu.step();
                }
            }
        }
    }
}
