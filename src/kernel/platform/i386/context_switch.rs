use core::arch::asm;

pub struct ProcessContext {
    stack_base_ptr: *const u8,
}

/// Using Rust ABI, the arguments are pushed onto the stack before the 
/// return address. Accesses are then done with relative offsets. This means
/// that `esp` contains the original return address.
#[inline(never)]
pub unsafe fn switch_context(entry_point: fn(), ctx: &ProcessContext) {
    unsafe {
        asm!(
            "pop {tmp}", // pop off the top of the original stack; the original return address.
            "mov ebp, {stack}", // change stack base pointer.
            "mov esp, ebp", // update stack pointer.
            "push {process}", // push return address to new stack, which will be ret'ed.
            stack = in(reg) ctx.stack_base_ptr,
            process = in(reg) entry_point,
            tmp = out(reg) _,
            options(nomem)
        );
    }
}
