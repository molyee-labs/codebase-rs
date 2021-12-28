mod record;
#[cfg(feature = "map")]
mod map;
#[cfg(feature = "multimap")]
mod multimap;
#[cfg(feature = "slicemap")]
mod slicemap;
#[cfg(feature = "trie")]
mod trie;
#[cfg(feature = "list")]
mod list;

#[cfg(feature = "map")]
pub use map::Map;

#[cfg(feature = "multimap")]
pub use multimap::MultiMap;

#[cfg(feature = "slicemap")]
pub use slicemap::{SliceMap, StringMap};

#[cfg(feature = "trie")]
pub use trie::Trie;

#[cfg(feature = "list")]
pub use list::List;

//mod dawg;
