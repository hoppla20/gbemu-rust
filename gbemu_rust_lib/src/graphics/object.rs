use crate::utils::bit_operations::bit;

#[derive(Default, Debug, Clone, Copy)]
pub struct SpriteFlags {
    pub background_priority: bool,
    pub flip_y: bool,
    pub flip_x: bool,
    pub palette: bool,
}

impl From<u8> for SpriteFlags {
    fn from(value: u8) -> Self {
        Self {
            background_priority: bit!(value: u8, 7),
            flip_y: bit!(value: u8, 6),
            flip_x: bit!(value: u8, 5),
            palette: bit!(value: u8, 4),
        }
    }
}

impl From<SpriteFlags> for u8 {
    fn from(value: SpriteFlags) -> Self {
        let mut result = 0b1111_0000;
        result |= if value.background_priority { 1 << 7 } else { 0 };
        result |= if value.flip_y { 1 << 6 } else { 0 };
        result |= if value.flip_x { 1 << 5 } else { 0 };
        result |= if value.palette { 1 << 4 } else { 0 };
        result
    }
}

pub struct Object {
    pub pos_x: u8,
    pub pos_y: u8,
    pub tile_number: u8,
    pub sprite_flags: SpriteFlags,
}
