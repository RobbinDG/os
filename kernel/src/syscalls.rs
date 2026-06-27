use core::slice;

use crate::{KERNEL, platform::i386::interrupt::data::InterruptHandlerData};
use common::SysCall;

#[inline]
pub unsafe fn call(syscall: &SysCall, data: &mut InterruptHandlerData) -> usize {
    unsafe {
        match syscall {
            SysCall::Unknown => 255,
            SysCall::Write => write(data),
            SysCall::Read => read(data),
            SysCall::MMap => mmap(data),
            SysCall::MUnmap => munmap(data),
            SysCall::_LastDummy => 255,
        }
    }
}

unsafe fn write(regs: &mut InterruptHandlerData) -> usize {
    unsafe {
        KERNEL.tmp_tty.with_unwrap(|tty| {
            tty.write(slice::from_raw_parts(
                regs.reg.ecx as *const u8,
                regs.reg.edx as usize,
            ))
        })
    }
}

unsafe fn read(regs: &mut InterruptHandlerData) -> usize {
    unsafe {
        KERNEL.tmp_tty.with_unwrap(|tty| {
            tty.read(
                core::slice::from_raw_parts_mut(regs.reg.ecx as *mut u8, regs.reg.edx as usize),
                regs.reg.edx as usize,
            )
        })
    }
}

unsafe fn mmap(regs: &mut InterruptHandlerData) -> usize {
    unsafe {
        KERNEL
            .mem
            .with_unwrap(|mem| mem.map(regs.reg.ecx as usize, false)) as usize
    }
}

unsafe fn munmap(regs: &mut InterruptHandlerData) -> usize {
    unsafe {
        KERNEL
            .mem
            .with_unwrap(|mem| mem.unmap(regs.reg.ecx as *mut (), regs.reg.edx as usize));
        0
    }
}
