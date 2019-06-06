use convert::*;

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

impl_transmute!(u8, Version);
impl_transmute!(u8, Variant);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Variant {
    NCS = 0,
    RFC4122,
    Microsoft,
    Future,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Uuid([u8; 16]);

impl Uuid {
    pub fn bytes(&self) -> [u8; 16] {
        self.0
    }
}

pub trait Specs {
    fn version(&self) -> Version;
    fn variant(&self) -> Variant;
}

impl Specs for Uuid {
    fn version(&self) -> Version {
        let version: u8 = self.0[6] >> 4;
        if version > 5u8 {
            Version::NIL
        } else {
            version.transmute()
        }
    }

    fn variant(&self) -> Variant {
        let variant: u8 = self.0[8] >> 5;
        match variant {
            6 => Variant::Microsoft,
            7 => Variant::Future,
            4 | 5 => Variant::RFC4122,
            _ => Variant::NCS,
        }
    }
}

#[cfg(feature = "v1")]
pub mod v1;

#[cfg(feature = "v4")]
pub mod v4;
