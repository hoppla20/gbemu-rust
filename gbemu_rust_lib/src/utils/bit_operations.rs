macro_rules! extract_bits {
    ($value:ident : $t:ty, $i:literal, $j:literal) => {{
        // compile-time checks
        const _: () = assert!($i <= $j, "i must be less then or equal to j!");
        const _: () = assert!(
            $j < std::mem::size_of::<$t>() * 8,
            "j must be less than the size of the variables type!"
        );
        const _: () = assert!($i >= 0, "i must be greater than or equal to 0!");

        const BIT_MASK: $t = ((1 << ($j - $i + 1)) - 1) << $i;

        (($value) & BIT_MASK) >> $i
    }};
}

pub(crate) use extract_bits;

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_single_bit() {
        let value: u8 = 0b00001111;
        assert_eq!(extract_bits!(value: u8, 0, 0), 1);
        assert_eq!(extract_bits!(value: u8, 3, 3), 1);
        assert_eq!(extract_bits!(value: u8, 4, 4), 0);
        assert_eq!(extract_bits!(value: u8, 7, 7), 0);
    }

    #[test]
    fn test_extract_multiple_bits() {
        let value: u8 = 0b00001111;
        assert_eq!(extract_bits!(value: u8, 0, 3), 0b1111);
        assert_eq!(extract_bits!(value: u8, 4, 7), 0b0000);
        assert_eq!(extract_bits!(value: u8, 1, 4), 0b0111);
        assert_eq!(extract_bits!(value: u8, 2, 5), 0b0011);
    }
}
