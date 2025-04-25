mod arithmetic;
mod instructions;
mod registers;

use registers::Registers;

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
}
