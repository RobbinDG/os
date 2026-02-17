use core::{
    arch::{asm, naked_asm},
    mem::offset_of,
};

use crate::{KERNEL, scheduler_entrypoint};

/// Stores all information about a process.
/// repr(C) is necessary because we need to compute a consistent offset.
#[repr(C)]
pub struct ProcessCtrlBlock {
    stack_base_ptr: usize, // This is a pointer, but kept as usize to prevent issues with Send/Sync.
    stack_ptr: usize,
    prog_ctr: usize,
}

impl ProcessCtrlBlock {
    pub unsafe fn current_process() -> Self {
        let mut s = Self {
            stack_base_ptr: 0,
            stack_ptr: 0,
            prog_ctr: 0,
        };
        unsafe {
            asm!(
                "mov {sb}, ebp",
                "mov {sp}, esp",
                sb = out(reg) s.stack_base_ptr,
                sp = out(reg) s.stack_ptr,
                options(nostack)
            )
        }
        s
    }

    pub fn new_process(stack_base_ptr: usize, entry_point: fn()) -> Self {
        Self {
            stack_base_ptr,
            stack_ptr: stack_base_ptr,
            prog_ctr: entry_point as usize,
        }
    }
}

/// We have to write naked ASM here, as Rust will not take the changed stack pointer into account.
/// We use and follow the C/i386 calling convention to update necessary information. Changing the structure of
/// `ProcessCtrlBlock` will require updating this as well.
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(
    /*entry_point: fn(),*/ cur: &ProcessCtrlBlock,
    new: &ProcessCtrlBlock,
) -> ! {
    naked_asm!(
        "pop edx", // pop off the top of the original stack; the original return address. Assume that edx is safe to use...
        "pop eax", // First argument
        "pop ecx", // Second argument
        "mov [eax + {o1}], ebp", // Backup base pointer.
        "mov [eax + {o2}], esp", // Backup stack pointer.
        "mov [eax + {o3}], edx", // Backup program counter as the original return address.
        "mov ebp, [ecx + {o1}]", // update stack base pointer.
        "mov esp, [ecx + {o2}]", // update stack pointer.
        "push [ecx + {o3}]", // Push function pointer, first argument.
        "call {entry}", // Jump to the entry point of the process. No jmp because the call convention would be broken.
        o1 = const offset_of!(ProcessCtrlBlock, stack_base_ptr),
        o2 = const offset_of!(ProcessCtrlBlock, stack_ptr),
        o3 = const offset_of!(ProcessCtrlBlock, prog_ctr),
        entry = sym enter_thread,
    );
}

#[inline(never)]
pub unsafe extern "C" fn enter_thread(func_ptr: usize) -> ! {
    let func: extern "C" fn() = unsafe { core::mem::transmute(func_ptr) };
    func();

    match KERNEL.get() {
        Ok(kernel) => unsafe { exit_thread(kernel.scheduler.scheduler_process()) },
        Err(_) => loop {}, // Unrecoverable error
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn exit_thread(scheduler_ctx: &ProcessCtrlBlock) -> ! {
    naked_asm!(
        "pop edx", // pop off the top of the original stack; the original return address. Assume that edx is safe to use...
        "pop eax", // First argument
        "mov ebp, [eax + {o1}]", // restore stack base pointer.
        "mov esp, [eax + {o2}]", // restore stack pointer.
        "jmp {scheduler_entry}", // restore instruction pointer
        o1 = const offset_of!(ProcessCtrlBlock, stack_base_ptr),
        o2 = const offset_of!(ProcessCtrlBlock, stack_ptr),
        scheduler_entry = sym scheduler_entrypoint,
    )
}
