use crate::kernel::kernel::Kernel;

pub mod acpi;
mod console;
pub mod event_buf;
pub mod global;
mod interrupt_handlers;
pub mod kernel;
pub mod keyboard_driver; // TODO remove from kernel, make separate module
pub mod mem;
pub(in crate::kernel) mod paging;
pub mod platform;
mod ports;
pub mod pre_boot;
mod process_manager;
mod ps2;
pub mod scheduler;
pub mod syscalls;
pub mod tty;
pub mod vga_driver;

pub(self) static KERNEL: Kernel = Kernel::new();
