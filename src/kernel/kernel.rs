use once_cell_no_std::OnceCell;

use crate::kernel::mem::MemoryManager;

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
    pub fn init(&self, low_mem: u16) {
        let kernel = Kernel::new();
        kernel.memory_manager().lock().set_low_mem(low_mem);
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
    pub fn new() -> Self {
        Self {
            mem: spin::Mutex::new(MemoryManager::new()),
        }
    }

    pub fn memory_manager(&self) -> &spin::Mutex<MemoryManager> {
        &self.mem
    }
}
