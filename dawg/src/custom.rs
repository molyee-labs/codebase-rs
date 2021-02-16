pub struct Dawg<K, V> {
    nodes: Vec<Node<V>>,
    edges: Vec<Edge<K>>,
}

struct Node<V> {
    value: Option<V>,
}

impl<V> Node<V> {
    pub fn new(value: Option<V>) -> Self {
        Self { value }
    }

    pub fn empty() -> Self {
        Self::new(None)
    }
}

struct Edge<K> {
    source: usize,
    target: usize,
    key: K
}

impl<K, V> Dawg<K, V> {
    pub fn new() -> Self {
        let nodes = vec![Node::empty()];
        let edges = vec![];
        Self { nodes, edges }
    }

    pub fn root(&self) -> &Node<V> {
        &self.nodes[0]
    }

    pub fn insert<I>(&mut self, keys: I, value: V) -> Option<V>
    where I: Iterator<Item = K>
    {
        todo!()
    }

    pub fn remove<I>(&mut self, keys: I) -> Option<V>
    where I: Iterator<Item = K>
    {
        todo!()
    }

    pub fn find<I>(&self, keys: I) -> Option<V>
    where I: Iterator<Item = K>
    {
        todo!()
    }

    pub fn traverse<I>(&mut self, keys: I) -> Traverse<K, V, I>
    where I: Iterator<Item = K>
    {
        todo!()
    }

    fn node(&self, idx: usize) -> &Node<V> {
        &self.nodes[idx]
    }

    fn children(&self, idx: usize, k: K) -> &[Edge<K>] {
        todo!()
    }

}

pub struct Traverse<'d, K, V, I> {
    inner: &'d Dawg<K, V>,
    keys: I
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTS: Vec<&'static str> = vec![

    ];

    #[test]
    fn insert_unique_words() {
        assert!(false)
    }

    #[test]
    fn store_unique_pair() {
        assert!(false)
    }

    #[test]
    fn static_insert_and_find() {
        assert!(false)
    }

    #[test]
    fn random_insert_and_find() {
        assert!(false)
    }
}