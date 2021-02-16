use core::mem;
use mem::size_of;

use crate::pool::Pool;

type Int = usize;

pub struct BitPool {
    inner: Pool<Int>,
    len: usize,
}

impl BitPool {
    pub const BLOCK_SIZE: usize = 1 << 9;

    pub fn get(&self, index: usize) -> bool {
        let chunk = mem::size_of::<Int>();
        let i = index / chunk;
        let flag = 1 << (index % chunk);
        self.inner[i] & flag == flag 
    }

}