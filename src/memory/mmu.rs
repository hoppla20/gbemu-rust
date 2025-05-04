use tracing::debug;

use crate::serial::Serial;

use super::mbc::Mbc;

use super::{
    E_RAM_BANK_ADDR, ECHO_RAM_ADDR, H_RAM_ADDR, H_RAM_SIZE, IE_REGISTER_ADDR, IO_REGISTERS_ADDR,
    IoRegisters, OAM_ADDR, UNUSABLE_ADDR, V_RAM_ADDR, W_RAM_BANK_0_ADDR, W_RAM_BANK_SIZE,
};
use crate::graphics::GraphicsState;

static CYCLES_PER_CLOCK_LOOKUP: [u16; 4] = [256, 4, 16, 64];

pub struct Mmu {
    oam_transfer: bool,

    mbc: Box<dyn Mbc + 'static>,
    w_ram: Vec<u8>,
    h_ram: [u8; H_RAM_SIZE],

    pub io: IoRegisters,
    pub graphics: GraphicsState,
    pub serial: Box<dyn Serial>,
}

impl Mmu {
    pub fn new(mbc: Box<dyn Mbc + 'static>, serial: Box<dyn Serial>) -> Self {
        Mmu {
            oam_transfer: false,

            mbc,
            w_ram: vec![0; W_RAM_BANK_SIZE * 2],
            h_ram: [0; H_RAM_SIZE],

            io: IoRegisters::default(),
            graphics: GraphicsState::default(),
            serial,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address < V_RAM_ADDR {
            // rom

            return self.mbc.read_rom(address);
        }

        if address < E_RAM_BANK_ADDR {
            // vram

            return self.graphics.v_ram[(address - V_RAM_ADDR) as usize];
        }

        if address < W_RAM_BANK_0_ADDR {
            // eram

            return self.mbc.read_ram(address - E_RAM_BANK_ADDR);
        }

        if address < ECHO_RAM_ADDR {
            // wram

            return self.w_ram[(address - W_RAM_BANK_0_ADDR) as usize];
        }

        if address < OAM_ADDR {
            // echo ram

            return self.read_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR);
        }

        if address < UNUSABLE_ADDR {
            // oam

            unimplemented!("OAM not implemented!");
        }

        if address < IO_REGISTERS_ADDR {
            // unusable

            if self.oam_transfer {
                return 0xFF;
            } else {
                return 0x00;
            }
        }

        if address < H_RAM_ADDR {
            // io registers

            match address {
                // serial
                0xFF01 => return self.serial.read(),
                0xFF02 => return self.io.serial_transfer_control,

                //timer
                0xFF04 => return self.io.timer.divider(),
                0xFF05 => return self.io.timer.counter,
                0xFF06 => return self.io.timer.modulo,
                0xFF07 => return self.io.timer.control,

                // interrupt
                0xFF0F => return self.io.interrupt_flags,

                // TODO: audio
                0xFF10..=0xFF26 => {
                    debug!(
                        name: "mmu::address::noimpl",
                        "Reading from not implemented audio register at address 0x{:02X}. Returning 0x00",
                        address
                    );
                    return 0x00;
                },

                // graphics
                0xFF40 => return self.graphics.registers.lcd_control,
                0xFF41 => return self.graphics.registers.lcd_status,
                0xFF42 => return self.graphics.registers.screen_y,
                0xFF43 => return self.graphics.registers.screen_x,
                0xFF44 => return self.graphics.registers.lcd_y,
                0xFF45 => return self.graphics.registers.lcd_y_compare,
                0xFF47 => return self.graphics.registers.background_palette,
                0xFF48 => return self.graphics.registers.obj_palette[0],
                0xFF49 => return self.graphics.registers.obj_palette[1],
                0xFF4A => return self.graphics.registers.window_y,
                0xFF4B => return self.graphics.registers.window_x,
                _ => todo!("Implement i/o register read at address 0x{:02X}", address),
            }
        }

        if address < IE_REGISTER_ADDR {
            // hram

            return self.h_ram[(address - H_RAM_ADDR) as usize];
        }

        if address == IE_REGISTER_ADDR {
            // interrupt enable register

            return self.io.interrupt_enable;
        }

        panic!(
            "Memory read from address 0x{:02X} is not implemented!",
            address
        );
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address < V_RAM_ADDR {
            // rom is read-only, do nothing

            return;
        }

        if address < E_RAM_BANK_ADDR {
            // vram

            self.graphics.v_ram[(address - V_RAM_ADDR) as usize] = value;
            return;
        }

        if address < W_RAM_BANK_0_ADDR {
            // eram

            self.mbc.write_ram(address - E_RAM_BANK_ADDR, value);
            return;
        }

        if address < ECHO_RAM_ADDR {
            // wram

            self.w_ram[(address - W_RAM_BANK_0_ADDR) as usize] = value;
            return;
        }

        if address < OAM_ADDR {
            // echo ram

            self.write_byte(address - ECHO_RAM_ADDR + W_RAM_BANK_0_ADDR, value);
            return;
        }

        if address < UNUSABLE_ADDR {
            // oam

            unimplemented!("OAM not implemented!");
        }

        if address < IO_REGISTERS_ADDR {
            // unusable

            return;
        }

        if address < H_RAM_ADDR {
            // io registers

            match address {
                // serial
                0xFF01 => {
                    self.serial.write(value as char);
                    return;
                },
                0xFF02 => {
                    self.serial.transfer(&mut self.io.serial_transfer_control);
                    return;
                },

                // timer
                0xFF04 => {
                    self.io.timer.reset_divider();
                    return;
                },
                0xFF05 => {
                    self.io.timer.write_counter(value);
                    return;
                },
                0xFF06 => {
                    self.io.timer.modulo = value;
                    return;
                },
                0xFF07 => {
                    self.io.timer.control = value;
                    return;
                },

                // interrupt
                0xFF0F => {
                    self.io.interrupt_flags = value;
                    return;
                },

                // TODO: audio
                0xFF10..=0xFF26 => {
                    debug!(
                        name: "mmu::address::noimpl",
                        "Writing to not implemented audio register 0x{:02X}",
                        address
                    );
                    return;
                },

                // graphics
                0xFF40 => {
                    self.graphics.registers.lcd_control = value;
                    return;
                },
                0xFF41 => {
                    self.graphics.registers.lcd_status = value;
                    return;
                },
                0xFF42 => {
                    self.graphics.registers.screen_y = value;
                    return;
                },
                0xFF43 => {
                    self.graphics.registers.screen_x = value;
                    return;
                },
                0xFF44 => {
                    self.graphics.registers.lcd_y = value;
                    return;
                },
                0xFF45 => {
                    self.graphics.registers.lcd_y_compare = value;
                    return;
                },
                0xFF47 => {
                    self.graphics.registers.background_palette = value;
                    return;
                },
                0xFF48 => {
                    self.graphics.registers.obj_palette[0] = value;
                    return;
                },
                0xFF49 => {
                    self.graphics.registers.obj_palette[1] = value;
                    return;
                },
                0xFF4A => {
                    self.graphics.registers.window_y = value;
                    return;
                },
                0xFF4B => {
                    self.graphics.registers.window_x = value;
                    return;
                },
                _ => todo!("Implement i/o register read from address 0x{:02X}", address),
            }
        }

        if address < IE_REGISTER_ADDR {
            // hram

            self.h_ram[(address - H_RAM_ADDR) as usize] = value;
            return;
        }

        if address == IE_REGISTER_ADDR {
            // interrupt enable register

            self.io.interrupt_enable = value;
            return;
        }

        panic!(
            "Memory write to address 0x{:02X} is not implemented!",
            address
        );
    }
}
