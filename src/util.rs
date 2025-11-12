#[inline]
pub const fn lo_byte(two_byte: u16) -> u8 {
    (two_byte & 0x00FF) as u8
}

#[inline]
pub const fn hi_byte(two_byte: u16) -> u8 {
    (two_byte >> 8) as u8
}

#[inline]
pub const fn address_lo_16_bytes(addr: usize) -> u16 {
    (addr & 0x0000_FFFF) as u16
}

#[inline]
pub const fn address_hi_16_bytes(addr: usize) -> u16 {
    (addr >> 16) as u16
}

#[inline]
pub const fn read_bit_mask(b: u8, mask: u8) -> bool {
    ((b & mask) > 0) as bool
}