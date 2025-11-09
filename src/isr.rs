use core::arch::asm;

use crate::{
    idt::{IDTGate, IDTReg},
    interrupt_handlers::INTERRUPT_HANDLERS,
    pic::PIC,
    printer::TTY,
};

const NUM_IDT_GATES: usize = 256;
type IDTGates = [IDTGate; NUM_IDT_GATES];

static mut IDT_REG: IDTReg = IDTReg::null();
static mut IDT: IDTGates = {
    let emtpy_gate = IDTGate::new();
    [emtpy_gate; NUM_IDT_GATES]
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
    ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    int_no: u32,
    err_code: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32,
}

#[unsafe(no_mangle)]
unsafe extern "C" fn isr_handler(regs: Registers) {
    unsafe {
        if let Some(mut tty) = TTY::get_instance() {
            tty.println_ascii(ISR_EXCEPTION_MSGS[regs.int_no as usize].as_bytes());
            tty.print_hex(regs.int_no as u16);
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn irq_handler(regs: Registers) {
    unsafe {
        PIC::send_eoi(regs.int_no as u8);
    }
    if regs.int_no > 0 {
        INTERRUPT_HANDLERS[regs.int_no as usize](regs);
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
