// TODO: Remove at a later point
#![allow(dead_code)]
#![allow(unused_macros)]

mod utils;

mod cpu;
mod graphics;
mod memory;
mod timer;

pub mod prelude {
    pub use super::{
        cpu::Cpu, cpu::registers::Registers, memory::mbc::Mbc, memory::mbc::Mbc0, memory::mmu::Mmu,
    };
}

#[cfg(test)]
mod tests;
