use crate::{
    KERNEL,
    decimal_printable::{DecimalDigits, DecimalPrintable},
    dyn_array::DynArray,
    hex_printable::HexPrintable,
    kernel::vga_driver::{HEIGHT, WIDTH},
};

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
pub struct Console {
    x: u16,
    y: u16,
}

impl Console {
    /// Creates a new instance using a driver. The state is not synchronized and
    /// will overwrite existing text when needed.
    pub unsafe fn create() -> Self {
        unsafe { Self { x: X, y: Y } }
    }

    /// Obtains an instance of the TTY, if one has not been used yet.
    /// The returned Option acts as a non-blocking lock, returning `None`
    /// when an instance is already in use.
    pub unsafe fn get_instance() -> Option<Self> {
        unsafe { if ACTIVE { None } else { Some(Self::create()) } }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe {
            X = self.x;
            Y = self.y;
            ACTIVE = false;
        }
    }
}

impl Console {
    pub unsafe fn clear(&mut self) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                for i in 0..HEIGHT {
                    vga.clear_row(i);
                }
                self.x = 0;
                self.y = 0;
                vga.update_cursor_position(self.x, self.y);
            })
        };
    }

    pub unsafe fn put_char(&mut self, c: u8) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                vga.put_char_raw(c, self.x, self.y);
                self.move_cursor(1, 0);
                vga.update_cursor_position(self.x, self.y);
            });
        }
    }

    pub unsafe fn scroll(&mut self, columns: u16) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                for i in 0..HEIGHT {
                    if i + columns < HEIGHT {
                        vga.copy_row(i + columns, i);
                    } else {
                        vga.clear_row(i);
                    }
                }
            });
        }
    }

    pub unsafe fn bs(&mut self) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                vga.put_char_raw(b' ', self.x - 1, self.y);
                self.move_cursor(-1, 0);
                vga.update_cursor_position(self.x, self.y);
            });
        }
    }

    pub unsafe fn print_ascii(&mut self, s: &[u8]) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                for c in s {
                    if *c == 0x00 {
                        break;
                    }
                    vga.put_char_raw(*c, self.x, self.y);
                    self.move_cursor(1, 0);
                }
                vga.update_cursor_position(self.x, self.y);
            });
        }
    }

    pub fn nl(&mut self) {
        unsafe {
            self.move_cursor(0, 1);
        }
        self.x = 0;
        unsafe {
            KERNEL
                .vga_driver
                .with_unwrap(|vga| vga.update_cursor_position(self.x, self.y))
        };
    }

    pub unsafe fn println_ascii(&mut self, s: &[u8]) {
        unsafe {
            self.print_ascii(s);
            self.nl();
        }
    }

    unsafe fn print_char_buffer<'b>(&mut self, buf: DynArray<u8>) {
        unsafe {
            KERNEL.vga_driver.with_unwrap(|vga| {
                for i in 0..buf.len() {
                    match buf.get(i) {
                        Ok(0) => break,
                        Ok(c) => {
                            vga.put_char_raw(*c, self.x, self.y);
                            self.move_cursor(1, 0);
                        }
                        Err(_) => break,
                    }
                }
                vga.update_cursor_position(self.x, self.y);
            });
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
