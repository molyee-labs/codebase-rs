use core::cmp::Ordering;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SliceMap<K, V> {
    buf: Vec<K>,
    map: Vec<(usize, V)>,
}

impl<K, V> SliceMap<K, V> {
    pub fn new() -> Self {
        let buf = Vec::new();
        let map = Vec::new();
        Self { buf, map }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let buf = Vec::with_capacity(cap);
        let map = Vec::new();
        Self { buf, map }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn put<I: Iterator<Item = K>>(&mut self, k: I, v: V) {
        self.map.push((self.buf.len(), v));
        self.buf.extend(k);
    }

    pub fn get(&self, index: usize) -> Option<(&[K], &V)> {
        let value = self.value(index)?;
        let key = self.key(index)?;
        Some((key, value))
    }

    pub fn key(&self, index: usize) -> Option<&[K]> {
        let start = self.key_offset(index)?;
        let end = self.key_offset(index + 1).unwrap_or_else(|| self.buf.len());
        Some(&self.buf[start..end])
    }

    pub fn value(&self, index: usize) -> Option<&V> {
        self.map.get(index).map(|i| &i.1)
    }

    fn key_offset(&self, index: usize) -> Option<usize> {
        self.map.get(index).map(|i| i.0)
    }
}

impl<K: Ord, V> SliceMap<K, V> {
    fn binary_search_index(&self, seq: &[K]) -> Option<usize> {
        let buf_end = self.buf.len();
        if buf_end == 0 {
            return None;
        }
        let mut l = 0;
        let mut r = self.map.len() - 1;
        let mut i = r;
        while l <= r {
            let m = l + ((r - l) / 2);
            let b = self.map[m].0;
            let e = self.map.get(m + 1).map(|(i, _)| *i).unwrap_or(buf_end);
            match &self.buf[b..e].cmp(seq) {
                Ordering::Equal => return Some(m),
                Ordering::Less => l = m + 1,
                Ordering::Greater => r = m,
            }
            if m == i {
                break;
            }
            i = m;
        }
        None
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StringMap<V>(SliceMap<u8, V>);

impl<V> StringMap<V> {
    pub fn new() -> Self {
        Self(SliceMap::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(SliceMap::with_capacity(cap))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn put<S: AsRef<str>>(&mut self, s: S, v: V) {
        let k = s.as_ref().as_bytes().iter().cloned();
        self.0.put(k, v)
    }

    pub fn get(&self, index: usize) -> Option<(&str, &V)> {
        let (bytes, v) = self.0.get(index)?;
        let s = std::str::from_utf8(bytes).unwrap();
        Some((s, v))
    }

    pub fn key(&self, index: usize) -> Option<&str> {
        let bytes = self.0.key(index)?;
        let s = std::str::from_utf8(bytes).unwrap();
        Some(s)
    }

    pub fn value(&self, index: usize) -> Option<&V> {
        self.0.value(index)
    }

    pub fn binary_search<S: AsRef<str>>(&self, s: S) -> Option<&V> {
        let b = s.as_ref().as_bytes();
        self.0.binary_search_index(b).and_then(|i| self.value(i))
    }
}

impl<V> From<Vec<(String, V)>> for StringMap<V> {
    fn from(mut src: Vec<(String, V)>) -> Self {
        let len = src.len();
        src.sort_by(|a, b| a.0.cmp(&b.0));
        src.dedup_by(|a, b| a.0.eq(&b.0));
        assert_eq!(src.len(), len);
        let cap = src.iter().fold(0, |acc, i| acc + i.0.len());
        let mut map = StringMap::with_capacity(cap);
        src.into_iter().for_each(|(k, v)| map.put(k, v));
        map
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn smap() -> StringMap<u32> {
        StringMap::new()
    }

    #[test]
    fn map_put_and_find() {
        let mut map = smap();
        map.put("word3", 3);
        map.put("word2", 2);
        map.put("word5", 5);
        map.put("word4", 4);
        map.put("word1", 1);
        println!("{:?}", map);
        for i in 1..6 {
            let key = &format!("word{}", i);
            assert_eq!(map.binary_search(key), Some(&i));
        }
    }
}
