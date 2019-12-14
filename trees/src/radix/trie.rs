pub(crate) struct Trie<T> {
    root: Node<T>,
    len: usize,
}

pub(crate) type Key = AsRef<[u8]>;

impl Trie<T> {
    pub fn new() -> Self {
        Trie {
            root: Node::root(),
            len: 0
        }
    }

    pub fn root(&self) -> &Node<T> {
        &self.root
    }

    pub fn into_root(&self) -> Node<T> {
        self.root
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.root = Node::root();
        self.len = 0;
    }

    pub fn insert<K: Key>(&mut self, key: K, value: T) -> Option<T> {
        unimplemented!()
    }

    pub fn get<K: Key>(&self, key: K) -> Option<&T> {
        unimplemented!()
    }

    pub fn find<K: Key>(&self, key: K) -> Option<Node<T>> {
        unimplemented!()
    }

    pub fn remove<K: Key>(&self, key: K) -> Option<Node<T>> {
        unimplemented!()
    }
}
