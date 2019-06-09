use super::node::Node;

pub struct TrieMap<T> {
    root: Node<T>,
    len: usize,
}