use std::marker::PhantomData;
use std::mem::{size_of, uninitialized};
use std::slice;
use std::alloc::{alloc, dealloc, Layout, System, Alloc};
use std::convert::TryInto;
use std::sync::atomic::AtomicPtr;

const CHILD: u16 = 1 << FLAGS_SHIFT;
const NEIGHBOR: u16 = 2 << FLAGS_SHIFT;
const VALUE: u16 = 4 << FLAGS_SHIFT;
const DELETED: u16 = 8 << FLAGS_SHIFT;

const HEADER_SIZE: usize = size_of::<Header>();
const FLAGS_NUMBER: usize = 4;
const FLAGS_SHIFT: usize = HEADER_SIZE * 8 - FLAGS_NUMBER; // bits;
const MAX_LABEL_LEN: usize = 2.pow(HEADER_SIZE - FLAGS_NUMBER) - 1;
const LABEL_LEN_MASK: u16 = MAX_LABEL_LEN as u16;

const CACHE_LINE_SIZE: usize = 64;

// 2bytes - Header,
// usize
#[repr(C)]
pub(crate) struct Node<T> {
    p: AtomicPtr<[u8; CACHE_LINE_SIZE]>,
    _m: PhantomData<T>
}

struct Header {
    inner: u16,
}

impl Header {
    #[inline]
    unsafe fn from_raw(src: u16) -> Self {
        Header { inner: src }
    }

    #[inline]
    fn new(
        has_child: bool,
        has_neighbor: bool,
        has_value: bool,
        label_len: usize
    ) -> Self {
        debug_assert!(label_len <= MAX_LABEL_LEN);
        let inner = label_len as u16
            | CHILD * (has_child as u16)
            | NEIGHBOR * (has_neighbor as u16)
            | VALUE * (has_value as u16);
        Header { inner }
    }

    #[inline]
    fn label_len(&self) -> usize {
        (self.inner & LABEL_LEN_MASK) as usize
    }

    #[inline]
    fn has_child(&self) -> bool {
        self.inner & CHILD == CHILD;
    }

    #[inline]
    fn has_neighbor(&self) -> bool {
        self.inner & NEIGHBOR == NEIGHBOR
    }

    #[inline]
    fn has_value(&self) -> bool {
        self.inner & VALUE == VALUE
    }
}

impl<T> Node<T> {
    const ALIGN: usize = CACHE_LINE_SIZE;
    const SIZE: usize = ALIGN;
    const SIZE_WITHOUT_LABEL: usize = HEADER_SIZE
        + size_of::<T>()
        + 2 * size_of::<Self>();
    const MAX_LABEL_SIZE: usize = SIZE - SIZE_WITHOUT_LABEL;

    #[inline]
    unsafe fn from_raw(p: *const u8) -> Self {
        Node { p: AtomicPtr::new(p), _m: Default::default() }
    }

    #[inline]
    unsafe fn offset<U>(&self, offset: isize) -> *mut U {
        self.p.offset(offset) as *mut U
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
        debug_assert!(label.len() <= Self::MAX_LABEL_SIZE);
        debug_assert!(Self::MAX_LABEL_SIZE <= MAX_LABEL_LEN);
        let mut buf: [u8; 64] = mem::uninitialized();
        let has_child = child.is_some();
        let has_neighbor = neighbor.is_some();
        let has_value = value.is_some();
        let label_len = label.len();
        let header = Header::new(has_child, has_neighbor, has_value, label_len);

    }

    #[inline]
    fn header(&self) -> &Header {
        unsafe { &*self.p }
    }

    #[inline]
    fn size(&self) -> usize {
        SIZE
    }

    #[inline]
    fn label(&self) -> &[u8] {
        let len = self.header().label_len();
        unsafe {
            let ptr = self.offset(self.label_offset());
            slice::from_raw_parts(ptr, len)
        }
    }

    #[inline]
    fn value(&self) -> Option<&T> {
        if self.header().has_value() {
            Some(unsafe { self.offset_as_ref(self.value_offset()) })
        } else {
            None
        }
    }

    #[inline]
    fn neighbor(&self) -> Option<&Self> {
        if self.header().has_neighbor() {
            Some(unsafe { self.offset_as_ref(self.neighbor_offset()) })
        } else {
            None
        }
    }

    #[inline]
    fn child(&self) -> Option<&Self> {
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
    pub(crate) fn replace(b: &Builder<'_, T, G>) {
        unimplemented!()
    }

    #[inline]
    fn info(&self) -> Info<'_, T, G> {
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
    fn label_offset(&self) -> isize {
        HEADER_SIZE as isize
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
    fn neighbor_offset(&self) -> isize {

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
