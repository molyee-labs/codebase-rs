use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

pub trait Build {
    type Out;

    fn build(&self) -> Self::Out;
}

#[derive(Default)]
pub struct DefaultBuilder<T>(PhantomData<T>);

pub struct FnBuilder<T>(Box<Fn() -> T>);

impl<F, T> From<F> for FnBuilder<T>
where
    F: Fn() -> T + Sized + 'static,
{
    fn from(from: F) -> Self {
        FnBuilder(Box::new(from))
    }
}

pub struct SyncBuilder<T>(Arc<Mutex<dyn Build<Out = T>>>);

unsafe impl<T> Send for SyncBuilder<T> {}
unsafe impl<T> Sync for SyncBuilder<T> {}

impl<T> Clone for SyncBuilder<T> {
    fn clone(&self) -> Self {
        SyncBuilder(Arc::clone(&self.0))
    }
}

impl<T> From<DefaultBuilder<T>> for SyncBuilder<T>
where
    T: Default + 'static,
{
    fn from(from: DefaultBuilder<T>) -> Self {
        SyncBuilder(Arc::new(Mutex::new(from)))
    }
}

impl<T> From<FnBuilder<T>> for SyncBuilder<T>
where
    T: 'static,
{
    fn from(from: FnBuilder<T>) -> Self {
        SyncBuilder(Arc::new(Mutex::new(from)))
    }
}

impl<T> Build for DefaultBuilder<T>
where
    T: Default,
{
    type Out = T;

    fn build(&self) -> T {
        T::default()
    }
}

impl<T> Build for FnBuilder<T> {
    type Out = T;

    fn build(&self) -> T {
        (self.0)()
    }
}

impl<T> Build for SyncBuilder<T> {
    type Out = T;

    fn build(&self) -> T {
        let inner = self.0.lock().unwrap();
        inner.build()
    }
}
