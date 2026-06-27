pub const MAX_HIGH_MEM_ENTRIES: usize = 15;

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct RawHighMemEntry {
    base: u64,
    len: u64,
    typ: u32,
}

#[derive(Clone, Copy)]
pub enum MemType {
    Usable,
    Reserved,
    ACPIReclaimable,
    ACPI,
    Bad,
    Error,
}

impl From<u32> for MemType {
    fn from(value: u32) -> Self {
        match value {
            1 => MemType::Usable,
            2 => MemType::Reserved,
            3 => MemType::ACPIReclaimable,
            4 => MemType::ACPI,
            5 => MemType::Bad,
            _ => MemType::Error,
        }
    }
}

impl Into<u8> for &MemType {
    fn into(self) -> u8 {
        match self {
            MemType::Usable => 1,
            MemType::Reserved => 2,
            MemType::ACPIReclaimable => 3,
            MemType::ACPI => 4,
            MemType::Bad => 5,
            MemType::Error => 0,
        }
    }
}


#[derive(Clone, Copy)]
pub struct HighMemEntry {
    pub base: u64,
    pub len: u64,
    pub typ: MemType,
}

impl From<RawHighMemEntry> for HighMemEntry {
    fn from(value: RawHighMemEntry) -> Self {
        Self {
            base: value.base,
            len: value.len,
            typ: value.typ.into()
        }
    }
}

#[derive(Clone)]
pub struct MemSpec {
    pub low_mem_size: u16,
    pub high_mem: [Option<HighMemEntry>; MAX_HIGH_MEM_ENTRIES],
}

/// Detects the low memory size and stores it in this object.
/// This implementation is *incredibly* hacky and does not account for errors.
/// Reason 1: The address is hard coded in boot_sect.asm so we know where to read the value
/// Reason 2: The BIOS call can only be made from real mode, so we get it
///     in the boot sector and put it away for a bit.
/// Reason 3: The address of the stored value is in the boot sector, so we are essentially
///     overwriting it if ever it would get filled up.
/// Reason 4: The interrupt 0x12 comes with an error bit on the carry flag. We don't store
///     it right now.
/// Reason 5: This doesn't really fix anything, if we want to expand, we need to do all this
///     trickery again.
///
/// The solution is veritual 16 bit mode. That way, we can read it during execution.
unsafe fn detect_low_mem() -> u16 {
    let addr = 508 as *const u16;
    unsafe { *addr }
}

unsafe fn detect_high_mem() -> [Option<HighMemEntry>; MAX_HIGH_MEM_ENTRIES] {
    let mut entries = [None; MAX_HIGH_MEM_ENTRIES];
    let addr_count = 0x500 as *const u8;
    let addr_entries = 0x510 as *const RawHighMemEntry;

    let num_entries = unsafe { *addr_count } as usize;
    for i in 0..MAX_HIGH_MEM_ENTRIES {
        if i >= num_entries {
            break;
        }
        let size = unsafe { *addr_count.add((1 + i) as usize) };
        if size != 20 {
            // This doesn't quite work yet unfortunately. The assumption is that it is always 20
            // break;
        }
        let entry = unsafe { *addr_entries.add(i as usize) };
        entries[i] = Some(HighMemEntry::from(entry))
    }
    entries
}

pub unsafe fn read_mem_spec() -> MemSpec {
    unsafe {
        MemSpec {
            low_mem_size: detect_low_mem(),
            high_mem: detect_high_mem(),
        }
    }
}
