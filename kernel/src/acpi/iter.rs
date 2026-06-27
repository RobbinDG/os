use crate::kernel::acpi::acpi::{ACPI, ACPISDTHeader};


pub struct SDTIterator<'a> {
    idx: usize,
    acpi: &'a ACPI,
}

impl<'a> SDTIterator<'a> {
    pub unsafe fn new(acpi: &'a ACPI) -> Self {
        Self { idx: 0, acpi }
    }

    pub fn len(&self) -> usize {
        self.acpi.sdt_cnt
    }

    pub unsafe fn next(&mut self) -> Option<&'a ACPISDTHeader> {
        if self.idx >= self.acpi.sdt_cnt {
            return None;
        }
        unsafe {
            let addr = self.acpi.sdt_ptrs.add(self.idx);
            let header = &**addr;
            self.idx += 1;
            Some(header)
        }
    }
}
