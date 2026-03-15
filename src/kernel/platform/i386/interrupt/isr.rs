unsafe extern "C" {
    pub fn isr0();
    pub fn isr1();
    pub fn isr2();
    pub fn isr3();
    pub fn isr4();
    pub fn isr5();
    pub fn isr6();
    pub fn isr7();
    pub fn isr8();
    pub fn isr9();
    pub fn isr10();
    pub fn isr11();
    pub fn isr12();
    pub fn isr13();
    pub fn isr14();
    pub fn isr15();
    pub fn isr16();
    pub fn isr17();
    pub fn isr18();
    pub fn isr19();
    pub fn isr20();
    pub fn isr21();
    pub fn isr22();
    pub fn isr23();
    pub fn isr24();
    pub fn isr25();
    pub fn isr26();
    pub fn isr27();
    pub fn isr28();
    pub fn isr29();
    pub fn isr30();
    pub fn isr31();
    pub fn isr_syscall();
    pub fn irq0();
    pub fn irq1();
    pub fn irq2();
    pub fn irq3();
    pub fn irq4();
    pub fn irq5();
    pub fn irq6();
    pub fn irq7();
    pub fn irq8();
    pub fn irq9();
    pub fn irq10();
    pub fn irq11();
    pub fn irq12();
    pub fn irq13();
    pub fn irq14();
    pub fn irq15();
}

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
