pub mod acpi;
mod interrupt_handlers;
pub mod event_buf;
pub mod kernel;
pub mod keyboard_driver; // TODO remove from kernel, make separate module
pub mod mem;
pub mod platform;
mod ports;
pub mod pre_boot;
mod process_manager;
mod ps2;
pub mod scheduler;
pub mod vga_driver;
pub mod syscalls;
pub mod global;
pub mod tty;