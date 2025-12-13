use crate::kernel::{kernel::KernelError, mem::MemoryManager};

pub struct DynArray<'a, T>
where
    T: Sized,
{
    start: *mut T,
    size: usize,
    mem: &'a spin::Mutex<MemoryManager>,
}

impl<'a, T: Sized> DynArray<'a, T> {
    pub unsafe fn new(count: usize, align: bool, mem: &'a spin::Mutex<MemoryManager>) -> Self {
        unsafe {
            let size = core::mem::size_of::<T>() * count;
            let start = mem.lock().malloc(size, align) as *mut T;
            Self { start, size, mem }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub unsafe fn get(&self, i: usize) -> Result<&T, KernelError> {
        unsafe { Ok(&*self.elem_ptr(i)?) }
    }

    pub unsafe fn set(&mut self, i: usize, v: T) -> Result<(), KernelError> {
        unsafe {
            let addr = self.elem_ptr(i)?;
            *addr = v;
            Ok(())
        }
    }

    unsafe fn elem_ptr(&self, i: usize) -> Result<*mut T, KernelError> {
        if i < self.size {
            unsafe {
                let addr = self.start.add(i * core::mem::size_of::<T>());
                Ok(addr)
            }
        } else {
            Err(KernelError::OutOfBounds)
        }
    }
}
