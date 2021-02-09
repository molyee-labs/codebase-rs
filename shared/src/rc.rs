use alloc::sync::Arc;
use core::{ops::{Deref, DerefMut}, sync::atomic::{AtomicUsize, Ordering}};
use core::marker::PhantomData;
use core::mem;

pub struct Rc<T: ?Sized>(Arc<T>);

impl<T> Rc<T> {
    pub fn new(data: T) -> Self {
        Self(Arc::new(data))
    }
}

impl<T: ?Sized> Clone for Rc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> DerefMut for Rc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

pub struct RcCell<T>(AtomicUsize, PhantomData<Rc<T>>);

impl<T> Drop for RcCell<T> {
    fn drop(&mut self) {
        self.take();
    }
}

impl<T> From<Rc<T>> for RcCell<T> {
    fn from(src: Rc<T>) -> Self {
        let rc = unsafe { mem::transmute(src) };
        Self(AtomicUsize::new(rc), PhantomData)
    }
}

impl<T> Clone for RcCell<T> {
    fn clone(&self) -> Self {
        Self::from(self.get())
    }
}

impl<T> RcCell<T> {
    pub fn new(t: T) -> Self {
        let rc = Rc::new(t);
        Self::from(rc)
    }

    pub fn get(&self) -> Rc<T> {
        let t = self.take();
        let out = t.clone();
        self.put(t);
        out
    }

    pub fn set(&self, t: Rc<T>) -> Rc<T> {
        let old = self.take();
        self.put(t);
        old
    }

    fn take(&self) -> Rc<T> {
        loop {
            match self.0.swap(0, Ordering::Acquire) {
                0 => {}
                n => return unsafe { mem::transmute(n) }
            }
        }
    }

    fn put(&self, t: Rc<T>) {
        debug_assert_eq!(self.0.load(Ordering::SeqCst), 0);
        self.0.store(unsafe { mem::transmute(t) }, Ordering::Release);
    }
}