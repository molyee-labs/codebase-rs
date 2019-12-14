use core::ptr::{write_unaligned, copy_nonoverlapping};
use core::mem;

#[inline]
pub unsafe fn write_next<T, U>(dst: *mut T, src: T) -> *mut U {
    write_unaligned(dst, src);
    let size = mem::size_of::<T>() as isize;
    dst.offset(size) as *mut U
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
    let size = (mem::size_of::<T>() * count) as isize;
    dst.offset(size) as *mut U
}
