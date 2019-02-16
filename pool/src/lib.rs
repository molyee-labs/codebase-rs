use builder::*;
use std::mem::ManuallyDrop;
use std::ptr;
use shared::LinkMut;

pub enum Error {
    LimitExceeded,
}

pub type Result<T> = std::result::Result<T, Error>;

struct Inner<T>(LinkMut<Vec<T>>);

pub struct Pool<T> {
    inner: Inner<T>,
    builder: SyncBuilder<T>,
}

pub struct Holder<T> {
    pool: Pool<T>,
    entry: ManuallyDrop<T>,
}

impl<T> Clone for Inner<T> {
    fn clone(&self) -> Self {
        Inner(LinkMut::clone(&self.0))
    }
}

impl<T> Inner<T> {
    fn new() -> Self {
        Inner(Default::default())
    }

    fn take(&self) -> Option<T> {
        let mut cell = self.0.lock();
        cell.get_mut().pop()
    }

    fn release(&self, entry: T) {
        let mut cell = self.0.lock();
        cell.get_mut().push(entry)
    }
}

impl<T> Clone for Pool<T> {
    fn clone(&self) -> Self {
        Pool {
            inner: Inner::clone(&self.inner),
            builder: SyncBuilder::clone(&self.builder),
        }
    }
}

unsafe impl<T> Sync for Pool<T> {}
unsafe impl<T> Send for Pool<T> {}

impl<T> From<DefaultBuilder<T>> for Pool<T>
where
    T: Default + 'static,
{
    fn from(from: DefaultBuilder<T>) -> Self {
        Pool::new(SyncBuilder::from(from))
    }
}

impl<T> From<FnBuilder<T>> for Pool<T>
where
    T: 'static,
{
    fn from(from: FnBuilder<T>) -> Self {
        Pool::new(SyncBuilder::from(from))
    }
}

impl<T> Pool<T>
where
    T: 'static,
{
    pub fn with_closure<F>(f: F) -> Self
    where
        F: Fn() -> T + Sized + 'static,
    {
        Pool::from(FnBuilder::from(f))
    }
}

impl<T> Pool<T> {
    pub fn new(builder: SyncBuilder<T>) -> Self {
        Pool {
            inner: Inner::new(),
            builder,
        }
    }

    pub fn take(&self) -> Holder<T> {
        let entry = self.inner.take().unwrap_or_else(|| self.builder.build());
        Holder {
            pool: Pool::clone(&self),
            entry: ManuallyDrop::new(entry),
        }
    }

    pub fn release(&self, entry: T) {
        self.inner.release(entry)
    }
}

impl<T> Drop for Holder<T> {
    fn drop(&mut self) {
        let field = unsafe { ptr::read(&self.entry) };
        let entry = ManuallyDrop::into_inner(field);
        &self.pool.release(entry);
    }
}
