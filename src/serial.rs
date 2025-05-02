use log::debug;

pub trait Serial {
    fn write(&mut self, c: char);
    fn transfer(&mut self, serial_transfer_control: &mut u8);
    fn read(&self) -> u8;
    fn get_last_buffer(&self) -> &String;
}

#[derive(Default)]
pub struct LogSerial {
    current: char,
    buffer: String,
    last_buffer: String,
}

impl Serial for LogSerial {
    fn write(&mut self, c: char) {
        self.current = c;
    }

    fn transfer(&mut self, serial_transfer_control: &mut u8) {
        if self.current == '\n' {
            debug!("Serial output: '{}'", self.buffer);
            self.last_buffer = self.buffer.clone();
            self.buffer.clear();
        } else {
            self.buffer.push(self.current);
        }

        *serial_transfer_control &= !(1 << 7);

        // TODO: request serial interrupt
    }

    fn read(&self) -> u8 {
        0xFF
    }

    fn get_last_buffer(&self) -> &String {
        &self.last_buffer
    }
}
