use super::node::Node;
use super::trie::Trie;

pub struct TrieMap<T> {
    inner: Trie<T>,
}
