use tracing::info;

use crate::utils::bit_operations::bit;

pub trait Serial {
    fn write(&mut self, value: u8);
    fn read(&self) -> u8;

    fn get_transfer_control(&self) -> u8;
    fn set_transfer_control(&mut self, value: u8);

    fn get_last_buffer(&self) -> &String;
}

#[derive(Debug, Default, Clone, Copy)]
struct SerialControl {
    enabled: bool,
    clock_select: bool,
}

impl From<u8> for SerialControl {
    fn from(value: u8) -> Self {
        Self {
            enabled: bit!(value: u8, 7),
            clock_select: bit!(value: u8, 0),
        }
    }
}

impl From<SerialControl> for u8 {
    fn from(value: SerialControl) -> Self {
        let mut result = 0b0111_1110;
        result |= if value.enabled { 1 << 7 } else { 0 };
        result |= if value.clock_select { 1 } else { 0 };
        result
    }
}

#[derive(Debug, Default)]
pub struct LogSerial {
    transfer_data: char,
    transfer_control: SerialControl,
    buffer: String,
    last_buffer: String,
}

impl Serial for LogSerial {
    fn read(&self) -> u8 {
        0xFF
    }

    fn write(&mut self, value: u8) {
        self.transfer_data = value as char;
    }

    fn get_transfer_control(&self) -> u8 {
        self.transfer_control.into()
    }

    fn set_transfer_control(&mut self, value: u8) {
        self.transfer_control = value.into();
        if self.transfer_control.enabled && self.transfer_control.clock_select {
            // transfer requested
            self.transfer()
        } else {
            // receive requested
            self.receive()
        }
    }

    fn get_last_buffer(&self) -> &String {
        &self.last_buffer
    }
}

impl LogSerial {
    fn transfer(&mut self) {
        if self.transfer_data == '\n' {
            info!(name: "serial::transfer", "{}", self.buffer);
            self.last_buffer = self.buffer.clone();
            self.buffer.clear();
        } else {
            self.buffer.push(self.transfer_data);
        }

        self.transfer_control.enabled = false;
    }

    fn receive(&mut self) {
        self.transfer_control.enabled = false;
    }
}
