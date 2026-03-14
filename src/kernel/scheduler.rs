use core::ptr;

use crate::{KERNEL, kernel::platform::i386::context_switch::{ProcessCtrlBlock, switch_to_user_mode}};

pub struct Scheduler {
    root: ProcessCtrlBlock,
    current: *mut ProcessCtrlBlock,
}

impl Scheduler {
    pub const unsafe fn new() -> Self {
        Self {
            root: ProcessCtrlBlock::blank(),
            current: ptr::null_mut(),
        }
    }

    #[inline]
    pub unsafe fn current_process(&self) -> &ProcessCtrlBlock {
        if self.current.is_null() {
            &self.root
        } else {
            unsafe { &*self.current }
        }
    }

    #[inline]
    pub unsafe fn current_process_mut(&mut self) -> &mut ProcessCtrlBlock {
        if self.current.is_null() {
            &mut self.root
        } else {
            unsafe { &mut *self.current }
        }
    }

    #[inline]
    pub unsafe fn set_current_process(&mut self, new: *mut ProcessCtrlBlock) {
        self.current = new;
    }

    #[inline]
    pub fn scheduler_process(&self) -> &ProcessCtrlBlock {
        &self.root
    }

    #[inline]
    pub fn scheduler_process_mut(&mut self) -> &mut ProcessCtrlBlock {
        &mut self.root
    }

    pub fn run_process(&mut self, process: &mut ProcessCtrlBlock) {
        unsafe {
            self.set_current_process(process);
            if let Ok(kernel) = KERNEL.get() {
                kernel.tss
            }
            switch_to_user_mode(process, KERNEL.get())
        }
    }
}
