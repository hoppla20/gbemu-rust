use tracing::info;

pub trait Serial {
    fn write(&mut self, value: u8);
    fn read(&self) -> u8;

    fn get_transfer_control(&self) -> u8;
    fn set_transfer_control(&mut self, value: u8);

    fn get_last_buffer(&self) -> &String;
}

#[derive(Default)]
pub struct LogSerial {
    transfer_data: char,
    transfer_control: u8,
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
        self.transfer_control
    }

    fn set_transfer_control(&mut self, value: u8) {
        self.transfer_control = value;
        match self.transfer_control {
            0x80 => {
                // receive requested
            },
            0x81 => {
                // transfer requested
                self.transfer()
            },
            _ => {},
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

        self.transfer_control &= !(1 << 7);
    }
}
