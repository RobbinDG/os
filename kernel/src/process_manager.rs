use crate::kernel::platform::i386::context_switch::{ProcessCtrlBlock, switch_context};

pub struct ProcessManager {
    processes: Option<ProcessCtrlBlock>,
}

impl ProcessManager {
    /// Create a new process manager and allocate process tables.
    /// Assume that this needs heap allocation.
    pub fn new() -> Self {
        Self { processes: None }
    }

    #[inline(never)]
    pub unsafe fn create_process(&mut self, process: fn()) -> ProcessCtrlBlock {
        let pcb = ProcessCtrlBlock::new_process(0x7FFFF, process);
        // self.processes = Some(pcb);
        pcb
    }
}
