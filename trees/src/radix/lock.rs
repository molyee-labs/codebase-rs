use std::sync::atomic::{Atomic, Ordering};

const REMOVED_MASK: u8 = 0b10000000;
const BUNCH_MASK: u8 = 0b01000000;
const OPEN_MUT_MASK: u8 = 0b00100000;
const COUNT_MASK: u8 = 0b00011111;

#[repr(transparent)]
pub(crate) struct Lock {
    raw: AtomicU8
}

// 1bit - 1 - removed
// 1bit - 0 - node, 1 - bunch
// 1bit - 1 - open as mut
// 5bits - lock (access counter)
impl State for u8 {
    fn is_removed(&self) -> bool {
        self & REMOVED_MASK == REMOVED_MASK
    }
    
    fn is_open_as_mut(&self) -> bool {
        self & OPEN_MUT_MASK == OPEN_MUT_MASK
    }

    fn is_bunch(&self) -> bool {
        self & BUNCH_MASK == BUNCH_MASK
    }

    fn is_open(&self) -> bool {
        self & COUNT_MASK > 0
    }

    fn is_max_open(&self) -> bool {
        self & COUNT_MASK == COUNT_MASK
    }

    fn can_open(&self) -> bool {
        !self.is_deleted() && !self.is_open_as_mut() && !self.is_max_open()
    }

    fn open(&self) -> Option<Self> {
        if self.can_open() {
            None
        } else {
            Some(self + 1)
        }
    }

    fn can_open_mut(&self) -> bool {
        !self.is_deleted() && !self.is_open_mut() && !self.is_open()
    }

    fn open_mut(&self) -> Option<Self> {
        if self.can_open_mut() {
            Some(self | OPEN_MUT_MASK)
        } else {
            None
        }
    }

    fn close(&self) -> Option<Self> {
        if self.is_open() {
            Some(self - 1)
        } else {
            None
        }
    }

    fn close_mut(&self) -> Option<Self> {
        if self.is_open_as_mut() {
            Some(self ^ OPEN_MUT_MASK)
        } else {
            None
        }
    }

    fn remove(&self) -> Option<Self> {
        if self.is_removed() {
            None
        } else {
            Some(self | REMOVED_MASK)
        }
    }
}

trait State {
    fn is_removed(&self) -> bool;
    fn is_bunch(&self) -> bool;
    fn is_open(&self) -> bool;
    fn is_open_mut(&self) -> bool;
    fn open(&self) -> Option<Self>;
    fn open_mut(&self) -> Option<Self>;
    fn close(&self) -> Option<Self>;
    fn close_mut(&self) -> Option<Self>;
    fn remove(&self) -> Option<Self>;
}

type UpdateResult = Result<State, State>;

impl Lock {
    pub(crate) unsafe fn from_raw(src: *mut u8) -> Self {
        Lock { lock: AtomicPtr::new(src) }
    }

    unsafe fn update(&self, f: F, fetch_order: Oridering, set_order: Ordering) -> UpdateResult
    where F: FnMut(State) -> Option<State> {
        self.raw
            .fetch_update(|v| f(State::from(v)).into(), fetch_order, set_order)
            .map_err(State::from)
    }

    unsafe fn release(&self) {
        self.raw
            .fetch_update(State::close_read, Ordering::SeqCst, Ordering::SeqCst)
            .unwrap()
    }

    unsafe fn release_mut(&mut self) {
        self.raw
            .fetch_update(State::close_write, Ordering::SeqCst, Ordering::SeqCst)
            .unwrap()
    }
}

impl Gate for Lock {
    fn get(&self) -> AccessResult {
        self.update(u8::open, Ordering::SeqCst, Ordering::SeqCst)
        let permit = Permission::Read;
        Ok(Access { permit, lock: self })
    }

    fn get_mut(&self) -> AccessResult {
        self.update(u8::open_mut, Ordering::SeqCst, Ordering::SeqCst)
            .map_err(u8::open_mut)
        let permit = Permission::Write;
        Ok(Access { permit, lock: self }
    }

    fn delete(&self) -> AccessResult {
        self.get_mut()
            .map_err()
    }
}


