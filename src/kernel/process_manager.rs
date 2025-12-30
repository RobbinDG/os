use crate::kernel::platform::i386::context_switch::{ProcessContext, switch_context};
use core::hint::black_box;


pub struct ProcessManager {}

impl ProcessManager {
    #[inline(never)]
    pub unsafe fn start_process(&mut self, process: fn()) {
        let ctx = ProcessContext { stack_base_ptr: 0x7FFFF as *const u8 };
        unsafe { switch_context(black_box(process), &ctx) };
    }
}
