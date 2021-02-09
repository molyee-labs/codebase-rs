use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
use core::ptr;

pub struct Ptr<T>(AtomicPtr<T>);

unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}

impl<T> Default for Ptr<T> {
    #[inline]
    fn default() -> Self {
        Self(AtomicPtr::default())
    }
}

impl<T> From<&mut T> for Ptr<T> {
    fn from(p: &mut T) -> Self {
        Self(AtomicPtr::new(p))
    }
}

impl<T> Ptr<T> {
    pub fn null() -> Self {
        Self::default()
    }
    
    pub fn new(p: &mut T) -> Self {
        p.into()
    }

    fn take(&self) -> *mut T {
        loop {
            let p = self.0.swap(ptr::null_mut(), Ordering::Acquire);
            if !p.is_null() {
                return p;
            }
        }
    }

    fn put(&self, p: &mut T) {
        debug_assert_eq!(self.0.load(Ordering::SeqCst), ptr::null_mut());
        self.0.store(p, Ordering::Release);
    }

    pub fn replace(&mut self, p: &mut T) -> bool {
        let mut cur = self.take();
        let mut stored = cur as *const T;
        while stored == self.0.compare_and_swap(cur, p, Ordering::Release) {
            cur = self.take();
            stored = cur as *const T;
        }
        !cur.is_null()
    }
}
