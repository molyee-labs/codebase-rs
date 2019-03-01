use super::*;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidV1(uuid::Uuid);

pub fn new() -> impl Uuid {
    UuidV1::new()
}

impl UuidV1 {
    pub(crate) fn new() -> Self {
        UuidV1(uuid::Uuid::new_v1())
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
