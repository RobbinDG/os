use core::{cell::UnsafeCell, panic};

/// Helper structure for mutable global state. It internally manages
/// an `UnsafeCell`. Although the way the state is exposes promotes safe
/// usage of the internal data, no guarantees can be made as any function can be passed.
/// Function that have `!` as return type will still be allowed, which is
/// not generally desireable.
pub struct Global<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for Global<T> {}

impl<T> Global<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub unsafe fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { f(&mut *self.inner.get()) }
    }

    pub unsafe fn ptr(&self) -> *mut T {
        self.inner.get()
    }
}

pub struct GlobalLazy<T> {
    inner: Global<Option<T>>,
}

impl<T> GlobalLazy<T> {
    pub const fn empty() -> Self {
        Self {
            inner: Global::new(None),
        }
    }

    pub unsafe fn init(&self, inner: T) {
        unsafe { self.inner.with(|g| g.replace(inner)) };
    }

    pub unsafe fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    {
        unsafe { self.inner.with(|g| match g {
            Some(r) => f(r),
            None => panic!(),
        }) }
    }
}

impl<T> Default for GlobalLazy<T> {
    fn default() -> Self {
        Self::empty()
    }
}
