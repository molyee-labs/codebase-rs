use std::marker::PhantomData;
use super::node::Node;
use super::access::Gate;

#[repr(C)]
pub(crate) struct Bunch<T, G: Gate> {
    gate: G,
    size: u32, // allocated bytes
    _m: PhantomData<T>,
}

impl<T> Deref for Link<T> {
    type Target = Bunch<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
        //unsafe { &*self.ptr.load(Ordering::SeqCst) }
    }
}

/*impl<T> Link<T> {
    pub(crate) fn open(&self) -> Reader<'_, T> {
        Reader::new(&*self)
    }
}

impl<T> Drop for Link<T> {
    fn drop(&mut self) {
        let size = self.open().size();
        let layout = unsafe {
            Layout::from_size_align_unchecked(size, ALLOC_ALIGN)
        };
        unsafe { dealloc(self.ptr(), layout) };
    }
}*/

impl<T, A> Bunch<T, A> {
    fn shrink_to_fit(&mut self) {
        unimplemented!()
    }

    fn len(&self) -> usize {
        unimplemented!()
    }

    fn size(&self) -> usize {
        self.raw
    }
}

impl Bunch<T, A> {
    pub(crate) fn add_node(
        label: &'b [u8],
        neighbor: Option<&mut Node<T>>,
        child: Option<&mut Node<T>>,
        value: Option<T>
    ) -> &'a mut Self {
        let label_len = label.len();
        assert!(label_len <= MAX_LABEL_LEN);
        let mut flags = Flag::empty();
        flags.set(Flag::NEIGHBOR, neighbor.is_some());
        flags.set(Flag::CHILD, child.is_some());
        flags.set(Flag::VALUE, value.is_some());
        let size = flags.calc_node_size::<T>();
        let flags_and_label_len = (flags.bits() << FLAGS_SHIFT) + (label_len as u8);
        unsafe {
            let node: Alloc<Self> = alloc(size, ALLOC_ALIGN);
            let pos = pointer::write_next(node.as_ptr() as *mut u8, flags_and_label_len);
            let pos = pointer::copy_next(label.as_ptr(), pos, label_len);
            let pos = pointer::try_write_next(pos, neighbor);
            let pos = pointer::try_write_next(pos, child);
            forget(pointer::try_write_next(pos, value) as *mut u8);
            Alloc::leak(node)
        }
    }

    pub(crate) fn root<'a>() -> &'a Self {
        Node::new(b"", None, None, None)
    }
}
