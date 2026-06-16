use core::arch::asm;

use crate::kernel::paging::{
    page_dir::{PAGE_DIR_ENTRIES, PageDir},
    page_table::PAGE_FRAME_SIZE,
};

const BIG_PAGE_SIZE: usize = PAGE_FRAME_SIZE * PAGE_DIR_ENTRIES;
const PAGE_MASK: u32 = PAGE_FRAME_SIZE as u32 - 1;

pub struct MMU {
    mem_start: *mut u8,
    pd_ptr: *mut PageDir,
}

impl MMU {
    /// Creates a new MMU controller and initialises a new, empty
    /// page directory. Paging is **not** enabled after creating.
    pub unsafe fn create(mem_start: *mut u8) -> MMU {
        if mem_start as u32 & PAGE_MASK > 0 {
            panic!()
        }
        let pd_ptr = mem_start as *mut PageDir;
        unsafe { *pd_ptr = PageDir::make_empty() };
        Self { mem_start, pd_ptr }
    }

    pub unsafe fn enable(&self) {
        unsafe {
            asm!(
                "mov cr3, {pd}", // Set page directory in CR3
                "mov eax, cr0",
                "or eax, 0x80000001", // Set PG and PE of CR0
                "mov cr0, eax",
                pd = in(reg) self.pd_ptr,
            );
            self.page_dir_mut().identity_map_first_mb();
        }
    }

    #[inline(always)]
    unsafe fn page_dir_mut(&self) -> &mut PageDir {
        unsafe { &mut *self.pd_ptr }
    }
}
