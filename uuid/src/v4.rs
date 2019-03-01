use super::*;
use rand::RngCore;

const VERSION_AND_VARIANT_BITS : u128 = 0x4u128 << 68 | 0x80u128 << 56;
const RANDOM_MASK : u128 = 0xffffffffffffff0f3fffffffffffffff;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidV4(u128);

pub fn new() -> impl Uuid {
    UuidV4::new()
}

impl Uuid for UuidV4 {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill_bytes(bytes.as_mut());
        let raw = u128::from_ne_bytes(bytes);
        let data = raw & RANDOM_MASK | VERSION_AND_VARIANT_BITS;
        UuidV4(data)
    }

    fn version(&self) -> Version {
        Version::RANDOM
    }

    fn variant(&self) -> Variant {
        Variant::RFC4122
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_uuids() -> Vec<UuidV4> {
        let mut arr = vec![];
        for _ in 0..10 {
            arr.push(UuidV4::new());
        }
        arr.push(UuidV4(0x550e8400e29b41d4a716446655440000u128));
        arr.push(UuidV4(0x67e5504410b1426f9247bb680e5fe0c8u128));
        arr
    }

    #[test]
    fn check_variant() {
        for &uuid in generate_uuids().iter() {
            assert_eq!(uuid.0.to_ne_bytes()[8] >> 6, 0b10u8);
            assert_eq!(uuid.variant(), Variant::RFC4122);
        }
    }

    #[test]
    fn check_version() {
        for &uuid in generate_uuids().iter() {
            assert_eq!(uuid.0.to_ne_bytes()[7] >> 4, 0b0100u8);
            assert_eq!(uuid.version(), Version::RANDOM);
        }
    }
}