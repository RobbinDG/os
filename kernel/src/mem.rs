use crate::{
    platform::i386::paging::{constants::PAGE_FRAME_SIZE, mmu::MMU, page_table::PageTable},
    pre_boot::{MemSpec, read_mem_spec},
};

const FREE_TABLE_START_ADDR: *mut PageTable = (0x110000 as *mut PageTable).wrapping_add(1);
const FREE_MEM_START_ADDR: usize = 0x150000;
const PAGE_SIZE_MASK: usize = !(PAGE_FRAME_SIZE - 1);

/// The kernel component responsible for managing memory and allocation thereof.
/// Although the boilerplate for paging is implemented under the MMU, it does not
/// perform any checks on availability.
pub struct MemoryManager {
    mmu: MMU,
    free_mem_addr: usize,
    free_table_addr: *mut PageTable,
    mem_spec: MemSpec,
}

impl MemoryManager {
    pub unsafe fn init() -> Self {
        Self {
            mmu: unsafe { MMU::create(0x00100000 as *mut ()) },
            free_mem_addr: FREE_MEM_START_ADDR,
            free_table_addr: FREE_TABLE_START_ADDR,
            mem_spec: unsafe { read_mem_spec() },
        }
    }

    pub unsafe fn enable_paging(&mut self) {
        unsafe { self.mmu.enable() };

        unsafe { self.alloc_table() };
    }

    /// Maps a region of virtual address space and returns a pointer to it.
    /// All allocations are entirely page aligned; that is, every new allocation
    /// inhabits its own page frame. This is a quick implementation for simplicity,
    /// to be improved later.
    pub unsafe fn map(&mut self, size: usize, align: bool) -> *mut () {
        let phys = self.free_mem_addr as *mut ();
        self.free_mem_addr += PAGE_FRAME_SIZE;
        self.free_mem_addr &= PAGE_SIZE_MASK; // Should not be necessary
        unsafe { self.mmu.alloc_frame(1, phys) }
    }

    pub unsafe fn unmap(&mut self, virt_addr: *mut (), length: usize) {
        unsafe { self.mmu.unmap_table_entry(virt_addr) };
    }

    /// Allocate a new page table in the next free location in memory, and map
    /// it in the MMU (page directory).
    pub unsafe fn alloc_table(&mut self) {
        let ptr = self.free_table_addr;
        unsafe { *ptr = PageTable::new_emtpy() };
        self.free_table_addr = self.free_table_addr.wrapping_add(1);
        unsafe {
            self.mmu.map_table(&*ptr);
        }
    }

    pub unsafe fn get_memory(&mut self) -> MemSpec {
        self.mem_spec.clone()
    }
}
