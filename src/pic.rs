use crate::ports::{Port, io_wait, write_port_byte};

const EOI: u8 = 0x20;
const CASCADE_IRQ: u8 = 2;

enum ICW1 {
    ICW4 = 0x01,
    Single = 0x02,
    Interval4 = 0x04,
    Level = 0x08,
    Init = 0x10,
}

enum ICW4 {
    Mode8086 = 0x01,
    Auto = 0x02,
    BufSlave = 0x04,
    BufMaster = 0x08,
    SFNM = 0x10,
}

/// Driver for the Programmable Interrupt Controller
pub struct PIC {}

impl PIC {
    /// Send an End Of Interrupt (EOI) for a given IRQ to the PIC.
    pub unsafe fn send_eoi(irq: u8) {
        unsafe {
            if irq >= 8 {
                write_port_byte(Port::SlavePICCommand.into(), EOI);
            }
            write_port_byte(Port::MasterPICCommand.into(), EOI);
        }
    }

    /// Initialises and remaps the PIC's IRQs. This function sends the initialisation command 
    /// and the following 3 initialisation words.
    pub unsafe fn remap(offset1: u8, offset2: u8) {
        unsafe {
            // Init command
            write_port_byte(Port::MasterPICCommand.into(), ICW1::Init as u8 | ICW1::ICW4 as u8);
            io_wait();
            write_port_byte(Port::SlavePICCommand.into(), ICW1::Init as u8 | ICW1::ICW4 as u8);
            io_wait();

            // Init word 1
            write_port_byte(Port::MasterPICData.into(), offset1);
            io_wait();
            write_port_byte(Port::SlavePICData.into(), offset2);
            io_wait();

            // Init word 2: 
            // Master: there is a slave at IRQ 2
            // Slave: set cascade identity
            write_port_byte(Port::MasterPICData.into(), 1 << CASCADE_IRQ);
            io_wait();
            write_port_byte(Port::SlavePICData.into(), 2);
            io_wait();

            // Init word 3: use 8086 mode over 8080 mode.
            write_port_byte(Port::MasterPICData.into(), ICW4::Mode8086 as u8);
            io_wait();
            write_port_byte(Port::SlavePICData.into(), ICW4::Mode8086 as u8);
            io_wait();

            // Unmask
            write_port_byte(Port::MasterPICData.into(), 0);
            write_port_byte(Port::SlavePICData.into(), 0);
        }
    }
}
