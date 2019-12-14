use super::node;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ops::{Deref, DerefMut};
use std::alloc::{Layout};
use std::marker::PhantomData;
use super::access::Gate;
use super::Node;

#[repr(transparent)]
pub(crate) struct Link<N, G> {
    ptr: *mut G,
    _m: PhantomData<&(N, G)>
}

/*impl<T, B: Bunch<T>> From<&B> for Link<T> {
    fn from(src: &B) -> Self {
        let ptr = AtomicPtr::new()
        unsafe { Link::from_raw(src as *const Bunch<T> as *const u8) }
    }
}

impl<T, N: Node<T>> From<&N> for Link<T> {
    fn from(src: &N) -> Self {
        unsafe { Link::from_raw(src as *const Node<T> as *const u8) }
    }
}*/

/*impl<N, G> Deref for Link<'_, N, G> {
    type Target = Unit<'_, N, G>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.load(Ordering::SeqCst) as &Self::Target }
    }
}

impl<N, G> DerefMut for Link<'_, N, G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.load(Ordering::SeqCst) as &mut Self::Target }
    }
}*/

/*impl<N, G> From<&mut Unit<'_, N, G>> for Link<'_, N, G> {
    fn from<'a, 'u: 'a>(u: &'u mut Unit<'_, N, G>) -> Link<'a, N, G> {
        unsafe { Link::from_raw(u as *mut Unit<'_, N, G>) }
    }
}

impl<N, G> Link<'_, N, G> {
    unsafe fn from_raw(ptr: *mut Unit<N, G>) -> Self {
        Link { ptr: AtomicPtr::new(ptr) }
    }
}&/

/*pub(crate) struct Iter<'a, 'u: 'a, N: Node, G> {
    ptr: *mut Unit<'u, N, G>
}

impl<'a, 'u: 'a, N, G> Iterator for Iter<'a, N, G> {
    type Item = &'a Unit<'u, N, G>;

    fn next(&mut self) -> Option<Self::Item> {

    }
}

pub(crate) struct IterMut<'a, 'u: 'a, N: Node, G> {
    ptr: *mut Unit<'u, N, G>
}

impl<'a, 'u, N, G> Iterator for IterMut<'a, N, G> {
    type Item = &'a mut Unit<'u, N, G>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}*/

/*impl<N, G> Unit<'_, N, G> {
    pub fn children(&self) -> Iter<'_, N, G> {
        unimplemented!()
    }

    pub fn neighbors(&self) -> Iter<'_, N, G> {
        unimplemented!()
    }

    pub fn children_mut(&mut self) -> IterMut<'_, N, G> {
        unimplemented!()
    }

    pub fn neighbors_mut(&mut self) -> IterMut<'_, N, G> {
        unimplemented!()
    }

    pub fn set_child<'a, 'u: 'a>(&'a mut self, new: Option<&'u mut Self>) {
        unimplemented!()
    }

    pub fn child<'a, 'u: 'a>(&'a self) -> Option<&Self> {
        unimplemented!()
    }

    pub fn replace_child<'a, 'u: 'a>(&'a mut self, new: Option<&'u mut Self>) -> Option<&'a mut Self> {
        self.gate.
    }

    pub fn delete_children(&mut self) {
        self.replace_child(None)
            .and_then(|child| child.delete_children())
    }
}

impl<N, G> Drop for Unit<'_, N, G> {
    fn drop(&mut self) {
        self.delete_children();
        self.gate.delete()
    }
}*/
