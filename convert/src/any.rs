use core::any::Any;

pub trait IntoAny {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
