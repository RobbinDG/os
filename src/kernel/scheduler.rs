use crate::kernel::platform::i386::context_switch::{ProcessCtrlBlock, switch_context};

pub struct Scheduler {
    root: ProcessCtrlBlock,
}

impl Scheduler {
    pub unsafe fn new() -> Self {
        Self {
            root: unsafe { ProcessCtrlBlock::current_process() },
        }
    }

    pub fn scheduler_process(&self) -> &ProcessCtrlBlock {
        &self.root
    }

    pub fn run_process(&self, process: &ProcessCtrlBlock) {
        unsafe { switch_context(&self.root, process) };
    }
}
