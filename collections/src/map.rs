use core::ops::{Index, IndexMut};
use core::mem::swap;

use crate::record::Record;

pub struct Map<K, V>(Vec<Record<K, V>>);

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Map::new()
    }
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<K, V> Index<usize> for Map<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.0[index].value()
    }
}

impl<K, V> IndexMut<usize> for Map<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0[index].value_mut()
    }
}

impl<K: Ord, V> Map<K, V> {
    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        match self.get_index(&k) {
            Ok(i) => { swap(&mut v, &mut self[i]); Some(v) },
            Err(i) => { self.0.insert(i, (k, v).into()); None }
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.get_index(k).ok().map(|i| &self[i])
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        let i = self.get_index(k).ok()?;
        Some(&mut self[i])
    }

    pub fn get_or_create_mut<F>(&mut self, k: K, new: F) -> &mut V
    where F: FnOnce() -> V
    {
        match self.get_index(&k) {
            Ok(i) => &mut self[i],
            Err(i) => {
                self.0.insert(i, (k, new()).into());
                &mut self[i]
            }
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let i = self.get_index(k).ok()?;
        Some(self.0.remove(i).into_pair().1)
    }

    fn get_index(&self, k: &K) -> Result<usize, usize> {
        self.0.binary_search_by(|r| k.cmp(r.key()))
    }
}