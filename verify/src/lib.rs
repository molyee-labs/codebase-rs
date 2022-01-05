use core::result::Result;

pub trait Check<V: Verify> {
    fn check(&self, v: V) -> Result<(), V::Err>;
}

impl<T, V: Verify<Obj = T>> Check<V> for T {
    #[inline]
    fn check(&self, v: V) -> Result<(), V::Err> {
        v.verify(self)
    }
}

pub trait Verify {
    type Obj;
    type Err;
    fn verify(&self, obj: &Self::Obj) -> Result<(), Self::Err>;
}
