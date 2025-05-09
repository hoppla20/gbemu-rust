#[derive(Default, Debug, Clone, Copy)]
pub struct SpriteFlags {
    pub background_priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette: bool,
}

pub struct Object {}
