use core::ptr;

use crate::{
    kernel::ports::{read_port_byte, write_port_byte},
    vga::{Port, VGA},
};

const VIDEO_MEM: *mut u8 = 0xb8000 as *mut u8;
pub const WIDTH: u16 = 80;
pub const HEIGHT: u16 = 25;
pub const SCREEN_SIZE: u16 = WIDTH * HEIGHT;
pub const CHAR_SIZE: u16 = 2;
pub const BUF_SIZE: u16 = CHAR_SIZE * SCREEN_SIZE;

pub struct VGAText {}

impl VGAText {
    pub unsafe fn put_char_raw(&mut self, c: u8, x: u16, y: u16) {
        unsafe {
            let char_addr: *mut u8 = VIDEO_MEM.add(2 * (y * WIDTH + x) as usize);
            let col_addr = char_addr.add(1);
            char_addr.write_unaligned(c);
            col_addr.write_unaligned(0x0f);
        }
    }

    pub fn get_cursor_position(&self) -> usize {
        write_port_byte(Port::VGA3Out as u16, VGA::CursorHiByte as u8);
        let mut offset = (read_port_byte(Port::VGA3In as u16) as usize) << 8;
        write_port_byte(Port::VGA3Out as u16, VGA::CursorLoByte as u8);
        offset += read_port_byte(Port::VGA3In as u16) as usize;
        offset * 2
    }

    pub fn update_cursor_position(&mut self, x: u16, y: u16) {
        let offset = y * WIDTH + x;
        let hi = offset >> 8;
        let lo = offset & 0x00ff;
        write_port_byte(Port::VGA3Out as u16, VGA::CursorLoByte as u8);
        write_port_byte(Port::VGA3In as u16, lo as u8);
        write_port_byte(Port::VGA3Out as u16, VGA::CursorHiByte as u8);
        write_port_byte(Port::VGA3In as u16, hi as u8);
    }

    pub unsafe fn clear_row(&mut self, idx: u16) {
        if idx >= HEIGHT {
            return;
        }
        let row_start = unsafe { VIDEO_MEM.add((CHAR_SIZE * WIDTH * idx) as usize) };
        for i in 0..WIDTH {
            unsafe {
                let addr = row_start.add((i * CHAR_SIZE) as usize);
                *addr = b' ';
            }
        }
    }

    pub fn copy_row(&mut self, src: u16, dst: u16) {
        if src >= HEIGHT || dst >= HEIGHT {
            return;
        }
        unsafe {
            let src_row = VIDEO_MEM.add((CHAR_SIZE * WIDTH * src) as usize);
            let dst_row = VIDEO_MEM.add((CHAR_SIZE * WIDTH * dst) as usize);
            ptr::copy_nonoverlapping(src_row, dst_row, (WIDTH * CHAR_SIZE) as usize);
        }
    }
}
