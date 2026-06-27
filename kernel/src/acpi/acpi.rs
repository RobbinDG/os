use crate::kernel::acpi::iter::SDTIterator;

const SIGNATURE: [u8; 8] = [b'R', b'S', b'D', b' ', b'P', b'T', b'R', b' '];
const EBDA_ADDR: *const u8 = 0x80000 as *const u8;

#[repr(C, packed)]
pub struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
}

#[repr(C, packed)]
pub struct ACPISDTHeader {
    pub signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

pub struct ACPI {
    // Fixed pointer to the RSDP
    rsdp_ptr: *const RSDP,
    // Remaining SDT pointers are part of the RSDT. We calculate the amount up-front.
    pub(crate) sdt_ptrs: *const *const ACPISDTHeader,
    pub(crate) sdt_cnt: usize,
}

impl ACPI {
    pub unsafe fn load() -> Option<Self> {
        unsafe {
            // unsafe Option<>::or_else does not exist apparently :(
            let mut rsdp_ptr = Self::find_rsdp_in_ebda();
            if let None = rsdp_ptr {
                rsdp_ptr = Self::find_rsdp_in_bios_area();
            }
            let rsdp_ptr = rsdp_ptr?;
            let rsdt_ptr = (*rsdp_ptr).rsdt_addr as *const ACPISDTHeader;
            let sdt_cnt = ((*rsdt_ptr).length as usize - core::mem::size_of::<ACPISDTHeader>())
                / core::mem::size_of::<*const ACPISDTHeader>();
            let sdt_ptrs = (rsdt_ptr as *const u8)
                .add(core::mem::size_of::<ACPISDTHeader>())
                .cast();

            Some(Self {
                rsdp_ptr,
                sdt_ptrs,
                sdt_cnt,
            })
        }
    }

    unsafe fn find_rsdp_in_ebda() -> Option<*const RSDP> {
        unsafe {
            for offset in (0x0..0x400).step_by(0x10) {
                let mut found = true;
                for i in 0..8 {
                    let addr = EBDA_ADDR.add(offset + i);
                    if *addr != SIGNATURE[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    return Some(EBDA_ADDR.add(offset).cast());
                }
            }
            None
        }
    }

    unsafe fn find_rsdp_in_bios_area() -> Option<*const RSDP> {
        unsafe {
            for base_addr in (0xE0000..0xFFFFF).step_by(0x10) {
                let mut found = true;
                for i in 0..8 {
                    let addr = (base_addr + i) as *const u8;
                    if *addr != SIGNATURE[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    return Some(base_addr as *const RSDP);
                }
            }
            None
        }
    }

    pub unsafe fn iter(&'_ self) -> SDTIterator<'_> {
        unsafe { SDTIterator::new(self) }
    }
}
