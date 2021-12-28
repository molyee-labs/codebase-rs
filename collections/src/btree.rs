use crate::record::Rec;
use core::ops::{Index, IndexMut};
use core::mem;
use core::borrow::Borrow;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

/// A map based on both [B-Tree] and [Vec]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub struct Map<K, V>(Vec<Rec<K, V>>);

pub type Set<K> = Map<K, ()>;

impl<K, V> Default for Map<K, V> {
    #[inline]
    fn default() -> Self {
        Map::new()
    }
}

impl<K, V> Map<K, V> {
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
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
            Ok(i) => { mem::swap(&mut v, &mut self[i]); Some(v) },
            Err(i) => { self.0.insert(i, (k, v).into()); None }
        }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.get_index(k).ok().map(|i| &self[i])
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
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

    #[inline]
    pub fn contains<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.get_index(k).is_ok()
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let i = self.get_index(k).ok()?;
        Some(self.0.remove(i).val)
    }

    #[inline]
    fn get_index<Q: ?Sized>(&self, k: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.0.binary_search_by(|r| r.key.borrow().cmp(k))
    }
}