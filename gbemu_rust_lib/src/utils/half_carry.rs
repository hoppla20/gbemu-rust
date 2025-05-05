pub fn half_carry_add_r8(a: u8, b: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F)) > 0x0F
}

pub fn half_carry_add_r8_3(a: u8, b: u8, c: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F) + (c & 0x0F)) > 0x0F
}

pub fn half_carry_sub_r8(a: u8, b: u8) -> bool {
    ((a & 0x0F) as i8) - ((b & 0x0F) as i8) < 0
}

pub fn half_carry_sub_r8_3(a: u8, b: u8, c: u8) -> bool {
    ((a & 0x0F) as i8) - ((b & 0x0F) as i8) - ((c & 0x0F) as i8) < 0
}

pub fn half_carry_add_r16(a: u16, b: u16) -> bool {
    ((a & 0x00FF) + (b & 0x00FF)) > 0x00FF
}
