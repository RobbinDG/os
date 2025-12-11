static mut FREE_MEM_ADDR: usize = 0x10000;
const PAGE_SIZE: usize = 0x1000;
const PAGE_SIZE_MASK: usize = !(PAGE_SIZE - 1);

pub unsafe fn malloc(size: usize, align: bool) -> *mut u8 {
    unsafe {
        if align && (FREE_MEM_ADDR & !PAGE_SIZE_MASK) > 0 {
            FREE_MEM_ADDR &= PAGE_SIZE_MASK;
            FREE_MEM_ADDR += PAGE_SIZE;
        }

        let phys_addr = FREE_MEM_ADDR as *mut u8;
        FREE_MEM_ADDR += size;
        phys_addr
    }
}

pub struct OutOfBounds;

pub struct DynArray<T>
where
    T: Sized,
{
    start: *mut T,
    size: usize,
}

impl<T: Sized> DynArray<T> {
    pub unsafe fn new(count: usize, align: bool) -> Self {
        unsafe {
            let size = core::mem::size_of::<T>() * count;
            let start = malloc(size, align) as *mut T;
            Self { start, size }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub unsafe fn get(&self, i: usize) -> Result<&T, OutOfBounds> {
        unsafe { Ok(&*self.elem_ptr(i)?) }
    }

    pub unsafe fn set(&mut self, i: usize, v: T) -> Result<(), OutOfBounds> {
        unsafe {
            let addr = self.elem_ptr(i)?;
            *addr = v;
            Ok(())
        }
    }

    unsafe fn elem_ptr(&self, i: usize) -> Result<*mut T, OutOfBounds> {
        if i < self.size {
            unsafe {
                let addr = self.start.add(i * core::mem::size_of::<T>());
                Ok(addr)
            }
        } else {
            Err(OutOfBounds)
        }
    }
}
