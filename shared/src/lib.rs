#![no_std]

extern crate alloc;

mod ptr;
pub use ptr::Ptr;

mod link;
#[cfg(feature = "std")]
pub use link::Link;

mod rc;
pub use rc::Rc;
pub use rc::RcCell;