use core::ops::{Index, IndexMut};
use core::mem::swap;

pub struct Map<K, V> {
    inner: Vec<(K, V)>
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Map::new()
    }
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }
    
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> Index<usize> for Map<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index].1
    }
}

impl<K, V> IndexMut<usize> for Map<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index].1
    }
}

impl<K: Ord, V> Map<K, V> {
    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        match self.find_index(&k) {
            Ok(i) => { swap(&mut v, &mut self[i]); Some(v) },
            Err(i) => { self.inner.insert(i, (k, v)); None }
        }
    }

    pub fn find(&self, k: &K) -> Option<&V> {
        self.find_index(k).ok().map(|i| &self[i])
    }

    pub fn find_mut(&mut self, k: &K) -> Option<&mut V> {
        let i = self.find_index(k).ok()?;
        Some(&mut self[i])
    }

    pub fn get_or_create_mut<F>(&mut self, k: K, new: F) -> &mut V
    where F: FnOnce() -> V
    {
        match self.find_index(&k) {
            Ok(i) => &mut self[i],
            Err(i) => {
                self.inner.insert(i, (k, new()));
                &mut self[i]
            }
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let i = self.find_index(k).ok()?;
        Some(self.inner.remove(i).1)
    }

    fn find_index(&self, k: &K) -> Result<usize, usize> {
        self.inner.binary_search_by(|(ref key, _)| k.cmp(key))
    }
}