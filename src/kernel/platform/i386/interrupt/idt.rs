use core::arch::asm;

use crate::{
    kernel::platform::i386::interrupt::{isr::*, pic::PIC},
    util::{address_hi_16_bytes, address_lo_16_bytes},
};

const NUM_IDT_GATES: usize = 256;
const KERNEL_CS: u16 = 0x08;

type IDTGates = [IDTGate; NUM_IDT_GATES];

static mut IDT_REG: IDTReg = IDTReg::null();
static mut IDT: IDTGates = {
    let emtpy_gate = IDTGate::new();
    [emtpy_gate; NUM_IDT_GATES]
};

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTGate {
    /// Lo bits of handler function
    lo_offset: u16, 
    /// Kernel segment selector
    sel: u16,       
    always_0: u8,
    /// First byte
    /// Bit 7: "Interrupt is present"
    /// Bits 6-5: Privilege level of caller (0=kernel..3=user)
    /// Bit 4: Set to 0 for interrupt gates
    /// Bits 3-0: bits 1110 = decimal 14 = "32 bit interrupt gate"
    flags: u8,
    /// Hi bits of handler function
    hi_offset: u16,
}

impl IDTGate {
    pub const fn new() -> Self {
        Self {
            lo_offset: 0,
            sel: KERNEL_CS,
            always_0: 0,
            flags: 0b1 << 7 // P: Always present
            | 3 << 5 // DPL: Can be called from ring 3 and below
            | 0 << 4 // 0
            | 0xE, // Gate Type: Interrupt gate
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

pub unsafe fn setup_idt() {
    unsafe {
        IDT[0].set(isr0);
        IDT[1].set(isr1);
        IDT[2].set(isr2);
        IDT[3].set(isr3);
        IDT[4].set(isr4);
        IDT[5].set(isr5);
        IDT[6].set(isr6);
        IDT[7].set(isr7);
        IDT[8].set(isr8);
        IDT[9].set(isr9);
        IDT[10].set(isr10);
        IDT[11].set(isr11);
        IDT[12].set(isr12);
        IDT[13].set(isr13);
        IDT[14].set(isr14);
        IDT[15].set(isr15);
        IDT[16].set(isr16);
        IDT[17].set(isr17);
        IDT[18].set(isr18);
        IDT[19].set(isr19);
        IDT[20].set(isr20);
        IDT[21].set(isr21);
        IDT[22].set(isr22);
        IDT[23].set(isr23);
        IDT[24].set(isr24);
        IDT[25].set(isr25);
        IDT[26].set(isr26);
        IDT[27].set(isr27);
        IDT[28].set(isr28);
        IDT[29].set(isr29);
        IDT[30].set(isr30);
        IDT[31].set(isr31);

        PIC::remap(32, 40);

        IDT[32].set(irq0);
        IDT[33].set(irq1);
        IDT[34].set(irq2);
        IDT[35].set(irq3);
        IDT[36].set(irq4);
        IDT[37].set(irq5);
        IDT[38].set(irq6);
        IDT[39].set(irq7);
        IDT[40].set(irq8);
        IDT[41].set(irq9);
        IDT[42].set(irq10);
        IDT[43].set(irq11);
        IDT[44].set(irq12);
        IDT[45].set(irq13);
        IDT[46].set(irq14);
        IDT[47].set(irq15);

        IDT[128].set(isr_syscall);

        IDT_REG.base = &IDT[0];
        IDT_REG.limit = (core::mem::size_of::<IDTGates>() - 1) as u16;
        let idt_reg_ptr: *const u16 = &raw const IDT_REG.limit;
        asm!(
            "lidt [{0}]",
            in(reg) idt_reg_ptr,
            options(nostack, preserves_flags)
        );
    }
}
