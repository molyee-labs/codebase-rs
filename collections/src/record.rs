#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub(crate) struct Rec<K, V> {
    pub(crate) key: K,
    pub(crate) val: V
}

impl<K, V> From<(K, V)> for Rec<K, V> {
    fn from((key, val): (K, V)) -> Self {
        Self { key, val }
    }
}

impl<K, V> Rec<K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.val
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.val
    }

    pub fn into_pair(self) -> (K, V) {
        (self.key, self.val)
    }
}