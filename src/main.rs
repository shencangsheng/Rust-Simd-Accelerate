#![feature(portable_simd)]
use std::simd::u8x16;

use core_simd::*;
fn main() {
    let s1 = u8x4::from_array([1, 1, 0, 0]);
    let s2 = u8x4::from_array([1, 0, 0, 1]);
    println!("{:?}", s1 & s2);
}
