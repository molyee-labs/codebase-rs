use std::marker::PhantomData;
use std::io::{Bytes, Cursor, SeekFrom};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::{size_of, size_of_val, transmute};
use std::cell::RefCell;

pub struct Trie<T> {
    root: Node<T>,
    len: usize,
}

bitflags! {
    struct Has: u8 {
        const Value = 1;
        const Child = 2;
        const Neighbor = 4;
    }
}

pub struct Node<T> {
    // 3bits - flags
    // 5bits - label length
    // [u8; label_len] - label 32 symbols
    // ~ [u8, sizeof(Node)] - neighbor node
    // ~ [u8, sizeof(Node)] - child node
    // ~ [u8, sizeof(T)] - data (value)
    ptr: AtomicPtr<u8>,
    _val: PhantomData<T>,
}

pub struct Info<'a, T> {
    flags: Has,
    label: Bytes,
    value: Option(T),
    neighbor: Option<Node>,
    child: Option<Node>,
}

impl Info<T> {
    fn new(flags: Has, label: Bytes) -> Self {
        Info { flags, label, neighbor: None, child: None }
    }
}

pub struct Reader<T> {
    pos: RefCell<Position>,
    cursor: Cursor<u8>,
    _val: PhantomData<T>,
}

pub enum Position<T> {
    Start,
    BeforeLabel {flags: Has, label_len: u16},
    Label {flags: Has, label: Bytes, label_len: u16},
    BeforeNeighbor {flags: Has},
    Neighbor {flags: Has, node: Node},
    AfterNeighbor {flags: Has},
    BeforeChild {flags: Has},
    Child {flags: Has, node: Node},
    AfterChild {flags: Has},
    BeforeValue {flags: Has},
    Value {flags: Has, value: T},
    End,
}

impl Reader<T> {
    #[inline]
    pub fn new(ptr: *mut u8) {
        Reader {
            position: RefCell::new(Position::Start),
            cursor: Cursor::new(ptr)
        }
    }

    pub fn info(&self) -> Info<T> {
        let (flags, label_len) = &self.read_flags_and_label_len();
        let mut offset = 2;
        let label = read_label(label_len, offset);
        let mut info = Info::new(flags, label);
        if flags.contains(Has::Neighbor) {
            info.neighbor = Some(&self.read_node(offset += label_len));
        }
        if flags.contains(Has::Child) {
            info.child = Some(&self.read_node(offset += label_len));
        }
        if flags.contains(Has::Value) {
            info.value = Some(&self.read);
        }
        info
    }

    pub fn label(&self) -> Bytes {

    }

    #[inline]
    fn read_flags_and_label_len(&self) -> (Has, u16) {
        let mut flags_and_label_len: [u8; 2] = self.cursor.bytes().take(2);
        let flags = Has::from_bits(flags_and_label_len[0] >> 1);
        flags_and_label_len[0] &= 0x1fu8;
        let label_len = u16::from_be_bytes(flags_and_label_len);
        (flags, label_len)
    }

    #[inline]
    fn read_label(&self, len: u16, offset: isize) -> Bytes {
        let label = self.cursor.seek(SeekFrom::Start(offset)).bytes().take(len);
        (label, offset + len)
    }

    #[inline]
    fn read_from<U>(&self, offset: isize) -> U {
        let len = size_of(U);
        let data = self.cursor.seek(SeekFrom::Start(offset)).bytes().take(len);
        unsafe { transmute(data) }
    }

    #[inline]
    fn pick<U>(&self) -> U {
        let len = size_of::<U>();
        let data = self.cursor.bytes().take(len);
        unsafe { transmute(data) }
    }

    #[inline]
    fn seek(&self, pos: Position, adjust: isize) -> Self {
        self.pos.replace(pos);
        self.cursor.seek(SeekFrom::Current(adjust));
    }

    #[inline]
    fn read_node(&self, offset: isize) -> Node {
        &self.read_from(offset)
    }

    #[inline]
    fn read_value(&self, offset: isize) -> T {
        &self.read_from(offset)
    }

    #[inline]
    fn reset(&self) -> Self {
        self.cursor.seek(SeekFrom::Start(0));
        self.pos.replace(Position::Start);
        self
    }
}

const NODE_SIZE: usize = size_of::<Node>;

impl Iterator for Reader {
    type Item = Position;

    fn next(&self) -> Option<Self::Item> {
        use Position::*;

        match self.pos {
            Start => {
                let (flags, label_len) = &self.read_flags_and_label_len();
                &self.seek(BeforeLabel(flags, label_len), 2).pos
            },
            BeforeLabel(flags, label_len) => {
                let label = &self.cursor.bytes();
                &self.seek(Label(flags, label, label_len), label_len).pos
            },
            Label(flags, label, offset) => {
                if flags.contains(Has::Neighbor) {
                    &self.seek(BeforeNeighbor(flags), 0);
                } else {
                    &self.seek(AfterNeighbor(flags), 0);
                }
                next()
            },
            BeforeNeighbor(flags) => {
                &self.seek(Neighbor(flags, &self.pick()), NODE_SIZE).pos
            },
            Neighbor(flags, node) => {
                &self.seek(AfterNeighbor(flags), 0);
                next()
            },
            AfterNeighbor(flags) => {
                if flags.contains(Has::Child) {
                    &self.seek(BeforeChild(flags), 0);
                } else {
                    &self.seek(AfterChild(flags), 0);
                }
                next()
                
            },
            BeforeChild(flags) => {
                &self.seek(Child(flags, &self.pick()), NODE_SIZE).pos
            },
            Child(flags, node) => {
                &self.seek(AfterChild(flags), 0);
                next()
            },
            AfterChild(flags) => {
                if flags.contains(Has::Value) {
                    &self.seek(BeforeValue(flags), 0);
                } else {
                    &self.seek(End, 0).pos
                }
                next()
            },
            BeforeValue(flags) => {
                &self.seek(Value(flags, &self.pick()), NODE_SIZE).pos
            },
            Value(flags, value) => {
                &self.seek(End, 0);
                next()
            },
        }        
    }
}

impl Node<T> {
    fn open(&self) -> Reader {
        let ptr = self.ptr.load(Ordering::SeqCst);
        Reader::new(ptr)
    }
}