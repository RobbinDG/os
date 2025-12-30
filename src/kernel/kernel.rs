use core::arch::asm;

use once_cell_no_std::OnceCell;

use crate::{
    kernel::{
        isr::set_isr, keyboard_driver::KeyboardDriver, mem::MemoryManager,
        process_manager::ProcessManager, vga_driver::VGAText,
    },
    printer::VGATextWriter,
};

pub enum KernelError {
    NotReady,
    OutOfBounds,
    Busy,
}

pub struct KernelAcc {
    inner: OnceCell<Kernel>,
}

impl KernelAcc {
    pub const fn new() -> Self {
        Self {
            inner: OnceCell::new(),
        }
    }

    /// Initialise the kernel. If, somehow, this fails, we loop forever so
    /// it can be easily debugged by GDB.
    pub unsafe fn init(&self) {
        if let Ok(kernel) = unsafe { Kernel::new() } {
            if let Err(_) = self.inner.set(kernel) {
                loop {}
            }
            return;
        }
        loop {}
    }

    pub fn get(&self) -> Result<&Kernel, KernelError> {
        self.inner.get().ok_or(KernelError::NotReady)
    }
}

pub struct Kernel {
    mem: spin::Mutex<MemoryManager>,
    pm: spin::Mutex<ProcessManager>,
    keyboard_driver: spin::Mutex<KeyboardDriver>,
    vga_driver: spin::Mutex<VGAText>,
}

impl Kernel {
    pub unsafe fn new() -> Result<Self, ()> {
        unsafe {
            // Setup interrupt handling
            set_isr();

            // Create kernel components
            let mem = MemoryManager::init();

            // Initialise drivers
            let mut vga_drv = VGAText {};
            let mut tty = match VGATextWriter::get_instance(&mut vga_drv) {
                Some(tty) => tty,
                None => return Err(()),
            };

            tty.clear();

            let keyboard_drv = match KeyboardDriver::initialise() {
                Ok(drv) => drv,
                Err(_) => {
                    tty.println_ascii("Couldn't load keyboard driver.".as_bytes());
                    loop {}
                }
            };
            asm!("sti"); // Sets the enable interrupt flag.

            // Cleanup used references to drivers.
            // This is done to avoid adding more nesting to this process.
            drop(tty);

            // Done
            Ok(Self {
                mem: spin::Mutex::new(mem),
                pm: spin::Mutex::new(ProcessManager {}),
                keyboard_driver: spin::Mutex::new(keyboard_drv),
                vga_driver: spin::Mutex::new(vga_drv),
            })
        }
    }

    pub fn memory_manager(&self) -> &spin::Mutex<MemoryManager> {
        &self.mem
    }

    pub fn process_manager(&self) -> &spin::Mutex<ProcessManager> {
        &self.pm
    }

    pub fn vga_driver(&self) -> &spin::Mutex<VGAText> {
        &self.vga_driver
    }

    pub fn keyboard_driver(&self) -> &spin::Mutex<KeyboardDriver> {
        &self.keyboard_driver
    }
}
