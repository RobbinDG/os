use core::arch::asm;

use crate::{
    kernel::{
        global::{Global, GlobalLazy},
        keyboard_driver::KeyboardDriver,
        mem::MemoryManager,
        platform::i386::{
            gdt::{CompiledGDTEntry, GDTEntry, GDTR}, interrupt::idt::setup_idt, tss::TSS
        },
        syscalls::SysCalls,
        vga_driver::VGAText,
    },
    printer::VGATextWriter,
};

const GDT_SIZE: usize = 6;

pub enum KernelError {
    NotReady,
    OutOfBounds,
    Busy,
}

pub struct Kernel {
    //--- GLOBAL STATE ---
    /// The GDT needs a static memory location native to the kernel, so
    /// we store it as a field. Note that the entries need to be in compiled form.
    gdt: Global<[CompiledGDTEntry; GDT_SIZE]>,
    pub tss: Global<TSS>,
    //--- SERVICES ---
    pub mem: GlobalLazy<MemoryManager>,
    pub keyboard_driver: GlobalLazy<KeyboardDriver>,
    pub vga_driver: GlobalLazy<VGAText>,
    syscalls: SysCalls,
}

impl Kernel {
    pub const fn new() -> Self {
        Self {
            gdt: Global::new([[0; 8]; GDT_SIZE]),
            tss: Global::new(TSS::new()),
            mem: GlobalLazy::empty(),
            keyboard_driver: GlobalLazy::empty(),
            vga_driver: GlobalLazy::empty(),
            syscalls: SysCalls {},
        }
    }

    pub unsafe fn init(&self) -> Result<(), ()> {
        unsafe {
            // Setup interrupt handling
            setup_idt();

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
            self.mem.init(mem);
            self.keyboard_driver.init(keyboard_drv);
            self.vga_driver.init(vga_drv);

            self.tss.with(|tss| tss.init(0x90000, 0x10));
            self.load_tss();
            Ok(())
        }
    }

    unsafe fn load_tss(&self) {
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
    unsafe fn append_tr_to_gdt(&self) -> usize {
        unsafe { asm!("cli") }
        let mut gdtr = GDTR::default();
        gdtr.load();

        let tss_ptr = unsafe { self.tss.ptr() };

        unsafe {
            self.gdt.with(|gdt| {
                if let Some(e) = gdtr.entry_raw(1) {
                    gdt[1] = e.clone();
                }
                if let Some(e) = gdtr.entry_raw(2) {
                    gdt[2] = e.clone();
                }

                gdt[3] = GDTEntry::new(true).encode();
                gdt[4] = GDTEntry::new(false).encode();

                gdt[5] = GDTEntry::for_task_state_segment(&*tss_ptr).encode();
            })
        };
        let new_gdtr = GDTR::for_gdt(unsafe { &*self.gdt.ptr() });
        new_gdtr.store();
        unsafe { asm!("sti") }
        5 * core::mem::size_of::<CompiledGDTEntry>()
    }
}
