pub trait Transmute<T> {
    fn transmute(self) -> T;
}

#[macro_export]
macro_rules! impl_transmute {
    ($t1:ty, $t2:ty) => {
        impl Transmute<$t2> for $t1 {
            #[inline]
            fn transmute(self) -> $t2 {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
}