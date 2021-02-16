use core::ops::Index;
use core::mem;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pool<T> {
    inner: Vec<T>
}

impl<T> Index<usize> for Pool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> Pool<T> {
    pub const BLOCK_SIZE: usize = 1 << 9;

    pub fn size(&self) -> usize {
        self.inner.len() * Self::BLOCK_SIZE
    }

    pub fn swap(&mut self, other: &mut Self) {
        todo!()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn allocate(&mut self) -> usize {
        self.inner.reserve_exact(Self::BLOCK_SIZE);
        self.size()
    }
}
