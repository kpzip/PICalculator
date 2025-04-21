use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct LazyLock<T, F: FnOnce() -> T = fn() -> T> {
    func: F,
    inner: UnsafeCell<Option<T>>,
    is_initializing: AtomicBool,
}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    pub const fn new(func: F) -> Self {
        Self {
            func,
            inner: UnsafeCell::new(None),
            is_initializing: AtomicBool::new(false),
        }
    }
}

impl<T> Deref for LazyLock<T, fn() -> T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            match &*self.inner.get() {
                Some(x) => x,
                None => {
                    if !self.is_initializing.fetch_or(true, Ordering::Acquire) {
                        let val = (self.func)();
                        *self.inner.get() = Some(val);
                        self.is_initializing.store(false, Ordering::Release);
                        (&*self.inner.get()).as_ref().unwrap()
                    } else {
                        while self.is_initializing.load(Ordering::Relaxed) {
                            core::hint::spin_loop();
                        }
                        (&*self.inner.get()).as_ref().unwrap()
                    }
                }
            }
        }
    }
}

unsafe impl<T: Send, F: FnOnce() -> T> Send for LazyLock<T, F> {}
unsafe impl<T: Send + Sync, F: FnOnce() -> T> Sync for LazyLock<T, F> {}
