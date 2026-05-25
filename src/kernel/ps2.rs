use crate::{
    kernel::ports::{Port, read_port_byte, write_port_byte},
    util::read_bit_mask,
};

const OUTPUT_BUFFER_STATUS: u8 = 1 << 0;
const INPUT_BUFFER_STATUS: u8 = 1 << 1;
const SYSTEM_FLAG: u8 = 1 << 2; // QEMU never sets this! A check will fail.
const COMMAND_OR_DATA: u8 = 1 << 3;
const TIME_OUT_ERROR: u8 = 1 << 6;
const PARITY_ERROR: u8 = 1 << 7;

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

const PS2_CONTROLLER_DATA_PORT: u16 = 0x60;
const PS2_CONTROLLER_STATUS_REGISTER: u16 = 0x64;
const PS2_CONTROLLER_COMMAND_REGISTER: u16 = 0x64;

/// PS/2 Controller Command bytes, derived from:
/// https://wiki.osdev.org/I8042_PS/2_Controller#PS/2_Controller_Commands
const PS2_CTRL_CMD_READ_CONFIG: u8 = 0x20;
const PS2_CTRL_CMD_WRITE_CONFIG: u8 = 0x60;
const PS2_CTRL_CMD_TEST: u8 = 0xAA;
const PS2_CTRL_CMD_DISABLE_PORT_1: u8 = 0xAD;
const PS2_CTRL_CMD_ENABLE_PORT_1: u8 = 0xAE;
const PS2_CTRL_CMD_DISABLE_PORT_2: u8 = 0xA7;
const PS2_CTRL_CMD_ENABLE_PORT_2: u8 = 0xA8;
const PS2_CTRL_CMD_TEST_PORT_1: u8 = 0xAB;
const PS2_CTRL_CMD_TEST_PORT_2: u8 = 0xA9;
// Device commands
const PS2_DEV_CMD_IDENTIFY: u8 = 0xF2;
const PS2_DEV_CMD_ENABLE_SCAN: u8 = 0xF4;
const PS2_DEV_CMD_DISABLE_SCAN: u8 = 0xF5;
const PS2_DEV_CMD_ACK: u8 = 0xFA;
const PS2_DEV_CMD_RESET: u8 = 0xFF;

// PS/2 Controller Command output reference.
const CONTROLLER_TEST_PASSED: u8 = 0x55;
const CONTROLLER_TEST_FAILED: u8 = 0xFC;

const PS2_CTRL_CFG_INTERRUPT_1: u8 = 1 << 0;
const PS2_CTRL_CFG_INTERRUPT_2: u8 = 1 << 1;
const PS2_CTRL_CFG_SYSFLAG: u8 = 1 << 2;
const PS2_CTRL_CFG_ZERO_LO: u8 = 1 << 3;
const PS2_CTRL_CFG_PORT_CLOCK_1_DISABLED: u8 = 1 << 4;
const PS2_CTRL_CFG_PORT_CLOCK_2_DISABLED: u8 = 1 << 5;
const PS2_CTRL_CFG_PORT_TRANSLATE_1: u8 = 1 << 6;
const PS2_CTRL_CFG_ZERO_HI: u8 = 1 << 7;

enum PS2DeviceCommand {
    Identify = 0xF2,
    EnableScanning = 0xF4,
    DisableScanning = 0xF5,
}

/*
pub fn tmp() {
    let status = unsafe { read_status() };
    unsafe {
        if let Some(mut tty) = VGATextWriter::get_instance() {
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
*/

unsafe fn read_status() -> Result<PS2Status, PS2Error> {
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

#[inline(never)] // TMP DEBUG
pub fn init_ps2() {
    // TODO Init USB
    // TODO Check PS/2 exists.
    // disable data during initialisation.
    ps2_send_command_no_response(PS2_CTRL_CMD_DISABLE_PORT_1);
    ps2_send_command_no_response(PS2_CTRL_CMD_DISABLE_PORT_2);
    // flush output buffer
    let _ = read_port_byte(PS2_CONTROLLER_DATA_PORT);
    // set configuration byte
    let mut config = ps2_send_command(PS2_CTRL_CMD_READ_CONFIG);
    config = config & 0b0111_0111; // Set 0 bytes
    config = config | 0b0000_0001; // Enable interrupts
    ps2_send_command_with_data(PS2_CTRL_CMD_WRITE_CONFIG, config);
    // perform self test
    while ps2_send_command(PS2_CTRL_CMD_TEST) != CONTROLLER_TEST_PASSED {}
    // check if there are 2 channels
    ps2_send_command_no_response(PS2_CTRL_CMD_ENABLE_PORT_2);
    let has_second_channel =
        (ps2_send_command(PS2_CTRL_CMD_READ_CONFIG) & PS2_CTRL_CFG_PORT_CLOCK_2_DISABLED) == 0;
    ps2_send_command_no_response(PS2_CTRL_CMD_DISABLE_PORT_2);
    // perform interface tests
    if ps2_send_command(PS2_CTRL_CMD_TEST_PORT_1) > 0 {
        // Port 1 test failed
        loop {}
    }
    if has_second_channel {
        if ps2_send_command(PS2_CTRL_CMD_TEST_PORT_2) > 0 {
            // Port 2 test failed
            loop {}
        }
    }
    // enable devices
    ps2_send_command_no_response(PS2_CTRL_CMD_ENABLE_PORT_1);
    ps2_send_command_no_response(PS2_CTRL_CMD_ENABLE_PORT_2);
    // reset devices
    ps2_send_data(PS2_DEV_CMD_RESET);
    let b1 = ps2_read_response_data(); // Should be 0xFA
    let b2 = ps2_read_response_data(); // Should be 0xAA
    let b3 = ps2_read_response_data_or_timeout(); // Should be device ID byte 1
    let b4 = ps2_read_response_data_or_timeout(); // Should be device ID byte 2
    if b1 != 0xFA || b2 != 0xAA {
        // No acknowledgement of reset from device
        loop {}
    }
}

pub enum KeyboardError {
    DeviceTimeout,
    NoDisableAck,
    NoEnableAck,
    NoIdentiyAck,
    Unknown,
}

/// Send a command to the current PS/2 device and check for
/// acknowledgment. Return whether the device ACK'ed or not.
fn send_device_command(dev_cmd_byte: u8) -> Option<bool> {
    ps2_send_data(dev_cmd_byte);
    ps2_read_response_data_or_timeout().map(|resp| resp == PS2_DEV_CMD_ACK)
}

fn send_and_check_device_command(dev_cmd_byte: u8, err: KeyboardError) -> Result<(), KeyboardError> {
    let success =
        send_device_command(dev_cmd_byte).ok_or(KeyboardError::DeviceTimeout)?;
    if !success{
        return Err(err);
    }
    Ok(())
}

fn identify_devices_raw() -> Result<(u8, u8), KeyboardError> {
    // Send disable scanning command (0xF5) to device
    send_and_check_device_command(PS2_DEV_CMD_DISABLE_SCAN, KeyboardError::NoDisableAck)?;

    // Send identify command (0xF2)
    send_and_check_device_command(PS2_DEV_CMD_IDENTIFY, KeyboardError::NoIdentiyAck)?;

    // Wait for reply and/or timeout
    let b1 = ps2_read_response_data_or_timeout();
    let b2 = ps2_read_response_data_or_timeout();
    Ok((b1.unwrap_or(0xFF), b2.unwrap_or(0xFF)))
}

#[inline(never)]
pub fn identify_devices() -> Result<(u8, u8), KeyboardError> {
    let res = identify_devices_raw();
    // Send enable scanning command (0xF4)
    send_and_check_device_command(PS2_DEV_CMD_ENABLE_SCAN, KeyboardError::NoEnableAck)?;
    res
}

fn ps2_send_data(data_byte: u8) {
    while (read_port_byte(PS2_CONTROLLER_STATUS_REGISTER) & INPUT_BUFFER_STATUS) != 0 {}
    write_port_byte(PS2_CONTROLLER_DATA_PORT, data_byte);
}

fn ps2_send_command_no_response(cmd_byte: u8) {
    // Wait for status register to indicate that input buffer is clear
    while (read_port_byte(PS2_CONTROLLER_STATUS_REGISTER) & INPUT_BUFFER_STATUS) != 0 {}
    write_port_byte(PS2_CONTROLLER_COMMAND_REGISTER, cmd_byte);
}

fn ps2_read_response_data() -> u8 {
    // Wait for status register to indicate that input is available.
    while (read_port_byte(PS2_CONTROLLER_STATUS_REGISTER) & OUTPUT_BUFFER_STATUS) == 0 {}
    read_port_byte(PS2_CONTROLLER_DATA_PORT)
}

fn ps2_read_response_data_or_timeout() -> Option<u8> {
    // Wait for status register to indicate that input is available.
    let mut attempts = 256;
    while (read_port_byte(PS2_CONTROLLER_STATUS_REGISTER) & OUTPUT_BUFFER_STATUS) == 0 {
        attempts -= 1;
        if (attempts == 0) {
            return None;
        }
    }
    Some(read_port_byte(PS2_CONTROLLER_DATA_PORT))
}

fn ps2_send_command(cmd_byte: u8) -> u8 {
    ps2_send_command_no_response(cmd_byte);
    ps2_read_response_data()
}

fn ps2_send_command_with_data(cmd_byte: u8, data: u8) {
    ps2_send_command_no_response(cmd_byte);
    ps2_send_data(data);
}
