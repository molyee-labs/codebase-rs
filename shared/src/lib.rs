use std::sync::{Arc, Mutex, MutexGuard};
use std::cell::{RefCell, UnsafeCell};

#[derive(Default)]
pub struct Ptr<T: ?Sized>(Arc<UnsafeCell<T>>);

impl<T> Clone for Ptr<T> {
    #[inline]
    fn clone(&self) -> Self {
        Ptr(Arc::clone(&self.0))
    }
}

unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}

impl<T: Default> Ptr<T> {
    #[inline]
    pub fn new() -> Self {
        Ptr(Default::default())
    }
}

impl<T> From<T> for Ptr<T> {
    #[inline]
    fn from(from: T) -> Self {
        Ptr(Arc::new(UnsafeCell::new(from)))
    }
}

#[derive(Default)]
pub struct Link<T: ?Sized>(Arc<Mutex<T>>);

impl<T> Clone for Link<T> {
    #[inline]
    fn clone(&self) -> Self {
        Link(Arc::clone(&self.0))
    }
}

unsafe impl<T> Send for Link<T> {}
unsafe impl<T> Sync for Link<T> {}

impl<T: Default> Link<T> {
    #[inline]
    pub fn new() -> Self {
        Link(Default::default())
    }
}

impl<T> Link<T> {
    #[inline]
    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

impl<T> From<T> for Link<T> {
    #[inline]
    fn from(from: T) -> Self {
        Link(Arc::new(Mutex::new(from)))
    }
}

#[derive(Default)]
pub struct LinkMut<T>(Arc<Mutex<RefCell<T>>>);

impl<T> Clone for LinkMut<T> {
    #[inline]
    fn clone(&self) -> Self {
        LinkMut(Arc::clone(&self.0))
    }
}

unsafe impl<T> Send for LinkMut<T> {}
unsafe impl<T> Sync for LinkMut<T> {}

impl<T: Default> LinkMut<T> {
    #[inline]
    pub fn new() -> Self {
        LinkMut(Default::default())
    }
}

impl<T> LinkMut<T> {
    #[inline]
    pub fn lock(&self) -> MutexGuard<RefCell<T>> {
        self.0.lock().unwrap()
    }
}

impl<T> From<T> for LinkMut<T> {
    #[inline]
    fn from(from: T) -> Self {
        LinkMut(Arc::new(Mutex::new(RefCell::new(from))))
    }
}
