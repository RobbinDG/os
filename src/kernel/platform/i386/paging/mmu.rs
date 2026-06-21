use core::arch::asm;

use crate::kernel::platform::i386::paging::{
    constants::{PAGE_DIR_ENTRIES, PAGE_FRAME_SIZE, PAGE_TABLE_ENTRIES},
    page_dir::PageDir,
    page_table::PageTable,
};

const PAGE_MASK: u32 = PAGE_FRAME_SIZE as u32 - 1;
pub struct MMU {
    mem_start: *mut (),
    pd_ptr: *mut PageDir,
}

impl MMU {
    /// Creates a new MMU controller and initialises a new, empty
    /// page directory. Paging is **not** enabled after creating.
    pub unsafe fn create(mem_start: *mut ()) -> MMU {
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
            asm!(
                "mov cr3, {pd}", // Set page directory in CR3
                "mov eax, cr0",
                "or eax, 0x80000001", // Set PG and PE of CR0
                "mov cr0, eax",
                pd = in(reg) self.pd_ptr,
            );
        }
    }

    /// Allocate a new page table.
    pub unsafe fn map_table(&mut self, table: &PageTable) -> usize {
        let pd = unsafe { self.page_dir_mut() };
        for i in 1..PAGE_DIR_ENTRIES {
            let pde = unsafe { pd.get_entry_mut(i) };
            if !pde.present() {
                pde.new_table_entry(table);
                return i;
            }
        }
        panic!()
    }

    /// Unmaps a page table entry based on the virtual address.
    /// After calling this function, the entire memory page is unmapped,
    /// so any virtual addresses with the same bits 31-12 will be invalid.
    pub unsafe fn unmap_table_entry(&mut self, virt: *const ()) {
        let (pd_idx, pt_idx) = Self::decompose_virt(virt as u32);
        let pde = unsafe { self.page_dir_mut().get_entry_mut(pd_idx as usize) };
        let pt = unsafe { pde.get_table_mut() };
        let pte = unsafe { pt.get_entry_mut(pt_idx as usize) };
        pte.unmap();
    }

    pub unsafe fn alloc_frame(&mut self, table: usize, phys_addr: *mut ()) -> *mut () {
        let pde = unsafe { self.page_dir_mut().get_entry_mut(table) };
        let pt = unsafe { pde.get_table_mut() };
        for i in 0..PAGE_TABLE_ENTRIES {
            let entry = unsafe { pt.get_entry_mut(i) };
            if !entry.present() {
                entry.map_to(phys_addr);
                return Self::make_virt(table as u32, i as u32, 0);
            }
        }
        panic!()
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
        let (pd_idx, pt_idx) = Self::decompose_virt(virt);
        let pde = unsafe { (&mut *self.pd_ptr).get_entry(pd_idx as usize) };
        let pt = unsafe { pde.get_table() };
        let pte = unsafe { pt.get_entry(pt_idx as usize) };
        unsafe { (pte.base() as *const u8).add((virt & 0xFFF) as usize) as *const () }
    }

    #[inline(always)]
    pub fn decompose_virt(virt: u32) -> (u32, u32) {
        let pd_idx = virt >> 22;
        let pt_idx = (virt >> 12) & 0x3FF;
        (pd_idx, pt_idx)
    }

    #[inline(always)]
    fn make_virt(pd_idx: u32, pt_idx: u32, frame_addr: u32) -> *mut () {
        ((pd_idx << 22) | (pt_idx << 12) | (frame_addr & 0xFFF)) as *mut ()
    }
}
