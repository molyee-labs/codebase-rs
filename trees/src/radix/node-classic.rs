use std::marker::PhantomData;
use std::mem::{size_of};
use std::slice;
use std::alloc::{alloc, dealloc, Layout, System, Alloc};
use std::convert::TryInto;


bitflags! {
    pub(crate) struct Flag: u8 {
        const VALUE = 1;
        const CHILD = 2;
        const NEIGHBOR = 4;
    }
}

const HEADER_SIZE: usize = 1; // flags and label len info
const FLAGS_BITS: usize = 3; // number of flags
const FLAGS_SHIFT: usize = 8 * HEADER_SIZE - FLAGS_BITS;
const MAX_LABEL_LEN: usize = 2 ^ FLAGS_SHIFT - 1;
const LABEL_LEN_MASK: u8 = MAX_LABEL_LEN as u8;
const LABEL_OFFSET: isize = HEADER_SIZE as isize; // after lock and header
const ALLOC_ALIGN: usize = 1;

#[repr(C)]
pub(crate) struct Node<T, G> {
    // ?bits - Gate
    // ?bits - Header
    // [u8; label_len] - label max 31 symbols
    // opt [u8, sizeof(T)] - data (value)
    // opt [u8, sizeof(Node)] - child node
    // opt [u8, sizeof(Node)] - neighbor node
    ptr: *const G,
    _m: PhantomData<(G, T)>,
}

struct Header {
    inner: u8,
}

impl Header {
    fn new(flags: Flag, label_len: usize) -> Self {
        debug_assert!(label_len <= MAX_LABEL_LEN);
        let inner = flags.bits() << FLAGS_SHIFT | (label_len as u8);
        Header { inner }
    }

    fn flags(&self) -> Flag {
        let flags = self.inner >> FLAGS_SHIFT;
        Flag::from_bits(flags).unwrap()
    }

    fn label_len(&self) -> usize {
        (self.inner & LABEL_LEN_MASK) as usize
    }

    fn has_child(&self) -> bool {
        self.flags().contains(Flag::CHILD)
    }

    fn has_neighbor(&self) -> bool {
        self.flags().contains(Flag::NEIGHBOR)
    }

    fn has_value(&self) -> bool {
        self.flags().contains(Flag::VALUE)
    }

    fn calc_node_size<T, G>(&self) -> usize {
        let flags = self.flags();
        let mut size = size_of::<G>() + HEADER_SIZE;
        if flags.contains(Flag::NEIGHBOR) {
            size += size_of::<Node<T, G>>();
        }
        if flags.contains(Flag::CHILD) {
            size += size_of::<Node<T, G>>();
        }
        if flags.contains(Flag::VALUE) {
            size += size_of::<T>();
        }
        size
    }
}

impl<T, G> Node<T, G> {
    #[inline]
    unsafe fn from_raw(ptr: *const G) -> Self {
        Node { ptr, _m: Default::default() }
    }

    #[inline]
    unsafe fn offset<U>(&self, offset: isize) -> *mut U {
        self.ptr.offset(offset) as *mut U
    }

    #[inline]
    unsafe fn offset_as_ref<U>(&self, offset: isize) -> &U {
        &*self.offset(offset)
    }

    #[inline]
    pub(crate) unsafe fn new(
        label: &[u8],
        value: Option<T>,
        child: Option<Self>,
        neighbor: Option<Self>
    ) -> Self {
        debug_assert!(label.len() <= MAX_LABEL_LEN);

    }

    #[inline]
    pub(crate) fn gate(&self) -> &G {
        unsafe { &*(self.ptr as *const G) }
    }

    #[inline]
    pub(crate) fn header(&self) -> &Header {
        unsafe { self.offset_as_ref(self.header_offset()) }
    }

    #[inline]
    pub(crate) fn size(&self) -> usize {
        self.header().calc_node_size::<T, G>()
    }

    #[inline]
    pub(crate) fn label(&self) -> &[u8] {
        let len = self.header().label_len();
        unsafe {
            let ptr = self.offset(self.label_offset());
            slice::from_raw_parts(ptr, len)
        }
    }

    #[inline]
    pub(crate) fn value(&self) -> Option<&T> {
        if self.header().has_value() {
            Some(unsafe { self.offset_as_ref(self.value_offset()) })
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn neighbor(&self) -> Option<&Self> {
        if self.header().has_neighbor() {
            Some(unsafe { self.offset_as_ref(self.neighbor_offset()) })
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn child(&self) -> Option<&Self> {
        if self.header().has_child() {
            Some(unsafe { self.offset_as_ref(self.child_offset()) })
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn replace_neighbor(&mut self, new: Self) -> Option<Self> {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn replace_child(&mut self, new: Self) -> Option<Self> {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn replace_value(&mut self, new: T) -> Option<T> {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn remove_value(&mut self) -> Option<T> {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn remove_child(&mut self) -> Option<Self> {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn remove_neighbor(&mut self) -> Option<Self> {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn replace(builder: &Builder<'_, T, G>) -> (usize, usize) {
        unimplemented!()
    }

    #[inline]
    pub(crate) fn info(&self) -> Info<'_, T, G> {
        let header = self.header();
        let label_len = header.label_len();
        let label_offset = self.label_offset();
        let mut ptr = self.ptr.offset(label_offset) as *mut u8;
        let label = slice::from_raw_parts(ptr, label_len);
        let value = match header.has_value() {
            false => None,
            true => {
                ptr = ptr.offset(size_of::<T>() as isize);
                Some(unsafe { &*(ptr as *mut T as *const T) })
            }
        };
        let child = match header.has_child() {
            false => None,
            true => {
                ptr = ptr.offset(size_of::<Self>() as isize);
                Some(unsafe { &*(ptr as *mut Self as *const Self) })
            }
        };
        let neighbor = match header.has_neighbor() {
            false => None,
            true => {
                ptr = ptr.offset(size_of::<Self>() as isize);
                Some(unsafe { &*(ptr as *mut Self as *const Self) })
            }
        };
        Info { label, value, child, neighbor, _m: Default::default() }
    }

    #[inline]
    fn header_offset(&self) -> isize {
        size_of::<G>() as isize
    }

    #[inline]
    fn label_offset(&self) -> isize {
        self.header_offset().wrapping_add(HEADER_SIZE as isize)
    }

    #[inline]
    fn value_offset(&self) -> isize {
        let label_len = self.header().label_len() as isize;
        self.label_offset().wrapping_add(label_len)
    }

    #[inline]
    fn child_offset(&self) -> isize {
        if self.header().has_value() {
            self.value_offset().wrapping_add(size_of::<T>() as isize)
        } else {
            self.value_offset()
        }
    }

    #[inline]
    fn neighbor_offset(&self, ) -> isize {
        if self.header().has_child() {
            self.child_offset().wrapping_add(size_of::<G>() as isize)
        } else {
            self.child_offset()
        }
    }
}

pub(crate) struct Info<'a, T, G> {
    label: &'a [u8],
    value: Option<&'a T>,
    child: Option<&'a Node<T, G>>,
    neighbor: Option<&'a Node<T, G>>,
    _m: PhantomData<(T, G)>
}

pub(crate) struct Builder<'a, T, G> {
    label: &'a [u8],
    value: Option<T>,
    child: Option<Node<T, G>>,
    neighbor: Option<Node<T, G>>,
    alloc: Option<&'a Alloc>,
}

impl<'a, T, G> Builder<'a, T, G> {
    #[inline]
    fn new<'b: 'a, 'c: 'a>(label: &'b [u8], alloc: Option<&'c Alloc>) -> Self {
        let alloc = alloc.get_or_insert_with(|| System);
        Builder { label, value: None, child: None, neighbor: None, alloc }
    }

    #[inline]
    fn set_value(self, value: Option<T>) -> Self {
        self.value = value;
        self
    }

    #[inline]
    fn set_child(self, node: Option<Node<T, G>>) -> Self {
        self.child = node;
        self
    }

    #[inline]
    fn set_neighbor(self, node: Option<Node<T, G>>) -> Self {
        self.neighbor = node;
        self
    }
}

impl<T, G> From<Builder<'_, T, G>> for Node<T, G> {
    fn from(b: Builder<'_, T, G>) -> Self {
        Node::new(b.label, b.value, b.child, b.neighbor)
    }
}
