use shared::{Rc, RcCell, Ptr};
use crate::map::Map;
use core::ops::{DerefMut, Deref};

trait Key: Ord { }

impl<T: Ord> Key for T { }

pub struct Trie<K, V> {
    root: NodeRef<K, V>,
    len: usize
}

impl<K: Key, V> Trie<K, V> {
    pub fn new() -> Self {
        let root = NodeRef::new(Node::new());
        let len = 0;
        Trie { root, len }
    }
}

struct NodeRef<K, V> {
    p: RcCell<Node<K, V>>
}

impl<K, V> Clone for NodeRef<K, V> {
    fn clone(&self) -> Self {
        let p = self.p.clone();
        Self { p }
    }
}

impl<K: Key, V> NodeRef<K, V> {
    pub fn new(node: Node<K, V>) -> Self {
        let p = RcCell::new(node);
        Self { p }
    }

    pub fn insert<I>(&mut self, mut keys: I, mut v: V) -> Option<V>
    where I: Iterator<Item = K>
    {
        if let Some(k) = keys.next() {
            self.p.get()
                .get_or_add_child(k)
                .insert(keys, v)
        } else {
            self.p.get()
                .set_value(v)
        }
    }
}

struct Node<K, V> {
    value: Ptr<V>,
    children: Map<K, NodeRef<K, V>>
}

impl<K: Key, V> Node<K, V> {
    fn new() -> Self {
        let value = Ptr::null();
        let children = Map::new();
        Self { value, children }
    }

    fn set_value(&mut self, mut v: V) -> Option<V> {
        if self.value.replace(&mut v) {
            Some(v)
        } else {
            None
        }
    }

    fn child(&self, k: &K) -> Option<&NodeRef<K, V>> {
        self.children.find(k)
    }

    fn child_mut(&mut self, k: &K) -> Option<&mut NodeRef<K, V>> {
        self.children.find_mut(k)
    }

    fn get_or_add_child(&mut self, k: K) -> &mut NodeRef<K, V> {
        todo!()
    }
}
