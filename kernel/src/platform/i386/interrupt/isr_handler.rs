use core::{arch::asm, hint::black_box};

use crate::platform::i386::{
    interrupt::data::InterruptHandlerData, syscall_handler::syscall,
};

#[unsafe(no_mangle)]
#[inline(never)]
unsafe extern "C" fn isr_handler(regs: &mut InterruptHandlerData) {
    unsafe {
        if regs.int_no & 0xFF == 0x80 {
            black_box({
                // The input object is directly allocated on the stack and will be
                // popped back into the process' state once we return from the interrupt.
                // Normally this is undesirable, but since we know that this ISR is generated
                // by the callee, we can safely overwrite it and the return value will be available
                // after the syscall returns.
                regs.reg.eax = syscall(regs) as u32;
            }
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn isr_common_stub() {
    unsafe {
        asm!(
            "pusha",        // Pushes edi,esi,ebp,esp,ebx,edx,ecx,eax (needs to be on kernel stack)
            "mov ax, ds",   // Lower 16-bits of eax = ds.
            "push eax",     // save the data segment descriptor
            "mov ax, 0x10", // kernel data segment descriptor
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            // Push a pointer to the sturcture created right before as argument to the isr_handler.
            // In principe, just calling the function instead and passing the argument by value works.
            // However, the compiler figures out that the value gets discarded at the end of the function,
            // and optimizes away every operation on it.
            // TODO Until I find a better way to let the compiler know that the value is, in fact, not
            // being discarded, I will pass a reference/pointer instead. Only takes 2 extra instuctions anyway.
            "push esp",
            "call {inner}",
            "pop esp",
            "pop eax",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "popa",
            "add esp, 8", // Cleans up the pushed error code and pushed ISR number
            "sti",
            "iretd", // pops 5 things at once: CS, EIP, EFLAGS, SS, and ESP
            inner = sym isr_handler,
        )
    }
}
