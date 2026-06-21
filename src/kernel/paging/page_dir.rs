use crate::kernel::paging::page_table::PageTable;

pub const PAGE_DIR_ENTRIES: usize = 1024;

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
/// RSVD bit. Not used by CPU, should always be 0.
const F_RESERVED: u32 = 1 << 21;
const M_BIG_PAGE_HI: u32 = 0b00000000_00011111_11100000_00000000;
const M_BIG_PAGE_LO: u32 = 0b11111111_11000000_11110000_00000000;
const M_PAGE_TABLE: u32 = 0b11111111_11111111_11110000_00000000;

#[derive(Copy, Clone)]
pub struct RawPageDirEntry(u32);

impl RawPageDirEntry {
    pub const fn new_unused() -> Self {
        Self(0x0000)
    }

    /// Assigns this entry as a big page pointing to the given address.
    /// This overwrites all relevant settings, as we cannot be sure about
    /// the state that the entry is in without evaluating all fields.
    ///
    /// This method **does not** validate the 4MiB alignment of the input address.
    ///
    /// Requires PSE to be enabled.
    pub fn new_page_entry(&mut self, page_addr: *const u8) {
        // Only the upper 20 bits are used. The lower should all be 0 as a page
        // table address is aligned at 4KiB.
        let addr = page_addr as u32 & M_PAGE_TABLE;
        self.0 |= F_PRESENT | F_READ_WRITE | F_PAGE_SIZE;
        self.0 &= !(F_AVAILABLE | F_ACCESSED);
        self.0 = (self.0 & !M_PAGE_TABLE) | addr;
    }

    /// Assigns this entry as a page table pointing to the given address.
    /// This overwrites all relevant settings, as we cannot be sure about
    /// the state that the entry is in without evaluating all fields.
    ///
    /// This method **does not** validate the 4KiB alignment of the input address.
    pub fn new_table_entry(&mut self, table: &PageTable) {
        // Only the upper 20 bits are used. The lower should all be 0 as a page
        // table address is aligned at 4KiB.
        let addr = table as *const PageTable as u32 & M_PAGE_TABLE;
        self.0 |= F_PRESENT | F_READ_WRITE;
        self.0 &= !(F_PAGE_SIZE | F_AVAILABLE | F_ACCESSED);
        self.0 = (self.0 & !M_PAGE_TABLE) | addr;
    }

    pub fn free(&mut self) {
        self.0 &= !F_PRESENT;
    }

    pub unsafe fn get_table(&self) -> &PageTable {
        unsafe { &*((self.0 & M_PAGE_TABLE) as *const PageTable) }
    }

    pub unsafe fn get_table_mut(&mut self) -> &mut PageTable {
        unsafe { &mut *((self.0 & M_PAGE_TABLE) as *mut PageTable) }
    }
}

#[repr(align(4096))]
pub struct PageDir([RawPageDirEntry; PAGE_DIR_ENTRIES]);

impl PageDir {
    pub const fn make_empty() -> Self {
        Self([RawPageDirEntry::new_unused(); PAGE_DIR_ENTRIES])
    }

    pub unsafe fn identity_map_first_mb(&mut self) {
        // Create new page table that includes the identity map.
        let first_table_ptr = 0x00110000 as *mut PageTable;
        unsafe { *first_table_ptr = PageTable::new_emtpy() };

        // Add the table to the first entry in the directory
        let first_table = unsafe { &mut *first_table_ptr };
        let pd_entry = &mut self.0[0];
        pd_entry.new_table_entry(first_table);

        // Create linear map
        let table = unsafe { pd_entry.get_table_mut() };
        table.linear_map(0 as *const (), 0x00100000 as *const ());
    }

    pub unsafe fn get_entry(&self, index: usize) -> &RawPageDirEntry {
        &self.0[index]
    }
}
