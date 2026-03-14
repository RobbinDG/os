#[repr(C, packed)]
#[derive(Default)]
pub struct TSS {
    _link: u16,
    _reserved0: u16,
    pub esp0: u32,
    pub ss0: u16,
    _reserved1: u16,
    esp1: u32,
    ss1: u16,
    _reserved2: u16,
    esp2: u32,
    ss2: u16,
    _reserved3: u16,
    _cr3: u32,
    _eip: u32,
    _eflags: u32,
    _eax: u32,
    _ecx: u32,
    _edx: u32,
    _ebx: u32,
    _esp: u32,
    _ebp: u32,
    _esi: u32,
    _edi: u32,
    es: u16,
    _es_reserved: u16,
    cs: u16,
    _cs_reserved: u16,
    ss: u16,
    _ss_reserved: u16,
    ds: u16,
    _ds_reserved: u16,
    fs: u16,
    _fs_reserved: u16,
    gs: u16,
    _gs_reserved: u16,
    _ldtr: u16,
    _ldtr_reserved: u16,
    _iopb_reserved: u16,
    iopb: u16,
    _ssp: u32,
}

impl TSS {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(&mut self, esp0: u32, data_segment: u16) {
        self.esp0 = esp0;
        self.ss0 = data_segment;
        self.iopb = core::mem::size_of::<TSS>() as u16;
        // We allow access to the TSS segments from ring 3 and below.
        // self.cs = code_segment | 0x3;
        // self.ss = data_segment | 0x3;
        // self.ds = data_segment | 0x3;
        // self.es = data_segment | 0x3;
        // self.fs = data_segment | 0x3;
        // self.gs = data_segment | 0x3;
    }
}
