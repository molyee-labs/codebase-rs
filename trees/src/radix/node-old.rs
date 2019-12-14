use std::marker::PhantomData;
use std::mem::{size_of};
use std::slice;
use super::Node;
use super::access::{Gate, Access};
use std::alloc::{alloc, dealloc, Layout};


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
#[derive(Default)]
pub(crate) struct Single<T, G: Gate> {
    gate: G,
    // 3bits - flags
    // 5bits - label length
    // [u8; label_len] - label max 31 symbols
    // ~ [u8, sizeof(T)] - data (value)
    // ~ [u8, sizeof(Node)] - child node
    // ~ [u8, sizeof(Node)] - neighbor node
    head: u8,
    _m: PhantomData<T>,
}

impl Flag {
    #[inline]
    pub(crate) fn calc_node_size<N: Node>(&self) -> usize {
        let mut size = HEADER_SIZE;
        if self.contains(Flag::NEIGHBOR) {
            size += size_of::<N::Neighbor>();
        }
        if self.contains(Flag::CHILD) {
            size += size_of::<N::Child>();
        }
        if self.contains(Flag::VALUE) {
            size += size_of::<N::Value>();
        }
        size
    }
}

impl<T, G: Gate<Target=Self>> Node for Single<T, G> {
    type Value = T;
    type Neighbor = G;
    type Child = G;

    #[inline]
    fn size(&self) -> usize {
        self.flags().calc_node_size::<Self>()
    }

    #[inline]
    fn label(&self) -> &[u8] {
        let len = self.label_len();
        unsafe {
            let data = self as *const Self as *const u8;
            slice::from_raw_parts(data, len)
        }
    }

    fn value(&self) -> Option<&T> {
        let flags = self.flags();
        if flags.contains(Flag::VALUE) {
            Some(self.move_to(self.value_offset(flags)))
        } else {
            None
        }
    }

    fn neighbor(&self) -> Option<&G> {
        let flags = self.flags();
        if flags.contains(Flag::NEIGHBOR) {
            Some(self.move_to(self.neighbor_offset()))
        } else {
            None
        }
    }

    fn child(&self) -> Option<&G> {
        let flags = self.flags();
        if flags.contains(Flag::CHILD) {
            Some(self.move_to(self.child_offset(flags)))
        } else {
            None
        }
    }

    fn replace_neighbor(new: Option<&mut Self::Neighbor>) -> Option<&mut Self::Neighbor> {
        unimplemented!()
    }

    fn replace_child(new: Option<&mut Self::Child>) -> Option<&mut Self::Child> {
        unimplemented!()
    }

    fn replace_value(new: Option<Self::Value>) -> Option<Self::Value> {
        unimplemented!()
    }
}

/*impl<T, G: Gate<Target=Self>> Drop for Single<T, G> {
    fn drop(&mut self) {
        let size = self.size();
        let align = self.align();
        let layout = Layout::from_size_align_unchecked(size, align);
        dealloc(self.as_mut_ptr() as *mut u8, layout);
    }
}*/

impl<T, G: Gate<Target=Self>> Single<T, G> {
    #[inline]
    fn flags(&self) -> Flag {
        let flags = self.head >> FLAGS_SHIFT;
        Flag::from_bits(flags).unwrap()
    }

    #[inline]
    fn label_len(&self) -> usize {
        (self.head & LABEL_LEN_MASK) as usize
    }

    #[inline]
    unsafe fn move_to_ptr<U>(&self, offset: isize) -> *mut U {
        self.as_mut_ptr().offset(offset) as *mut U
    }

    #[inline]
    fn move_to<U>(&self, offset: isize) -> &U {
        unsafe { &*self.move_to_ptr(offset) }
    }

    #[inline]
    fn neighbor_offset(&self) -> isize {
        LABEL_OFFSET + self.label_len() as isize
    }

    #[inline]
    fn child_offset(&self, flags: Flag) -> isize {
        let mut offset = self.neighbor_offset();
        if flags.contains(Flag::NEIGHBOR) {
            offset += size_of::<G> as isize;
        }
        offset
    }

    #[inline]
    fn value_offset(&self, flags: Flag) -> isize {
        let mut offset = self.child_offset(flags);
        if flags.contains(Flag::CHILD) {
            offset += size_of::<G> as isize;
        }
        offset
    }
}

//impl<T> Drop for Node<T> {
//    fn drop(&mut self) {
//        let size = self.open().size();
//        let layout = unsafe {
//            Layout::from_size_align_unchecked(size, ALLOC_ALIGN)
//        };
//        unsafe { dealloc(self.ptr(), layout) };
//    }
//}

/*pub(crate) struct Reader<'a, T> {
    node: &'a Node<T>,
}

impl<'a, 'b: 'a, T> Reader<'a, T> {
    pub(crate) fn new(node: &'b Node<T>) -> Self {
        Reader { node }
    }

    pub(crate) fn open(&mut self, node: &'b Node<T>) {
        self.node = node;
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.as_mut_ptr()
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut u8 {
        self.node.ptr()
    }

    #[inline]
    unsafe fn move_to_ptr<U>(&self, offset: isize) -> *mut U {
        self.as_mut_ptr().offset(offset) as *mut U
    }

    #[inline]
    fn move_to<U>(&self, offset: isize) -> &U {
        unsafe { &*self.move_to_ptr(offset) }
    }

    #[inline]
    pub(crate) fn label(&self) -> &[u8] {
        let label_len = self.label_len();
        unsafe {
            slice::from_raw_parts(self.move_to_ptr(LABEL_OFFSET), label_len)
        }
    }

    #[inline]
    fn neighbor_offset(&self) -> isize {
        LABEL_OFFSET + self.label_len() as isize
    }

    #[inline]
    pub(crate) fn neighbor(&self) -> Option<&Node<T>> {
        let flags = self.flags();
        if flags.contains(Flag::NEIGHBOR) {
            Some(self.move_to(self.neighbor_offset()))
        } else {
            None
        }
    }

    #[inline]
    fn child_offset(&self, flags: Flag) -> isize {
        let mut offset = self.neighbor_offset();
        if flags.contains(Flag::NEIGHBOR) {
            offset += size_of::<Node<T>> as isize;
        }
        offset
    }

    #[inline]
    pub(crate) fn child(&self) -> Option<&Node<T>> {
        let flags = self.flags();
        if flags.contains(Flag::CHILD) {
            Some(self.move_to(self.child_offset(flags)))
        } else {
            None
        }
    }

    #[inline]
    fn value_offset(&self, flags: Flag) -> isize {
        let mut offset = self.child_offset(flags);
        if flags.contains(Flag::CHILD) {
            offset += size_of::<Node<T>> as isize;
        }
        offset
    }

    #[inline]
    pub(crate) fn value(&self) -> Option<&T> {
        let flags = self.flags();
        if flags.contains(Flag::VALUE) {
            Some(self.move_to(self.value_offset(flags)))
        } else {
            None
        }
    }
}*/
