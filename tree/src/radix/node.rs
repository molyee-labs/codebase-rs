use std::marker::PhantomData;
use std::io::{Bytes, Cursor, SeekFrom};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::{size_of, size_of_val, transmute, forget};
use std::alloc::{alloc, dealloc, Layout};

bitflags! {
    pub(crate) struct Flag: u8 {
        const VALUE = 1;
        const CHILD = 2;
        const NEIGHBOR = 4;
    }
}

const FLAGS_BITS: usize = 3; // number of flags
const FLAGS_SHIFT: usize = 8 - FLAGS_BITS;
const MAX_LABEL_LEN: usize = 2 ^ FLAGS_SHIFT - 1;
const LABEL_LEN_MASK: u8 = MAX_LABEL_LEN as u8;
const ALLOC_ALIGN: usize = 1;

pub struct Node<T> {
    // 3bits - flags
    // 5bits - label length
    // [u8; label_len] - label max 31 symbols
    // ~ [u8, sizeof(Node)] - neighbor node
    // ~ [u8, sizeof(Node)] - child node
    // ~ [u8, sizeof(T)] - data (value)
    ptr: AtomicPtr<u8>,
    _val: PhantomData<T>,
}

impl Flag {
    #[inline]
    pub(crate) fn calc_node_size<T>(&self) -> usize {
        let mut size = 1usize;
        if self.contains(Flag::NEIGHBOR) {
            size += size_of::<Node<T>>();
        }
        if self.contains(Flag::CHILD) {
            size += size_of::<Node<T>>();
        }
        if self.contains(Flag::VALUE) {
            size += size_of::<T>();
        }
        size
    }
}

impl<T> Node<T> {
    pub(crate) fn new(
        label: &[u8],
        neighbor: Option<Node<T>>,
        child: Option<Node<T>>,
        value: Option<T>
    ) -> Self {
        let label_len = label.len();
        assert!(label_len <= MAX_LABEL_LEN);
        let mut flags = Flag::empty();
        flags.set(Flag::NEIGHBOR, neighbor.is_some());
        flags.set(Flag::CHILD, child.is_some());
        flags.set(Flag::VALUE, value.is_some());
        let size = flags.calc_node_size::<T>();
        let layout = Layout::from_size_align(size, ALLOC_ALIGN).unwrap();
        let ptr = unsafe { alloc(layout) };
        assert!(!ptr.is_null());

        let flags_and_label_len = (flags.bits() << FLAGS_SHIFT) + (label_len as u8);
        unsafe {
            let pos = pointer::write_next(ptr, flags_and_label_len);
            let pos = pointer::copy_next(label.as_ptr(), pos, label_len);
            let pos = pointer::try_write_next(pos, neighbor);
            let pos = pointer::try_write_next(pos, child);
            forget(pointer::try_write_next(pos, value) as *mut u8);
        }
        
        let atomic = AtomicPtr::new(ptr);
        Node { ptr: atomic, _val: Default::default() }
    }

    pub(crate) fn root() -> Self {
        Node::new(b"", None, None, None)
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr.load(Ordering::SeqCst)
    }

    pub(crate) fn open(&self) -> Reader<T> {
        Reader::new(&self)
    }

    pub(crate) fn size(&self) -> usize {
        self.flags().calc_node_size::<T>()
    }

    pub(crate) fn flags(&self) -> Flag {
        let flags = unsafe { *(self.as_mut_ptr()) >> FLAGS_SHIFT };
        Flag::from_bits(flags).unwrap()
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let size = self.size();
        let layout = unsafe {
            Layout::from_size_align_unchecked(size, ALLOC_ALIGN)
        };
        unsafe { dealloc(self.as_mut_ptr(), layout) };
    }
}

pub struct Reader<'a, T> {
    node: &'a Node<T>,
}

impl<'a, T> Reader<'a, T> {
    pub fn new(node: &'a Node<T>) -> Self {
        Reader { node }
    }


}

/*impl Reader<T> {
    #[inline]
    pub fn new(ptr: *u8) {
        Reader { ptr }
    }

    pub fn iter(&self) -> Iter<T> {
        let cursor = Cursor::new(self.ptr)
        Iter { cursor }
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
}*/


/*impl Iterator for Reader {
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
} */
