use crate::platform::i386::paging::page_table::PageTable;

pub const PAGE_DIR_ENTRIES: usize = 1024;
pub const BIG_PAGE_SIZE: usize = PAGE_FRAME_SIZE * PAGE_DIR_ENTRIES;
pub const PAGE_FRAME_SIZE: usize = 4096;
pub const PAGE_TABLE_ENTRIES: usize = 1024;
pub const PAGE_TABLE_SIZE: usize = core::mem::size_of::<PageTable>();
