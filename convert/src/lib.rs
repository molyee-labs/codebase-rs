#[cfg(feature="integer")]
pub mod integer;
#[cfg(feature="integer")]
pub use integer::*;

pub trait Transmute<T> {
    fn transmute(self) -> T;
}

#[macro_export]
macro_rules! impl_transmute {
    ($t1:ty, $t2:ty) => {
        impl Transmute<$t2> for $t1 {
            fn transmute(self) -> $t2 {
                unsafe { mem::transmute(self) }
            }
        }
    };
}
