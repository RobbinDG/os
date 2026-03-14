use core::arch::asm;

use once_cell_no_std::OnceCell;

use crate::{
    kernel::{
        isr::set_isr,
        keyboard_driver::KeyboardDriver,
        mem::MemoryManager,
        platform::i386::{
            gdt::{CompiledGDTEntry, GDTEntry, GDTR},
            tss::TSS,
        },
        syscalls::SysCalls,
        vga_driver::VGAText,
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
    /// The GDT needs a static memory location native to the kernel, so
    /// we store it as a field. Note that the entries need to be in compiled form.
    gdt: [CompiledGDTEntry; 6],
    tss: TSS,
    mem: spin::Mutex<MemoryManager>,
    keyboard_driver: spin::Mutex<KeyboardDriver>,
    vga_driver: spin::Mutex<VGAText>,
    syscalls: SysCalls,
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

            // All state ready.
            let mut kernel = Self {
                gdt: Default::default(),
                tss: TSS::new(),
                mem: spin::Mutex::new(mem),
                keyboard_driver: spin::Mutex::new(keyboard_drv),
                vga_driver: spin::Mutex::new(vga_drv),
                syscalls: SysCalls::new(),
            };

            kernel.tss.init(0x90000, 0x10);
            kernel.load_tss();
            Ok(kernel)
        }
    }

    pub fn memory_manager(&self) -> &spin::Mutex<MemoryManager> {
        &self.mem
    }

    pub fn vga_driver(&self) -> &spin::Mutex<VGAText> {
        &self.vga_driver
    }

    pub fn keyboard_driver(&self) -> &spin::Mutex<KeyboardDriver> {
        &self.keyboard_driver
    }

    unsafe fn load_tss(&mut self) {
        unsafe {
            let tr_idx = self.append_tr_to_gdt() as u16;
            asm!(
                "ltr {o:x}", // `ltr` takes a 16-bit register.
                "str ax",
                o = in(reg) tr_idx,
            )
        }
    }

    #[inline]
    unsafe fn append_tr_to_gdt(&mut self) -> usize {
        unsafe { asm!("cli") }
        let mut gdtr = GDTR::default();
        gdtr.load();

        if let Some(e) = gdtr.entry_raw(1) {
            self.gdt[1] = e.clone();
        }
        if let Some(e) = gdtr.entry_raw(2) {
            self.gdt[2] = e.clone();
        }

        self.gdt[3] = GDTEntry::new(true).encode();
        self.gdt[4] = GDTEntry::new(false).encode();

        self.gdt[5] = GDTEntry::for_task_state_segment(&self.tss).encode();
        let new_gdtr = GDTR::for_gdt(&self.gdt);
        new_gdtr.store();
        unsafe { asm!("sti") }
        5 * core::mem::size_of::<CompiledGDTEntry>()
    }
}
