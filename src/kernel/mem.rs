use crate::kernel::pre_boot::{MemSpec, read_mem_spec};

const FREE_MEM_START_ADDR: usize = 0x10000;
const PAGE_SIZE: usize = 0x1000;
const PAGE_SIZE_MASK: usize = !(PAGE_SIZE - 1);

pub struct MemoryManager {
    free_mem_addr: usize,
    mem_spec: MemSpec,
}

impl MemoryManager {
    pub unsafe fn init() -> Self {
        Self {
            free_mem_addr: FREE_MEM_START_ADDR,
            mem_spec: unsafe { read_mem_spec() },
        }
    }

    pub unsafe fn malloc(&mut self, size: usize, align: bool) -> *mut u8 {
        if align && (self.free_mem_addr & !PAGE_SIZE_MASK) > 0 {
            self.free_mem_addr &= PAGE_SIZE_MASK;
            self.free_mem_addr += PAGE_SIZE;
        }

        let phys_addr = self.free_mem_addr as *mut u8;
        self.free_mem_addr += size;
        phys_addr
    }

    pub unsafe fn free(&mut self, addr: *mut u8) {}

    pub unsafe fn get_memory(&mut self) -> MemSpec {
        self.mem_spec.clone()
    }
}
