use core::any::Any;

pub trait IntoAny: 'static + Sized {
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl<T: 'static> IntoAny for T { }
