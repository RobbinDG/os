use crate::{KERNEL, kernel::{
    event_buf::{EVENT_BUF, EVENT_BUF_SIZE},
    interrupt_handlers::INTERRUPT_HANDLERS,
    platform::i386::interrupt::{data::InterruptHandlerData, pic::PIC},
}, sys_event::SysEvent};

#[unsafe(no_mangle)]
unsafe extern "C" fn irq_handler(mut regs: InterruptHandlerData) {
    unsafe {
        PIC::send_eoi(regs.int_no as u8);
        if regs.int_no > 0 {
            if regs.int_no == 0x12 {
                regs.int_no += 1;
            }
            if let Some(event) = INTERRUPT_HANDLERS[regs.int_no as usize](regs) {
                // TODO BHV this leads to missed events if the buffer fills up, and gives no warnings.
                EVENT_BUF.buf[EVENT_BUF.len % EVENT_BUF_SIZE] = Some(event);
                EVENT_BUF.len += 1;
                match event {
                    SysEvent::Keyboard => KERNEL.keyboard_driver.with_init(|drv| drv.keyboard_interrupt_handler()),
                    _ => (),
                }
            }
        }
    }
}
