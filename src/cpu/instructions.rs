use std::fmt::Debug;

use super::{Cpu, registers::Register};

#[derive(Debug)]
pub enum Instruction {
    AddAR8 { source: Register },
    SubAR8 { source: Register },
}

enum ExecutionError {
    NoImpl { instruction: Instruction },
    UnknownOpcode { opcode: u8 },
    MemoryWrite { address: u16 },
    MemoryRead { address: u16 },
}

impl Debug for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoImpl { instruction } => f
                .debug_struct("NoImpl")
                .field("instruction", instruction)
                .finish(),
            Self::UnknownOpcode { opcode } => f
                .debug_struct("UnknownOpcode")
                .field("opcode", opcode)
                .finish(),
            Self::MemoryWrite { address } => f
                .debug_struct("MemoryWrite")
                .field("address", address)
                .finish(),
            Self::MemoryRead { address } => f
                .debug_struct("MemoryRead")
                .field("address", address)
                .finish(),
        }
    }
}

impl Cpu {
    fn decode_instruction(&self) -> Result<Instruction, ExecutionError> {
        Ok(Instruction::AddAR8 {
            source: Register::B,
        })
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ExecutionError> {
        match instruction {
            Instruction::AddAR8 { source } => {
                self.instr_add_a_r8(source);
                Ok(())
            },
            _ => Err(ExecutionError::NoImpl { instruction }),
        }
    }

    fn step(&mut self) -> Result<(), ExecutionError> {
        self.execute(self.decode_instruction()?)
    }
}
