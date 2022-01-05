use verify::*;
use core::fmt::{self, Debug, Display};

pub struct Email<S, V = Verifier>(S, V);

impl<S: ToOwned, V: Copy> ToOwned for Email<S, V> {
    type Owned = Email<S::Owned, V>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        Self(self.0.to_owned(), self.1)
    }
}

impl<S: Debug> Debug for Email<S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<S: Display> Display for Email<S> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct Error;

#[derive(Clone, Copy)]
pub struct Verifier;

// TODO use mailboxvalidator impl from crates.io
impl Verify for Verifier {
    type Obj = Email<String>;
    type Err = Error;
    #[inline]
    fn verify(&self, obj: &Self::Obj) -> Result<(), Self::Err> {
        todo!()
    }
}