use core::arch::asm;

use crate::kernel::isr::{clear_last_interrupt, last_interrupt};

pub enum Port {
    MBHexDisplay = 0x0080, // Unused after POST, used for IO waiting.

    // PIC
    MasterPICCommand = 0x0020,
    MasterPICData = 0x0021,
    SlavePICCommand = 0x00A0,
    SlavePICData = 0x00A1,

    // PS2
    PS2DataPort = 0x0060,
    PS2StatusCmdReg = 0x0064,
}

impl Into<u16> for Port {
    fn into(self) -> u16 {
        self as u16
    }
}

pub fn write_port_byte(port: u16, data: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") data,
            options(nomem, nostack, preserves_flags),
        )
    }
}

pub fn read_port_byte(port: u16) -> u8 {
    unsafe {
        let mut al: u8;
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") al,
            options(nomem, nostack, preserves_flags),
        );
        al
    }
}

pub fn write_port_word(port: u16, data: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") data,
            options(nomem, nostack, preserves_flags),
        )
    }
}

pub fn read_port_word(port: u16) -> u16 {
    unsafe {
        let mut ax: u16;
        asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") ax,
            options(nomem, nostack, preserves_flags),
        );
        ax
    }
}

pub fn io_wait() {
    write_port_byte(Port::MBHexDisplay as u16, 0)
}

pub unsafe fn kernel_write_port_byte(port: u16, data: u8) -> Result<(), u32> {
    unsafe {
        clear_last_interrupt();
        write_port_byte(port, data);
        let int = last_interrupt();
        if int > 0 {
            return Err(int);
        }
        Ok(())
    }
}
