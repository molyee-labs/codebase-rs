use super::*;
use std::fmt;
use rand::RngCore;

const VERSION_AND_VARIANT_BITS : u128 = 0x40u128 << 72 | 0x80u128 << 56;
const RANDOM_MASK : u128 = 0xffffffffffff0fff3fffffffffffffff;

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

impl fmt::Debug for UuidV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uuid'{:#x?}'", &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_uuids() -> Vec<UuidV4> {
        let mut arr = vec![];
        arr.push(UuidV4(0x434511c9932b4b58a0f9cedf28d44a35u128));
        arr.push(UuidV4(0xb8a6077771f64ba09216fd6059395cf9u128));
        arr.push(UuidV4(0x550e8400e29b41d4a716446655440000u128));
        arr.push(UuidV4(0x67e5504410b1426f9247bb680e5fe0c8u128));
        for _ in 0..10 {
            arr.push(UuidV4::new());
        }
        arr
    }

    #[test]
    fn check_variant() {
        for &uuid in generate_uuids().iter() {
            // 10x high 2 bits of 8 octet
            let byte = uuid.0.to_be_bytes()[8];
            assert_eq!(byte >> 6, 2u8,
                "{:?} wrong variant in 2 most significant bits of 8 octet {:#x?} 'clk_seq_hi_res' field",
                uuid, byte);
            assert_eq!(uuid.variant(), Variant::RFC4122);
        }
    }

    #[test]
    fn check_version() {
        for &uuid in generate_uuids().iter() {
            // 0100 high 4 bits of 6 octet
            let byte = uuid.0.to_be_bytes()[6];
            assert_eq!(byte >> 4, 0x4u8,
                "{:?} wrong version in 4 most significant bits of 6 octet {:#x?} 'time_hi_and_version' field",
                uuid, byte);
            assert_eq!(uuid.version(), Version::RANDOM);
        }
    }
}