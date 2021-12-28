use crate::record::Rec;
use core::mem;
use core::borrow::Borrow;
use core::slice;
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

/*impl<K, V> Index<usize> for Map<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.0[index].value()
    }
}

impl<K, V> IndexMut<usize> for Map<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0[index].value_mut()
    }
}*/

impl<K: Ord, V> Map<K, V> {
    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        match self.get_index(&k) {
            Ok(i) => { mem::swap(&mut v, &mut self.0[i].val); Some(v) },
            Err(i) => { self.0.insert(i, (k, v).into()); None }
        }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let i = self.get_index(k).ok()?;
        Some(&self.0[i].val)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let i = self.get_index(k).ok()?;
        Some(&mut self.0[i].val)
    }

    pub fn get_or_create_mut<F>(&mut self, k: K, new: F) -> &mut V
    where F: FnOnce() -> V
    {
        match self.get_index(&k) {
            Ok(i) => &mut self.0[i].val,
            Err(i) => {
                self.0.insert(i, (k, new()).into());
                &mut self.0[i].val
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

pub(crate) struct InnerIter<'i, K, V>(slice::Iter<'i, Rec<K, V>>);

impl<'i, K, V> Iterator for InnerIter<'i, K, V> {
    type Item = &'i Rec<K, V>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct Iter<'i, K, V>(InnerIter<'i, K, V>);

impl<'i, K, V> Iterator for Iter<'i, K, V> {
    type Item = (&'i K, &'i V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Rec::as_pair)
    }
}

pub struct Keys<'i, K, V>(InnerIter<'i, K, V>);

impl<'i, K, V> Iterator for Keys<'i, K, V> {
    type Item = &'i K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Rec::key)        
    }
}

pub struct Values<'i, K, V>(InnerIter<'i, K, V>);

impl<'i, K, V> Iterator for Values<'i, K, V> {
    type Item = &'i V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Rec::value)        
    }
}

impl<K, V> Map<K, V> {
    #[inline]
    pub(crate) fn inner_iter(&self) -> InnerIter<'_, K, V> {
        InnerIter(self.0.iter())
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter(self.inner_iter())
    }

    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(self.inner_iter())
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values(self.inner_iter())
    }
}
