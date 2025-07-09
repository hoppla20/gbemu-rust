const SELECT_BUTTONS_BIT: usize = 5;
const SELECT_DIRECTIONS_BIT: usize = 4;

pub enum Key {
    Down,
    Up,
    Left,
    Right,
    B,
    A,
    Select,
    Start,
}

#[derive(Default, Clone, Copy)]
pub struct JoypadRegister {
    interrupt: bool,

    pub select_buttons: bool,
    pub select_directions: bool,

    start_pressed: bool,
    select_pressed: bool,
    a_pressed: bool,
    b_pressed: bool,

    down_pressed: bool,
    up_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

impl From<JoypadRegister> for u8 {
    fn from(value: JoypadRegister) -> Self {
        let mut result = 0b1100_0000;

        result += if value.select_buttons {
            0
        } else {
            1 << SELECT_BUTTONS_BIT
        };
        result += if value.select_directions {
            0
        } else {
            1 << SELECT_DIRECTIONS_BIT
        };

        result += if (value.select_buttons && value.start_pressed)
            || (value.select_directions && value.down_pressed)
        {
            0
        } else {
            1 << 3
        };
        result += if (value.select_buttons && value.select_pressed)
            || (value.select_directions && value.up_pressed)
        {
            0
        } else {
            1 << 2
        };
        result += if (value.select_buttons && value.b_pressed)
            || (value.select_directions && value.left_pressed)
        {
            0
        } else {
            1 << 1
        };
        result += if (value.select_buttons && value.a_pressed)
            || (value.select_directions && value.right_pressed)
        {
            0
        } else {
            1
        };

        result
    }
}

impl JoypadRegister {
    pub fn write(&mut self, value: u8) {
        self.select_buttons = ((value >> SELECT_BUTTONS_BIT) & 0x01) == 0;
        self.select_directions = ((value >> SELECT_DIRECTIONS_BIT) & 0x01) == 0;
    }

    pub fn key_event(&mut self, key: Key, pressed: bool) {
        self.interrupt = true;
        match key {
            Key::Down => self.down_pressed = pressed,
            Key::Up => self.up_pressed = pressed,
            Key::Left => self.left_pressed = pressed,
            Key::Right => self.right_pressed = pressed,
            Key::B => self.b_pressed = pressed,
            Key::A => self.a_pressed = pressed,
            Key::Select => self.select_pressed = pressed,
            Key::Start => self.start_pressed = pressed,
        }
    }

    pub fn interrupt(&mut self) -> bool {
        let result = self.interrupt;
        self.interrupt = false;
        result
    }
}
