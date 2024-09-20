
pub mod binary_helpers {
    pub fn get_upper_byte(num: u16) -> u8 {
        (num >> 8) as u8
    }

    pub fn get_lower_byte(num: u16) -> u8 {
        (num & 0x00ff) as u8
    }

    pub fn split_u16(num: u16) -> (u8, u8) { // has structure (upper, lower), let (h, l) = split_u16(num);
        (get_upper_byte(num), get_lower_byte(num))
    }

    pub fn combine_bytes(h: u8, l: u8) -> u16 {
        ((h as u16) << 8) | (l as u16)
    }
}
