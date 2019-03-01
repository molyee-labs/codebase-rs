use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Version {
    NIL = 0,
    MAC,
    DCE,
    MD5,
    RANDOM,
    SHA1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Variant {
    NCS = 0,
    RFC4122,
    Microsoft,
    Future,
}

pub trait Uuid: Sized + Clone + Copy + Eq + Send + Eq + PartialEq + Hash + Ord {
    fn new() -> Self;
    fn version(&self) -> Version;
    fn variant(&self) -> Variant;
}

#[cfg(feature = "v1")]
pub mod v1;

#[cfg(feature = "v4")]
pub mod v4;
