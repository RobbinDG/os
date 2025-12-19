use crate::{
    decimal_printable::{DecimalDigits, DecimalPrintable},
    dyn_array::DynArray,
    hex_printable::HexPrintable,
    ports::{read_port_byte, write_port_byte},
    vga::{Port, VGA},
};

const VIDEO_MEM: *mut u8 = 0xb8000 as *mut u8;
const WIDTH: u16 = 80;
const HEIGHT: u16 = 25;
static mut X: u16 = 0;
static mut Y: u16 = 0;
static mut ACTIVE: bool = false;

/// Offers TTY printing to console when in console mode.
///
/// Under the hood, a global state is maintained that
/// contains the cursor position. When an instance is created,
/// it copies this state, storing it back when dropped. This circumvents
/// problems with mutably borrowing static mutables and follows borrowing
/// rules.
pub struct VGAText {
    x: u16,
    y: u16,
}

impl VGAText {
    /// Obtains an instance of the TTY, if one has not been used yet.
    /// The returned Option acts as a non-blocking lock, returning `None`
    /// when an instance is already in use.
    pub unsafe fn get_instance() -> Option<Self> {
        unsafe {
            if ACTIVE {
                None
            } else {
                ACTIVE = true;
                Some(Self { x: X, y: Y })
            }
        }
    }
}

impl Drop for VGAText {
    fn drop(&mut self) {
        unsafe {
            X = self.x;
            Y = self.y;
            ACTIVE = false;
        }
    }
}

impl VGAText {
    pub unsafe fn clear(&mut self) {
        let screen_size = WIDTH * HEIGHT;
        for i in 0..screen_size {
            unsafe {
                let addr = VIDEO_MEM.add(i as usize * 2);
                *addr = b' ';
            }
        }
        self.x = 0;
        self.y = 0;
        self.update_cursor_position();
    }

    pub fn get_cursor_position(&self) -> usize {
        write_port_byte(Port::VGA3Out as u16, VGA::CursorHiByte as u8);
        let mut offset = (read_port_byte(Port::VGA3In as u16) as usize) << 8;
        write_port_byte(Port::VGA3Out as u16, VGA::CursorLoByte as u8);
        offset += read_port_byte(Port::VGA3In as u16) as usize;
        offset * 2
    }

    pub fn update_cursor_position(&mut self) {
        let offset = self.y * WIDTH + self.x;
        let hi = offset >> 8;
        let lo = offset & 0x00ff;
        write_port_byte(Port::VGA3Out as u16, VGA::CursorLoByte as u8);
        write_port_byte(Port::VGA3In as u16, lo as u8);
        write_port_byte(Port::VGA3Out as u16, VGA::CursorHiByte as u8);
        write_port_byte(Port::VGA3In as u16, hi as u8);
    }

    pub unsafe fn put_char(&mut self, c: u8) {
        unsafe {
            Self::put_char_raw(c, self.x, self.y);
            self.move_cursor(1, 0);
        }
        self.update_cursor_position();
    }

    pub unsafe fn scroll(&mut self, columns: u16) {
        let row_size_bytes = 2 * WIDTH as usize;
        let offset = row_size_bytes * columns as usize;
        let mut dest_row = VIDEO_MEM;
        let clear_start = HEIGHT.wrapping_add(columns);
        unsafe {
            for i in 0..HEIGHT {
                if i < clear_start {
                    for j in 0..row_size_bytes {
                        let dest = dest_row.add(j);
                        let src = dest_row.add(j + offset);
                        *dest = *src;
                    }
                } else {
                    for j in 0..row_size_bytes {
                        let dest = dest_row.add(j);
                        *dest = b' ';
                    }
                }
                dest_row = dest_row.add(row_size_bytes);
            }
        }
    }

    pub unsafe fn bs(&mut self) {
        unsafe {
            Self::put_char_raw(b' ', self.x, self.y);
            self.move_cursor(1, 0);
        }
    }

    pub unsafe fn print_ascii(&mut self, s: &[u8]) {
        for c in s {
            unsafe {
                if *c == 0x00 {
                    break;
                }
                Self::put_char_raw(*c, self.x, self.y);
                self.move_cursor(1, 0);
            }
        }
        self.update_cursor_position();
    }

    pub fn nl(&mut self) {
        unsafe {
            self.move_cursor(0, 1);
        }
        self.x = 0;
        self.update_cursor_position();
    }

    pub unsafe fn println_ascii(&mut self, s: &[u8]) {
        unsafe {
            self.print_ascii(s);
            self.nl();
        }
    }

    unsafe fn print_char_buffer<'a>(&mut self, buf: DynArray<'a, u8>) {
        unsafe {
            for i in 0..buf.len() {
                match buf.get(i) {
                    Ok(0) => break,
                    Ok(c) => {
                        Self::put_char_raw(*c, self.x, self.y);
                        self.move_cursor(1, 0);
                    }
                    Err(_) => break,
                }
            }
            self.update_cursor_position();
        }
    }

    pub unsafe fn print_decimal<T: DecimalPrintable + DecimalDigits>(&mut self, n: T) {
        unsafe {
            match n.as_decimal() {
                Ok(dec) => self.print_char_buffer(dec),
                Err(_) => return,
            }
        }
    }

    pub unsafe fn print_hex<T: HexPrintable>(&mut self, n: T) {
        unsafe {
            match n.as_hex() {
                Ok(hex) => self.print_char_buffer(hex),
                Err(_) => return,
            }
        }
    }

    unsafe fn put_char_raw(c: u8, x: u16, y: u16) {
        unsafe {
            let char_addr: *mut u8 = VIDEO_MEM.add(2 * (y * WIDTH + x) as usize);
            let col_addr = char_addr.add(1);
            char_addr.write_unaligned(c);
            col_addr.write_unaligned(0x0f);
        }
    }

    #[inline]
    unsafe fn move_cursor(&mut self, dx: i16, dy: i16) {
        let x_acc = self.x.wrapping_add_signed(dx);
        self.x = x_acc % WIDTH;
        let mut new_y = self.y.wrapping_add_signed(dy) + x_acc / WIDTH;
        if new_y >= HEIGHT {
            let diff = new_y.wrapping_sub(HEIGHT - 1);
            unsafe { self.scroll(diff) };
            new_y = new_y.wrapping_sub(diff);
        }
        self.y = new_y;
    }
}
