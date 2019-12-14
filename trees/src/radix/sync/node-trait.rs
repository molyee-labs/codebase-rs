pub struct Key {
    key: u8
}

impl Key {
    const MIN: Key = Key::new(b'a');
    const MAX: Key = Key::new(b'z');
    const RANGE_LEN: usize = (MIN.key - MAX::key) as usize;

    const fn new(key: u8) -> Self {
        Key { key }
    }
}

bitflags! {
    struct Flag: u8 {
        const REMOVED = 0x80;
    }
}

struct Node<T> {
    inner: u64,
    mark: PhantomData<T>
}

struct Child<T> {
    head: Flag,
    key: Key,
    ptr: *mut Node<T>,
    mark: PhantomData<T>,
}

impl<T> Node<T> {
    const PTR_MASK: u64 = 0x0000FFFFFFFFFFFF;
    const KEY_MASK: u64 = 0x00FF000000000000;
    const KEY_SHIFT: usize = 48;
    const FLAGS: u64 = 0xFF00000000000000;
    const FLAGS_SHIFT: usize = 56;

    fn ptr(&self) -> *const u8 {
        (self.inner & PTR_MASK) as *const u8
    }

    fn bytes(&self) -> &[u8] {

    }

    unsafe fn get<D>(&self, offset: isize) -> &D {

    }

    fn child(&self, key: Key) -> Child<T> {

    }
}