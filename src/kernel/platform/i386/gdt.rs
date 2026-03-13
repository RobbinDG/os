use core::arch::asm;

use crate::kernel::platform::i386::tss::TSS;

#[repr(C, packed)]
#[derive(Default)]
pub struct GDTR {
    size: u16,
    offset: u32,
}

impl GDTR {
    /// Loads the GDTR safely into this object.
    #[inline]
    pub fn load(&mut self) {
        unsafe {
            asm!(
                "sgdt [{loc}]",
                loc = in(reg) self,
            );
        }
    }

    /// Stores the GDTR, loading the global descriptor table.
    #[inline]
    pub fn store(&self) {
        unsafe {
            asm!(
                "lgdt [{loc}]",
                loc = in(reg) self,
            )
        }
    }

    pub fn for_gdt<const N: usize>(gdt: &[CompiledGDTEntry; N]) -> Self {
        Self {
            size: (N * core::mem::size_of::<CompiledGDTEntry>()) as u16,
            offset: gdt as *const [CompiledGDTEntry; N] as u32,
        }
    }

    /// Returns a reference to a raw GDT entry at a given index, referenced by this GDTR.
    /// Returns `None` if the GDTR is not loaded or the index is out of range.
    pub fn entry_raw(&self, i: usize) -> Option<&CompiledGDTEntry> {
        if self.offset == 0 {
            return None
        }
        if (i + 1) * core::mem::size_of::<CompiledGDTEntry>() - 1 <= self.size as usize {
            let addr = (self.offset as usize + i * core::mem::size_of::<CompiledGDTEntry>()) as *const CompiledGDTEntry;
            Some(unsafe { &*addr })            
        } else {
            None
        }        
    }

    /// Returns a decoded GDT entry at a given index, referenced by this GDTR.
    /// Returns `None` if the GDTR is not loaded or the index is out of range.
    pub fn entry(&self, i: usize) -> Option<GDTEntry> {
        self.entry_raw(i).map(GDTEntry::decode)
    }
}

pub type CompiledGDTEntry = [u8; 8];

/// A code-usable representation of the GDT entry. The actual entry is stored
/// as a complex 8-byte sequencee, which needs to be decoded first.
#[repr(C, packed)]
#[derive(Clone)]
pub struct GDTEntry {
    base: u32,
    /// Only uses lower 20 bits.
    pub limit: u32,
    /// Only uses lower 4 bits
    pub flags: u8,
    access: u8,
}

impl GDTEntry {
    pub fn null() -> Self {
        Self {
            base: 0,
            limit: 0,
            flags: 0,
            access: 0,
        }
    }

    pub fn for_task_state_segment(tss_ptr: &TSS) -> Self {
        Self {
            base: tss_ptr as *const TSS as u32,
            limit: core::mem::size_of::<TSS>() as u32,
            flags: 0x40,
            access: 0x89,
        }
    }

    pub fn decode(buf: &CompiledGDTEntry) -> Self {
        Self {
            base: (buf[2] as u32)
                | ((buf[3] as u32) << 8)
                | ((buf[4] as u32) << 16)
                | ((buf[7] as u32) << 24),
            limit: (buf[0] as u32) | ((buf[1] as u32) << 8) | (((buf[6] & 0x0F) as u32) << 16),
            flags: buf[6] >> 4,
            access: buf[5],
        }
    }

    pub fn encode(&self) -> CompiledGDTEntry {
        let mut buf = [0; 8];
        buf[2] = (self.base & 0xFF) as u8;
        buf[3] = ((self.base >> 8) & 0xFF) as u8;
        buf[4] = ((self.base >> 16) & 0xFF) as u8;
        buf[7] = ((self.base >> 24) & 0xFF) as u8;

        buf[0] = (self.limit & 0xFF) as u8;
        buf[1] = ((self.limit >> 8) & 0xFF) as u8;
        buf[6] = ((self.limit >> 16) & 0x0F) as u8;

        buf[6] |= (self.flags) << 4;

        buf[5] = self.access;

        buf
    }
}
