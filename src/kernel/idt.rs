// Interrupt Discriptor Table

use crate::util::{address_hi_16_bytes, address_lo_16_bytes};

const KERNEL_CS: u16 = 0x08;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTGate {
    lo_offset: u16, // Lo bits of handler function
    sel: u16,       // Kernel segment selector
    always_0: u8,
    /* First byte
     * Bit 7: "Interrupt is present"
     * Bits 6-5: Privilege level of caller (0=kernel..3=user)
     * Bit 4: Set to 0 for interrupt gates
     * Bits 3-0: bits 1110 = decimal 14 = "32 bit interrupt gate"
     */
    flags: u8,
    hi_offset: u16, // Hi bits of handler function
}

impl IDTGate {
    pub const fn new() -> Self {
        Self {
            lo_offset: 0,
            sel: KERNEL_CS,
            always_0: 0,
            flags: 0x8E,
            hi_offset: 0,
        }
    }

    pub fn set(&mut self, handler: unsafe extern "C" fn()) {
        let handler_addr = handler as usize;
        self.hi_offset = address_hi_16_bytes(handler_addr);
        self.lo_offset = address_lo_16_bytes(handler_addr);
    }
}

// The IDT register must be 6 bytes in length.
#[repr(C, packed)] 
pub struct IDTReg {
    pub limit: u16,
    pub base: *const IDTGate, // assumed to be 4 bytes.
}

impl IDTReg {
    pub const fn null() -> Self {
        Self {
            limit: 0,
            base: core::ptr::null(),
        }
    }
}
