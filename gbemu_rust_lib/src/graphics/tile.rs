use std::array::from_fn;

use super::registers::LcdControlFlags;

#[derive(Debug, PartialEq, Eq)]
pub enum Pixel {
    Color0,
    Color1,
    Color2,
    Color3,
}

impl From<u8> for Pixel {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Color0,
            1 => Self::Color1,
            2 => Self::Color2,
            3 => Self::Color3,
            _ => panic!("Unknown Color {}", value),
        }
    }
}

#[derive(Default, Debug)]
pub struct TileRow {
    pub bytes: [u8; 2],
}

impl TileRow {
    pub fn get_pixel(&self, index: u8) -> Pixel {
        let bit = 7 - index;
        let bit1 = self.bytes[1] >> bit & 1;
        let bit2 = self.bytes[0] >> bit & 1;
        ((bit1 << 1) + bit2).into()
    }
}

#[derive(Default, Debug)]
pub struct Tile {
    pub rows: [TileRow; 8],
}

#[derive(Default)]
pub struct TileMap {
    tiles: [[u8; 32]; 32],
}

pub struct TileData {
    tiles: [Tile; 3 * 128],
}

impl Default for TileData {
    fn default() -> Self {
        Self {
            tiles: from_fn(|_| Tile::default()),
        }
    }
}

impl TileData {
    pub fn get_tile(&self, lcdc: LcdControlFlags, tile_number: u8) -> &Tile {
        if lcdc.tile_data_select {
            // 8000 method
            &self.tiles[tile_number as usize]
        } else {
            // 8800 method
            &self.tiles[(256 + ((tile_number as i8) as i16)) as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pixel_from_row() {
        let mut row = TileRow::default();

        row.bytes[0] = 0xA5;
        row.bytes[1] = 0xC3;

        assert_eq!(row.get_pixel(0), Pixel::Color3);
        assert_eq!(row.get_pixel(1), Pixel::Color2);
        assert_eq!(row.get_pixel(2), Pixel::Color1);
        assert_eq!(row.get_pixel(3), Pixel::Color0);
        assert_eq!(row.get_pixel(4), Pixel::Color0);
        assert_eq!(row.get_pixel(5), Pixel::Color1);
        assert_eq!(row.get_pixel(6), Pixel::Color2);
        assert_eq!(row.get_pixel(7), Pixel::Color3);
    }

    #[test]
    fn test_get_tile() {
        let tiles = TileData::default();

        // 8000 method
        let lcdc = (!0).into();
        // check for poiner (in-)equality
        assert_ne!(
            &tiles.tiles[0] as *const _,
            tiles.get_tile(lcdc, 1) as *const _
        );
        assert_eq!(
            &tiles.tiles[0] as *const _,
            tiles.get_tile(lcdc, 0) as *const _
        );
        assert_eq!(
            &tiles.tiles[1] as *const _,
            tiles.get_tile(lcdc, 1) as *const _
        );

        // 8800 method
        let lcdc = 0.into();
        assert_eq!(
            &tiles.tiles[256] as *const _,
            tiles.get_tile(lcdc, 0) as *const _
        );
        assert_eq!(
            &tiles.tiles[257] as *const _,
            tiles.get_tile(lcdc, 1) as *const _
        );
        assert_eq!(
            &tiles.tiles[255] as *const _,
            tiles.get_tile(lcdc, 255) as *const _
        );
    }
}
