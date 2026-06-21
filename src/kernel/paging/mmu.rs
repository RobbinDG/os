use core::{arch::asm, hint};

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
            self.page_dir_mut().identity_map_first_mb();
            hint::black_box(self.get_phys(0x39ce as *const ()));
            hint::black_box(self.get_phys(0x10004123 as *const ()));
            asm!(
                "mov cr3, {pd}", // Set page directory in CR3
                "mov eax, cr0",
                "or eax, 0x80000001", // Set PG and PE of CR0
                "mov cr0, eax",
                pd = in(reg) self.pd_ptr,
            );
        }
    }

    #[inline(always)]
    unsafe fn page_dir_mut(&self) -> &mut PageDir {
        unsafe { &mut *self.pd_ptr }
    }

    /// Get the physical address corresponding to a virtual address.
    /// Note that this is unsafe after paging is loaded, as the page
    /// table being referenced may not exist.
    #[inline(never)]
    pub unsafe fn get_phys(&self, virt: *const ()) -> *const () {
        let virt = virt as u32;
        let pd_idx = virt >> 22;
        let pt_idx = (virt >> 12) & 0x3FF;
        let pde = unsafe { (&mut *self.pd_ptr).get_entry(pd_idx as usize) };
        let pt = unsafe { pde.get_table() };
        let pte = unsafe { pt.get_entry(pt_idx as usize) };
        unsafe { (pte.base() as *const u8).add((virt & 0xFFF) as usize) as *const () }
    }
}
