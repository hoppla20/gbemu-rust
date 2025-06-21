// TODO: Remove at a later point
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

mod cartridge;
mod cpu;
mod emulator;
mod graphics;
mod joypad;
mod memory;
mod serial;
mod system;
mod timer;

pub mod utils;

pub mod prelude {
    pub use super::cpu::Cpu;
    pub use super::cpu::registers::Registers;
    pub use super::emulator::Emulator;
    pub use super::graphics::tile::Pixel;
    pub use super::memory::mbc::Mbc;
    pub use super::memory::mbc::Mbc0;
    pub use super::memory::mbc::Mbc1;
    pub use super::system::System;

    pub use super::graphics::{LCD_HEIGHT, LCD_WIDTH};
}

#[cfg(test)]
mod tests;
