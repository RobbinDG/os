use core::slice;

use crate::kernel::{KERNEL, platform::i386::interrupt::data::InterruptHandlerData};

pub enum SysCall {
    Unknown = 0,
    Write = 1,
    Read = 2,
    MMap = 3,
    MUnmap = 4,
    _LastDummy,
}

impl SysCall {
    pub unsafe fn transmute_u32(value: u32) -> Self {
        unsafe {
            // We expect to call this often, and correctly. Hint to the compiler
            // that this is the case so it will compile the comparisons favourably.
            if core::intrinsics::unlikely(value > SysCall::_LastDummy as u32) {
                SysCall::Unknown
            } else {
                core::mem::transmute(value as u8)
            }
        }
    }

    #[inline]
    pub unsafe fn call(&self, data: &mut InterruptHandlerData) -> usize {
        unsafe {
            match self {
                SysCall::Unknown => 255,
                SysCall::Write => Self::write(data),
                SysCall::Read => Self::read(data),
                SysCall::MMap => Self::mmap(data),
                SysCall::MUnmap => Self::munmap(data),
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
}
