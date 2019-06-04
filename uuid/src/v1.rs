use super::*;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidV1(u128);

pub fn new() -> impl Uuid {
    UuidV1::new()
}

impl UuidV1 {
    pub(crate) fn new() -> Self {
        unimplemented!()
    }
}

impl Uuid for UuidV1 {
    fn version(&self) -> Version {
        Version::MAC
    }

    fn variant(&self) -> Variant {
        Variant::RFC4122
    }
}
