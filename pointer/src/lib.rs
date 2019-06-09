use std::ptr::{write_unaligned, copy_nonoverlapping};
use std::mem::{size_of};

#[inline]
pub unsafe fn write_next<T, U>(dst: *mut T, src: T) -> *mut U {
    write_unaligned(dst, src);
    let size = size_of::<T>();
    dst.add(size) as *mut U
}

#[inline]
pub unsafe fn try_write_next<T, U>(dst: *mut T, src: Option<T>) -> *mut U {
    if src.is_some() {
        write_next(dst, src.unwrap())
    } else {
        dst as *mut U
    }
}

#[inline]
pub unsafe fn copy_next<T, U>(src: *const T, dst: *mut T, count: usize) -> *mut U {
    copy_nonoverlapping(src, dst, count);
    let size = size_of::<T>() * count;
    dst.add(size) as *mut U
}
