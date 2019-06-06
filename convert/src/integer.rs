use super::{impl_transmute, Transmute};

use std::mem as mem;

impl_transmute!(u128, [u8; 16]);
impl_transmute!(u64, [u8; 8]);
impl_transmute!(u32, [u8; 4]);
impl_transmute!(u16, [u8; 2]);
impl_transmute!([u8; 16], u128);
impl_transmute!([u8; 8], u64);
impl_transmute!([u8; 4], u32);
impl_transmute!([u8; 2], u16);
