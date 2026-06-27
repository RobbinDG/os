use core::{
    arch::{asm, naked_asm},
    mem::offset_of,
    ptr,
};

use crate::{
    SCHEDULER,
    kernel::platform::i386::{interrupt::data::InterruptHandlerData, tss::TSS},
    scheduler_entrypoint,
};

/// Stores all information about a process.
/// repr(C) is necessary because we need to compute a consistent offset.
#[repr(C)]
pub struct ProcessCtrlBlock {
    stack_base_ptr: usize, // This is a pointer, but kept as usize to prevent issues with Send/Sync.
    stack_ptr: usize,
    pub prog_ctr: usize,
    interrupt_data: *const InterruptHandlerData,
}

impl ProcessCtrlBlock {
    /// Creates a blank PCB that does not contain any meaningful information. This is
    /// needed to initialise a new PCB for the current thread. This should never be a problem
    /// as the thread should always store all required information during a context switch
    /// (so this is a lazy data structure).
    ///
    /// TODO is this really a good implementation? Can we not do something with None/Option
    /// optimisatino that does not cause problems for context switches?
    pub const fn blank() -> Self {
        Self {
            stack_base_ptr: 0xabcdef,
            stack_ptr: 0xabcdef,
            prog_ctr: 0xabcdef,
            interrupt_data: ptr::null(),
        }
    }

    pub unsafe fn current_process() -> Self {
        let mut s = Self {
            stack_base_ptr: 0,
            stack_ptr: 0,
            prog_ctr: 0,
            interrupt_data: ptr::null(),
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
            interrupt_data: ptr::null(),
        }
    }

    #[inline]
    pub fn set_entry(&mut self, func_ptr: usize) {
        self.prog_ctr = func_ptr;
    }

    #[inline]
    pub fn set_interrupt(&mut self, data: *const InterruptHandlerData) {
        self.interrupt_data = data;
    }
}

/// We have to write naked ASM here, as Rust will not take the changed stack pointer into account.
/// We use and follow the C/i386 calling convention to update necessary information. Changing the structure of
/// `ProcessCtrlBlock` will require updating this as well.
///
/// Executing a context switch will save the `cur` context and load the `new` context.
/// Execution continues from the last saved program counter in `new`.
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(
    cur: *mut ProcessCtrlBlock,
    new: *const ProcessCtrlBlock,
) -> ! {
    naked_asm!(
        "pop edx", // pop off the top of the original stack; the original return address. Assume that edx is safe to use...
        "pop eax", // First argument
        "pop ecx", // Second argument
        "pop esi", // Pop interrupt arg 1 (if any, dangerous if none! Should probably fix this.)
        "mov dword ptr [eax + {o1}], ebp", // Backup base pointer.
        "mov dword ptr [eax + {o2}], esp", // Backup stack pointer.
        "mov dword ptr [eax + {o3}], edx", // Backup program counter as the original return address.
        "mov ebp, dword ptr [ecx + {o1}]", // update stack base pointer.
        "mov esp, dword ptr [ecx + {o2}]", // update stack pointer.
        "push edi", // Push back interrupt info for later use
        "push esi", // ^
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

    unsafe { exit_thread(SCHEDULER.with(|s| s.scheduler_process() as *const ProcessCtrlBlock)) };
}

#[unsafe(naked)]
pub unsafe extern "C" fn exit_thread(scheduler_ctx: *const ProcessCtrlBlock) -> ! {
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

pub unsafe fn switch_to_user_mode(to_ctx: &ProcessCtrlBlock, tss: &mut TSS) -> ! {
    unsafe {
        asm!("mov {esp0}, esp", esp0 = out(reg) tss.esp0);
        enter_user_mode(to_ctx.prog_ctr, to_ctx.stack_base_ptr)
    }
}

/// Enter user mode by using the `iret` trick. This function continues
/// executing from `entry_point`. It is paramount that this function does not
/// leave breadcrumbs on the stack.
#[unsafe(naked)]
unsafe extern "C" fn enter_user_mode(entry_point: usize, stack_base_ptr: usize) -> ! {
    naked_asm!(
        "pop eax", // Pop original return address. `eax` will be overwritten.
        "pop edx", // Pop argument 1
        "pop ecx", // Pop argument 2
        "mov ax, (4 * 8) | 3",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        // Prepare `iret` stack frame
        "mov eax, (4 * 8) | 3",
        "push eax", // SS
        "push ecx", // Stack pointer
        "pushfd",   // Flags
        "mov eax, (3 * 8) | 3",
        "push eax", // CS
        "push edx", // continue execution from input argument entrypoint.
        "iretd",    // Compiles to `iret`. Putting `iret` here compiles to `iretw`.
    );
}
