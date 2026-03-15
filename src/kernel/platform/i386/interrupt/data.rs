/// Result of `pusha`.
#[repr(C, packed)]
#[derive(Default)]
pub struct Registers {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
}

/// Pushed during `int` call.
#[repr(C, packed)]
#[derive(Default)]
pub struct IntData {
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub useresp: u32,
    pub ss: u32,
}

#[repr(C, packed)]
#[derive(Default)]
pub struct InterruptHandlerData {
    /// Pushed during isr_common_stub
    pub ds: u32,
    /// Result of `pusha`. Will be popped off the stack on return to user mode, so return values
    /// can be inserted here.
    pub reg: Registers,
    // Manually pushed during ISR/IRQ hander in ASM.
    pub int_no: u32,
    pub err_code: u32, // Optional for built-in ISRs, so sometimes manually pushed

    pub int: IntData,
}