use crate::{
    ports::{Port, kernel_write_port_byte, read_port_byte, write_port_byte},
    printer::TTY,
    util::read_bit_mask,
};

const OUTPUT_BUFFER_STATUS: u8 = 1 << 0;
const INPUT_BUFFER_STATUS: u8 = 1 << 0;
const SYSTEM_FLAG: u8 = 1 << 0;
const COMMAND_OR_DATA: u8 = 1 << 0;
const TIME_OUT_ERROR: u8 = 1 << 0;
const PARITY_ERROR: u8 = 1 << 0;

struct PS2Status {
    output_buf_full: bool,
    input_buf_full: bool,
    sys_flag: bool,
    data_for_device: bool,
}

enum PS2Error {
    TimeOut,
    Parity,
}

enum PS2DeviceCommand {
    Identify = 0xF2,
    EnableScanning = 0xF4,
    DisableScanning = 0xF5,
}

pub fn tmp() {
    let status = unsafe { read_status() };
    unsafe {
        if let Some(mut tty) = TTY::get_instance() {
            match status {
                Ok(s) => {
                    if s.output_buf_full {
                        tty.println_ascii("Output buf".as_bytes());
                    }
                    if s.input_buf_full {
                        tty.println_ascii("Input buf".as_bytes());
                    }
                    if s.sys_flag {
                        tty.println_ascii("Sysflag".as_bytes());
                    }
                    if s.data_for_device {
                        tty.println_ascii("Device data".as_bytes());
                    }
                    tty.println_ascii("done".as_bytes());
                }
                Err(PS2Error::TimeOut) => tty.println_ascii("Timeout".as_bytes()),
                Err(PS2Error::Parity) => tty.println_ascii("Parity".as_bytes()),
            }
        }
    }
}

unsafe fn read_status() -> Result<PS2Status, PS2Error> {
    unsafe {
        let status_reg = read_port_byte(Port::PS2StatusCmdReg as u16);
        if read_bit_mask(status_reg, TIME_OUT_ERROR) {
            return Err(PS2Error::TimeOut);
        }
        if read_bit_mask(status_reg, PARITY_ERROR) {
            return Err(PS2Error::Parity);
        }
        Ok(PS2Status {
            output_buf_full: read_bit_mask(status_reg, OUTPUT_BUFFER_STATUS),
            input_buf_full: read_bit_mask(status_reg, INPUT_BUFFER_STATUS),
            sys_flag: read_bit_mask(status_reg, SYSTEM_FLAG),
            data_for_device: read_bit_mask(status_reg, COMMAND_OR_DATA),
        })
    }
}

pub fn init_ps2() {
    // TODO Init USB
    // TODO Check PS/2 exists.
    // TODO disable data during initialisation.
    // TODO flush output buffer
    // TODO set configuration byte
    // TODO perform self test
    // TODO check if there are 2 channels
    // TODO perform interface tests
    // TODO enable devices
    // TODO reset devices
}

pub unsafe fn identity_devices() -> Result<(u8, u8), ()> {
    unsafe {
        if let Some(mut tty) = TTY::get_instance() {
            // TODO send disable scanning command (0xF5) to device
            if let Err(c) = kernel_write_port_byte(
                Port::PS2DataPort as u16,
                PS2DeviceCommand::DisableScanning as u8,
            ) {
                tty.print_ascii("err: ".as_bytes());
                tty.print_hex(c as u16);
            }
            // TODO wait for ACK (0xFA)
            tty.print_ascii("ACK: ".as_bytes());
            tty.print_hex(read_port_byte(Port::PS2DataPort as u16) as u16);

            // TODO send identify command (0xF2)
            write_port_byte(Port::PS2DataPort as u16, PS2DeviceCommand::Identify as u8);
            // TODO wait for ACK (0xFA)
            tty.print_ascii("ACK: ".as_bytes());
            tty.print_hex(read_port_byte(Port::PS2DataPort as u16) as u16);
            // TODO wait for reply and/or timeout
            let b1 = read_port_byte(Port::PS2DataPort as u16);
            tty.print_ascii("B1: ".as_bytes());
            tty.print_hex(b1 as u16);
            let b2 = read_port_byte(Port::PS2DataPort as u16);
            tty.print_ascii("B2 ".as_bytes());
            tty.print_hex(b2 as u16);
            // TODO send enable scanning command (0xF4)
            write_port_byte(
                Port::PS2DataPort as u16,
                PS2DeviceCommand::EnableScanning as u8,
            );
            return Ok((b1, b2));
        }
        Err(())
    }
}
