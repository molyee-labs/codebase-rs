use std::marker::PhantomData;
use std::mem::{size_of};
use std::slice;
use std::alloc::{alloc, dealloc, Layout, System};
use std::convert::TryInto;
use std::ptr;
use std::sync::atomic::AtomicPtr;
use std::mem;

const HEADER_SIZE: usize = size_of::<Header>();
const FLAGS_BITS: usize = 3; // number of flags
const FLAGS_SHIFT: usize = 8 * HEADER_SIZE - FLAGS_BITS;
const VALUE_MASK: u8 = 1 << FLAGS_SHIFT;
const CHILD_MASK: u8 = 1 << (FLAGS_SHIFT + 1);
const NEIGHBOR_MASK: u8 = 1 << (FLAGS_SHIFT + 2);
const MAX_LABEL_LEN: usize = 2 ^ FLAGS_SHIFT - 1;
const LABEL_LEN_MASK: u8 = MAX_LABEL_LEN as u8;
const LABEL_OFFSET: isize = HEADER_SIZE as isize; // after lock and header

#[repr(align(64))] // same as cache line size for most of CPUs
pub(crate) struct Node<T> {
    header: Header,
    child: AtomicPtr<Node<T>>,
    neighbor: AtomicPtr<Node<T>>,
    value: Option<T>,
    label: [u8; MAX_LABEL_LEN],
}

#[inline]
fn to_atomic_ptr<T>(src: Option<&T>) -> AtomicPtr<T> {
    let ptr = src.map_or(ptr::null_mut(), to_mut_ptr);
    AtomicPtr::new(ptr)
}

#[inline]
fn to_mut_ptr<T>(target: &T) -> *mut T {
    unsafe { target as *const T as *mut T }
}

impl<T> Node<T> {
    const ALIGN: usize = 64;
    const SIZE: usize = size_of::<Self>();

    unsafe fn new(
        label: &[u8],
        child: Option<&Self>,
        neighbor: Option<&Self>,
        value: Option<T>
    ) -> Self {
        debug_assert!(Self::SIZE == Self::ALIGN);
        let has_child = child.is_some();
        let has_neighbor = neighbor.is_some();
        let has_value = value.is_some();
        let label_len = label.len();
        let header = Header::new(label_len, has_child, has_neighbor, has_value);
        let child = to_atomic_ptr(child);
        let neighbor = to_atomic_ptr(neighbor);
        let mut node =
            Node { header, child, neighbor, value, label: mem::uninitialized() };
        let label_ptr = unsafe { (&mut node.label).as_ptr() as *mut u8 };
        ptr::copy_nonoverlapping(label.as_ptr(), label_ptr, label_len);
        node
    }

    pub(crate) fn root() -> Self {
        Node::new(b"", None, None, None)
    }
}

// flags and label len info
struct Header {
    inner: u8,
}

impl Header {
    unsafe fn from_raw(src: u8) -> Self {
        Header { inner: src }
    }

    fn new(
        label_len: usize,
        has_child: bool,
        has_neighbor: bool,
        has_value: bool
    ) -> Self {
        debug_assert!(label_len <= MAX_LABEL_LEN);
        let flag_bits = (has_value as u8) << FLAGS_SHIFT
            | (has_child as u8) << (FLAGS_SHIFT + 1)
            | (has_neighbor as u8) << (FLAGS_SHIFT + 2);
        let inner = flag_bits | (label_len as u8);
        Header { inner }
    }

    fn label_len(&self) -> usize {
        (self.inner & LABEL_LEN_MASK) as usize
    }

    fn has_child(&self) -> bool {
        self.inner & CHILD_MASK == CHILD_MASK
    }

    fn has_neighbor(&self) -> bool {
        self.inner & NEIGHBOR_MASK == NEIGHBOR_MASK
    }

    fn has_value(&self) -> bool {
        self.inner & VALUE_MASK == VALUE_MASK
    }
}


