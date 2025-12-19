use once_cell_no_std::OnceCell;

use crate::kernel::{mem::MemoryManager, pre_boot::read_mem_spec};

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
        let kernel = unsafe { Kernel::new() };
        if let Err(_) = self.inner.set(kernel) {
            loop {}
        }
    }

    pub fn get(&self) -> Result<&Kernel, KernelError> {
        self.inner.get().ok_or(KernelError::NotReady)
    }
}

pub struct Kernel {
    mem: spin::Mutex<MemoryManager>,
}

impl Kernel {
    pub unsafe fn new() -> Self {
        Self {
            mem: spin::Mutex::new(unsafe { MemoryManager::init() }),
        }
    }

    pub fn memory_manager(&self) -> &spin::Mutex<MemoryManager> {
        &self.mem
    }
}
