use core::cmp::min;

use crate::kernel::platform::i386::paging::constants::{PAGE_FRAME_SIZE, PAGE_TABLE_ENTRIES};

const F_PRESENT: u32 = 1 << 0;
const F_READ_WRITE: u32 = 1 << 1;
/// U/S bit. Set -> All access, Unset -> only "supervisor"
const F_USER_SUPER: u32 = 1 << 2;
const F_ACCESSED: u32 = 1 << 5;
/// D bit. Only used for big pages.
const F_DIRTY: u32 = 1 << 6;
/// AVL bit. Only used for page tables, not used by the CPU.
const F_AVAILABLE: u32 = 1 << 6;
/// PS bit. Set -> Big Page, Unset -> Page Table
const F_PAGE_SIZE: u32 = 1 << 7;

const M_PAGE_BASE: u32 = 0b11111111_11111111_11110000_00000000;

#[derive(Clone, Copy)]
pub struct RawPageTableEntry(u32);

impl RawPageTableEntry {
    pub const fn new_unused() -> Self {
        Self(0x0000)
    }

    #[inline(always)]
    pub fn base(&self) -> *const () {
        (self.0 & M_PAGE_BASE) as *const ()
    }

    #[inline(always)]
    pub fn present(&self) -> bool {
        (self.0 & F_PRESENT) > 0
    }

    pub fn map_to(&mut self, addr: *const ()) {
        self.0 = (addr as u32) & M_PAGE_BASE;
        // TODO make this not accessible by ring 3 / user mode by default.
        self.0 |= F_PRESENT | F_READ_WRITE | F_USER_SUPER;
    }

    pub fn unmap(&mut self) {
        self.0 &= !F_PRESENT
    }
}

#[repr(align(4096))]
pub struct PageTable([RawPageTableEntry; PAGE_TABLE_ENTRIES]);

impl PageTable {
    pub const fn new_emtpy() -> Self {
        Self([RawPageTableEntry::new_unused(); PAGE_TABLE_ENTRIES])
    }

    pub unsafe fn get_entry(&self, index: usize) -> &RawPageTableEntry {
        &self.0[index]
    }

    pub unsafe fn get_entry_mut(&mut self, index: usize) -> &mut RawPageTableEntry {
        &mut self.0[index]
    }

    pub fn linear_map(&mut self, start: *const (), end: *const ()) {
        let m = (end as usize - start as usize) / PAGE_FRAME_SIZE;
        let start = start as *const u8;
        for i in 0..min(m, PAGE_TABLE_ENTRIES) {
            let x: &mut RawPageTableEntry = &mut self.0[i];
            x.map_to(unsafe { start.add(4096 * i) as *const () });
        }
    }
}
