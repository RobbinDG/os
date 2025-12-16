use core::arch::asm;

use crate::{
    idt::{IDTGate, IDTReg},
    interrupt_handlers::INTERRUPT_HANDLERS,
    pic::PIC,
    printer::VGAText,
    sys_event::SysEvent,
};

const NUM_IDT_GATES: usize = 256;
type IDTGates = [IDTGate; NUM_IDT_GATES];

static mut IDT_REG: IDTReg = IDTReg::null();
static mut IDT: IDTGates = {
    let emtpy_gate = IDTGate::new();
    [emtpy_gate; NUM_IDT_GATES]
};
static mut LAST_INTERRUPT: u32 = 0;

const EVENT_BUF_SIZE: usize = 8;
struct EventBuf {
    buf: [Option<SysEvent>; EVENT_BUF_SIZE],
    len: usize,
}

static mut EVENT_BUF: EventBuf = EventBuf {
    buf: [None; EVENT_BUF_SIZE],
    len: 0,
};

const ISR_EXCEPTION_MSGS: [&str; 32] = [
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt",
    "Coprocessor Fault",
    "Alignment Check",
    "Machine Check",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
];

pub unsafe fn set_isr() {
    unsafe {
        IDT[0].set(isr0);
        IDT[1].set(isr1);
        IDT[2].set(isr2);
        IDT[3].set(isr3);
        IDT[4].set(isr4);
        IDT[5].set(isr5);
        IDT[6].set(isr6);
        IDT[7].set(isr7);
        IDT[8].set(isr8);
        IDT[9].set(isr9);
        IDT[10].set(isr10);
        IDT[11].set(isr11);
        IDT[12].set(isr12);
        IDT[13].set(isr13);
        IDT[14].set(isr14);
        IDT[15].set(isr15);
        IDT[16].set(isr16);
        IDT[17].set(isr17);
        IDT[18].set(isr18);
        IDT[19].set(isr19);
        IDT[20].set(isr20);
        IDT[21].set(isr21);
        IDT[22].set(isr22);
        IDT[23].set(isr23);
        IDT[24].set(isr24);
        IDT[25].set(isr25);
        IDT[26].set(isr26);
        IDT[27].set(isr27);
        IDT[28].set(isr28);
        IDT[29].set(isr29);
        IDT[30].set(isr30);
        IDT[31].set(isr31);

        PIC::remap(32, 40);

        IDT[32].set(irq0);
        IDT[33].set(irq1);
        IDT[34].set(irq2);
        IDT[35].set(irq3);
        IDT[36].set(irq4);
        IDT[37].set(irq5);
        IDT[38].set(irq6);
        IDT[39].set(irq7);
        IDT[40].set(irq8);
        IDT[41].set(irq9);
        IDT[42].set(irq10);
        IDT[43].set(irq11);
        IDT[44].set(irq12);
        IDT[45].set(irq13);
        IDT[46].set(irq14);
        IDT[47].set(irq15);

        IDT_REG.base = &IDT[0];
        IDT_REG.limit = (core::mem::size_of::<IDTGates>() - 1) as u16;
        let idt_reg_ptr: *const u16 = &raw const IDT_REG.limit;
        asm!(
            "lidt [{0}]",
            in(reg) idt_reg_ptr,
            options(nostack, preserves_flags)
        );
    }
}

#[repr(C, packed)]
pub struct Registers {
    pub ds: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub int_no: u32,
    pub err_code: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub useresp: u32,
    pub ss: u32,
}

#[unsafe(no_mangle)]
unsafe extern "C" fn isr_handler(regs: Registers) {
    unsafe {
        if let Some(mut tty) = VGAText::get_instance() {
            LAST_INTERRUPT = regs.int_no;
            tty.println_ascii(ISR_EXCEPTION_MSGS[regs.int_no as usize].as_bytes());
            tty.print_hex(regs.int_no);
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn irq_handler(mut regs: Registers) {
    unsafe {
        PIC::send_eoi(regs.int_no as u8);
        if regs.int_no > 0 {
            LAST_INTERRUPT = regs.int_no;
            if regs.int_no == 0x12 {
                regs.int_no += 1;
            }
            if let Some(event) = INTERRUPT_HANDLERS[regs.int_no as usize](regs) {
                // TODO BHV this leads to missed events if the buffer fills up, and gives no warnings.
                EVENT_BUF.buf[EVENT_BUF.len % EVENT_BUF_SIZE] = Some(event);
                EVENT_BUF.len += 1;
            }
        }
    }
}

pub unsafe fn empty_event_buffer() -> [Option<SysEvent>; EVENT_BUF_SIZE] {
    unsafe {
        let cp = EVENT_BUF.buf;
        while EVENT_BUF.len > 0 {
            EVENT_BUF.len -= 1;
            EVENT_BUF.buf[EVENT_BUF.len] = None;
        }
        cp
    }
}

pub unsafe fn clear_last_interrupt() {
    unsafe {
        LAST_INTERRUPT = 0;
    }
}

pub unsafe fn last_interrupt() -> u32 {
    unsafe {
        let li = LAST_INTERRUPT;
        LAST_INTERRUPT = 0;
        li
    }
}

unsafe extern "C" {
    fn isr0();

    fn isr1();

    fn isr2();

    fn isr3();

    fn isr4();

    fn isr5();

    fn isr6();

    fn isr7();

    fn isr8();

    fn isr9();

    fn isr10();

    fn isr11();

    fn isr12();

    fn isr13();

    fn isr14();
    fn isr15();

    fn isr16();

    fn isr17();

    fn isr18();

    fn isr19();

    fn isr20();

    fn isr21();

    fn isr22();

    fn isr23();

    fn isr24();

    fn isr25();

    fn isr26();

    fn isr27();

    fn isr28();

    fn isr29();

    fn isr30();

    fn isr31();
    fn irq0();
    fn irq1();
    fn irq2();
    fn irq3();
    fn irq4();
    fn irq5();
    fn irq6();
    fn irq7();
    fn irq8();
    fn irq9();
    fn irq10();
    fn irq11();
    fn irq12();
    fn irq13();
    fn irq14();
    fn irq15();
}
