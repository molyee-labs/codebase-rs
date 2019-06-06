use super::*;
use convert::*;
use std::mem;

// The number of 100 ns ticks between the UUID epoch
// `1582-10-15 00:00:00` and the Unix epoch `1970-01-01 00:00:00`.
const UUID_TICKS_BETWEEN_EPOCHS: u64 = 0x01b21dd213814000u64;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidV1([u8; 16]);

#[derive(Default)]
pub struct Time {
    hi: [u8; 2],
    mid: [u8; 2],
    low: [u8; 4],
}

impl_transmute!(u64, Time);

impl Time {
    pub fn with_unix_time(seconds: u64, nanos: u32) -> Self {
        let unix_time: u64 = seconds * 10000000 + u64::from(nanos / 100);
        let uuid_time = unix_time + UUID_TICKS_BETWEEN_EPOCHS;
        let time: Time = uuid_time.to_be().transmute();
        time
    }
}

#[derive(Default)]
pub struct NodeId([u8; 6]);

impl From<[u8; 6]> for NodeId {
    fn from(from: [u8; 6]) -> Self {
        NodeId(from)
    }
}

#[derive(Default)]
pub struct ClockSequence {
    hi_res: u8,
    low: u8,
}

impl_transmute!(u16, ClockSequence);

impl From<u16> for ClockSequence {
    fn from(from: u16) -> Self {
        let mut clk_seq: ClockSequence = from.to_be().transmute();
        clk_seq.hi_res = clk_seq.hi_res & 0x3fu8 | 0x80u8;
        clk_seq
    }
}

pub trait Context: Sync {
    fn next(&self) -> (Time, ClockSequence);
    fn node(&self) -> NodeId;
}

struct Inner {
    time_low: [u8; 4],
    time_mid: [u8; 2],
    time_hi_and_version: [u8; 2],
    clk_seq: ClockSequence,
    node: NodeId,
}

impl_transmute!(Inner, [u8; 16]);
impl_transmute!([u8; 16], Inner);

pub fn new<T: Context>(context: &T) -> impl Uuid {
    UuidV1::from(context)
}

impl<T: Context> From<&T> for UuidV1 {
    fn from(context: &T) -> Self {
        let (time, clk_seq) = context.next();
        let time_low = time.low;
        let time_mid = time.mid;
        let mut time_hi_and_version = time.hi;
        time_hi_and_version[0] = time.hi[0] & 0x0fu8 | 0x10u8;
        let node = context.node();
        let inner = Inner {
            time_low,
            time_mid,
            time_hi_and_version,
            clk_seq,
            node,
        };
        UuidV1(inner.transmute())
    }
}

impl Uuid for UuidV1 {
    fn bytes(&self) -> [u8; 16] {
        self.0
    }

    fn version(&self) -> Version {
        Version::MAC
    }

    fn variant(&self) -> Variant {
        Variant::RFC4122
    }
}

impl UuidV1 {
    fn inner(&self) -> Inner {
        self.0.transmute()
    }

    fn time(&self) -> u64 {
        let inner = self.inner();
        let mut time_hi = inner.time_hi_and_version;
        time_hi[0] &= 0x3fu8;
        let time = (time_hi, inner.time_mid, inner.time_low);
        unsafe { mem::transmute(time) }
    }

    fn node(&self) -> [u8; 6] {
        self.inner().node.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Data {
        uuid: u128,
        seconds: u64,
        nanos: u32,
        node: [u8; 6],
        count: u16,
    }

    const DATA: &[Data] = &[
        Data {
            uuid: 0x206169344ba211e78000010203040506u128,
            seconds: 1496854535u64,
            nanos: 812946000u32,
            node: [1, 2, 3, 4, 5, 6],
            count: 0u16,
        },
        Data {
            uuid: 0x206169344ba211e78001010203040506u128,
            seconds: 1496854535u64,
            nanos: 812946000u32,
            node: [1, 2, 3, 4, 5, 6],
            count: 1u16,
        },
    ];

    struct MockCounterContext {
        seconds: u64,
        nanos: u32,
        node: [u8; 6],
        count: AtomicUsize,
    }

    impl MockCounterContext {
        fn new(seconds: u64, nanos: u32, node: [u8; 6], count: u16) -> Self {
            let count = AtomicUsize::new(count as usize);
            MockCounterContext {
                seconds,
                nanos,
                node,
                count,
            }
        }
    }

    impl Context for MockCounterContext {
        fn next(&self) -> (Time, ClockSequence) {
            let time = Time::with_unix_time(self.seconds, self.nanos);
            let count = (self.count.fetch_add(1, Ordering::SeqCst) & 0xffff) as u16;
            let clk_seq = ClockSequence::from(count);
            (time, clk_seq)
        }

        fn node(&self) -> NodeId {
            NodeId(self.node)
        }
    }

    fn generate_uuids() -> Vec<UuidV1> {
        let mut arr = vec![];
        for i in DATA {
            let context = MockCounterContext::new(i.seconds, i.nanos, i.node, i.count);
            arr.push(UuidV1::from(&context));
        }
        arr
    }

    #[test]
    fn check_node_id() {
        for (i, uuid) in generate_uuids().iter().enumerate() {
            let node = uuid.node();
            let origin = DATA[i].node;
            assert_eq!(node, origin, "wrong node id");
        }
    }

    #[test]
    fn check_times() {
        for (i, uuid) in generate_uuids().iter().enumerate() {
            let time = uuid.time() - UUID_TICKS_BETWEEN_EPOCHS;
            let origin = DATA[i].seconds * 10000000 + (DATA[i].nanos as u64) / 100;
            assert_eq!(time, origin);
        }
    }

    #[test]
    fn check_struct_size() {
        for uuid in generate_uuids().iter() {
            let size = std::mem::size_of_val(uuid);
            assert_eq!(size, 16, "wrong uuid data size");
        }
    }

    #[test]
    fn check_bytes_ordering() {
        let count = DATA.len();
        for (i, uuid) in generate_uuids().iter().take(count).enumerate() {
            let bytes = uuid.bytes();
            let origin: [u8; 16] = DATA[i].uuid.to_be().transmute();
            assert_eq!(bytes, origin, "wrong bytes ordering");
        }
    }

    #[test]
    fn check_variant() {
        for uuid in generate_uuids().iter() {
            // 10x high 2 bits of 8 octet
            let byte = uuid.bytes()[8];
            assert_eq!(byte >> 6, 2u8,
                "{:?} wrong variant in 2 most significant bits of 8 octet {:#x?} 'clk_seq_hi_res' field",
                uuid, byte);
            assert_eq!(uuid.variant(), Variant::RFC4122);
        }
    }

    #[test]
    fn check_version() {
        for uuid in generate_uuids().iter() {
            // 0100 high 4 bits of 6 octet
            let byte = uuid.bytes()[6];
            assert_eq!(byte & 0xf0u8, 0x10u8,
                "{:?} wrong version in 4 most significant bits of 6 octet {:#x?} 'time_hi_and_version' field",
                uuid, byte);
            assert_eq!(uuid.version(), Version::MAC);
        }
    }
}
