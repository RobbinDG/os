use core::arch::asm;

pub enum Port {
    MBHexDisplay = 0x0080, // Unused after POST, used for IO waiting.

    // PIC
    MasterPICCommand = 0x0020,
    MasterPICData = 0x0021,
    SlavePICCommand = 0x00A0,
    SlavePICData = 0x00A1,
}

impl Into<u16> for Port {
    fn into(self) -> u16 {
        self as u16 
    }
}

pub unsafe fn write_port_byte(port: u16, data: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") data,
        )
    }
}

pub unsafe fn read_port_byte(port: u16) -> u8 {
    unsafe {
        let mut al: u8;
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") al,
        );
        al
    }
}

pub unsafe fn write_port_word(port: u16, data: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") data,
        )
    }
}

pub unsafe fn read_port_word(port: u16) -> u16 {
    unsafe {
        let mut ax: u16;
        asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") ax,
        );
        ax
    }
}

pub unsafe fn io_wait() {
    unsafe {
        write_port_byte(Port::MBHexDisplay.into(), 0);
    }
}