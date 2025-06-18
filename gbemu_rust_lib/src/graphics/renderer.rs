use std::array::from_fn;

use super::tile::Pixel;
use super::{LCD_HEIGHT, LCD_WIDTH};

pub trait Renderer {
    fn set_pixel(&mut self, pixel: Pixel, y: usize, x: usize);

    fn get_framebuffer(&self) -> [[Pixel; LCD_WIDTH]; LCD_HEIGHT];

    fn v_blank(&mut self);

    fn h_blank(&mut self);
}

pub struct WGPURenderer {
    pub frame_buffer: [[Pixel; LCD_WIDTH]; LCD_HEIGHT],
}

impl Default for WGPURenderer {
    fn default() -> Self {
        Self {
            frame_buffer: from_fn(|_| from_fn(|_| Pixel::default())),
        }
    }
}

impl Renderer for WGPURenderer {
    fn set_pixel(&mut self, pixel: Pixel, y: usize, x: usize) {
        self.frame_buffer[y][x] = pixel;
    }

    fn v_blank(&mut self) {}

    fn h_blank(&mut self) {}

    fn get_framebuffer(&self) -> [[Pixel; LCD_WIDTH]; LCD_HEIGHT] {
        self.frame_buffer
    }
}
