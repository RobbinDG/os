#[inline]
pub const fn lo_byte(two_byte: u16) -> u8 {
    (two_byte & 0x00FF) as u8
}

#[inline]
pub const fn hi_byte(two_byte: u16) -> u8 {
    (two_byte >> 8) as u8
}

pub const fn address_lo_16_bytes(addr: usize) -> u16 {
    (addr & 0x0000_FFFF) as u16
}

pub const fn address_hi_16_bytes(addr: usize) -> u16 {
    (addr >> 16) as u16
}