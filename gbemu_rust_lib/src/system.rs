use tracing::debug;

use crate::cpu::interrupts::InterruptFlags;
use crate::graphics::Ppu;
use crate::memory::mbc::Mbc;
use crate::memory::{
    E_RAM_BANK_ADDR, ECHO_RAM_ADDR, H_RAM_ADDR, H_RAM_SIZE, IE_REGISTER_ADDR, IO_REGISTERS_ADDR,
    OAM_ADDR, UNUSABLE_ADDR, V_RAM_ADDR, W_RAM_BANK_0_ADDR, W_RAM_BANK_SIZE,
};
use crate::serial::Serial;
use crate::timer::TimerRegisters;

static CYCLES_PER_CLOCK_LOOKUP: [u16; 4] = [256, 4, 16, 64];

pub struct IoRegisters {
    pub interrupt_flags: InterruptFlags,
    pub interrupt_enable: u8,
    pub timer: TimerRegisters,
    pub serial: Box<dyn Serial>,
}

impl IoRegisters {
    pub fn new(serial: Box<dyn Serial>) -> Self {
        IoRegisters {
            serial,
            interrupt_flags: 0.into(),
            interrupt_enable: 0,
            timer: TimerRegisters::default(),
        }
    }
}

pub struct System {
    oam_transfer: bool,

    mbc: Box<dyn Mbc + 'static>,
    w_ram: Vec<u8>,
    h_ram: [u8; H_RAM_SIZE],

    pub io: IoRegisters,
    pub graphics: Ppu,
}

impl System {
    pub fn new(mbc: Box<dyn Mbc + 'static>, serial: Box<dyn Serial>) -> Self {
        System {
            oam_transfer: false,

            mbc,
            w_ram: vec![0; W_RAM_BANK_SIZE * 2],
            h_ram: [0; H_RAM_SIZE],

            io: IoRegisters::new(serial),
            graphics: Ppu::default(),
        }
    }

    pub fn get_io_register(&self, address: u16) -> u8 {
        match address {
            // serial
            0xFF01 => self.io.serial.read(),
            0xFF02 => self.io.serial.get_transfer_control(),

            //timer
            0xFF04 => self.io.timer.divider(),
            0xFF05 => self.io.timer.counter,
            0xFF06 => self.io.timer.modulo,
            0xFF07 => self.io.timer.control,

            // interrupt
            0xFF0F => self.io.interrupt_flags.into(),

            // TODO: audio
            0xFF10..=0xFF26 => {
                debug!(
                    "Reading from unimplemented audio register at address 0x{:02X}. Returning 0x00",
                    address
                );
                0x00
            },

            // graphics
            0xFF40 => self.graphics.registers.get_lcd_control(),
            0xFF41 => self.graphics.registers.get_lcd_status(),
            0xFF42 => self.graphics.registers.get_screen_y(),
            0xFF43 => self.graphics.registers.get_screen_x(),
            0xFF44 => self.graphics.registers.get_lcd_ly(),
            0xFF45 => self.graphics.registers.get_lcd_lyc(),
            0xFF47 => self.graphics.registers.get_background_palette(),
            0xFF48 => self.graphics.registers.get_obj_palette(0),
            0xFF49 => self.graphics.registers.get_obj_palette(1),
            0xFF4A => self.graphics.registers.get_window_y(),
            0xFF4B => self.graphics.registers.get_window_x(),
            _ => {
                debug!("Reading from unimplemented i/o register 0x{:02X}", address);
                0xFF
            },
        }
    }

    pub fn write_io_register(&mut self, address: u16, value: u8) {
        match address {
            // serial
            0xFF01 => self.io.serial.write(value),
            0xFF02 => self.io.serial.set_transfer_control(value),

            //timer
            0xFF04 => self.io.timer.reset_divider(),
            0xFF05 => self.io.timer.counter = value,
            0xFF06 => self.io.timer.modulo = value,
            0xFF07 => self.io.timer.control = value,

            // interrupt
            0xFF0F => self.io.interrupt_flags = value.into(),

            // TODO: audio
            0xFF10..=0xFF26 => {
                debug!(
                    "Writing to unimplemented audio register at address 0x{:02X}. Returning 0x00",
                    address
                );
            },

            // graphics
            0xFF40 => self.graphics.registers.set_lcd_control(value),
            0xFF41 => self.graphics.registers.set_lcd_status(value),
            0xFF42 => self.graphics.registers.set_screen_y(value),
            0xFF43 => self.graphics.registers.set_screen_x(value),
            0xFF44 => self.graphics.registers.set_lcd_ly(value),
            0xFF45 => self.graphics.registers.set_lcd_lyc(value),
            0xFF47 => self.graphics.registers.set_background_palette(value),
            0xFF48 => self.graphics.registers.set_obj_palette(0, value),
            0xFF49 => self.graphics.registers.set_obj_palette(1, value),
            0xFF4A => self.graphics.registers.set_window_y(value),
            0xFF4B => self.graphics.registers.set_window_x(value),
            _ => {
                debug!("Writing to unimplemented i/o register 0x{:02X}", address);
            },
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..V_RAM_ADDR => self.mbc.read_rom(address),
            V_RAM_ADDR..E_RAM_BANK_ADDR => self.graphics.v_ram[(address - V_RAM_ADDR) as usize],
            E_RAM_BANK_ADDR..W_RAM_BANK_0_ADDR => self.mbc.read_ram(address - E_RAM_BANK_ADDR),
            W_RAM_BANK_0_ADDR..ECHO_RAM_ADDR => self.w_ram[(address - W_RAM_BANK_0_ADDR) as usize],
            ECHO_RAM_ADDR..OAM_ADDR => self.read_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR),
            OAM_ADDR..UNUSABLE_ADDR => unimplemented!("OAM not implemented!"),
            UNUSABLE_ADDR..IO_REGISTERS_ADDR => {
                if self.oam_transfer {
                    0xFF
                } else {
                    0x00
                }
            },
            IO_REGISTERS_ADDR..H_RAM_ADDR => self.get_io_register(address),
            H_RAM_ADDR..IE_REGISTER_ADDR => self.h_ram[(address - H_RAM_ADDR) as usize],
            IE_REGISTER_ADDR => self.io.interrupt_enable,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..V_RAM_ADDR => self.mbc.write_rom(address, value),
            V_RAM_ADDR..E_RAM_BANK_ADDR => {
                self.graphics.v_ram[(address - V_RAM_ADDR) as usize] = value
            },
            E_RAM_BANK_ADDR..W_RAM_BANK_0_ADDR => {
                self.mbc.write_ram(address - E_RAM_BANK_ADDR, value)
            },
            W_RAM_BANK_0_ADDR..ECHO_RAM_ADDR => {
                self.w_ram[(address - W_RAM_BANK_0_ADDR) as usize] = value
            },
            ECHO_RAM_ADDR..OAM_ADDR => {
                self.write_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR, value)
            },
            OAM_ADDR..UNUSABLE_ADDR => unimplemented!("OAM not implemented!"),
            UNUSABLE_ADDR..IO_REGISTERS_ADDR => {},
            IO_REGISTERS_ADDR..H_RAM_ADDR => self.write_io_register(address, value),
            H_RAM_ADDR..IE_REGISTER_ADDR => self.h_ram[(address - H_RAM_ADDR) as usize] = value,
            IE_REGISTER_ADDR => self.io.interrupt_enable = value,
        }
    }
}
