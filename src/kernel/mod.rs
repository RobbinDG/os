pub mod acpi;
mod idt;
mod interrupt_handlers;
pub mod isr;
pub mod kernel;
pub mod keyboard_driver; // TODO remove from kernel, make separate module
pub mod mem;
mod pic;
pub mod platform;
mod ports;
pub mod pre_boot;
mod process_manager;
mod ps2;
pub mod vga_driver;
