#![feature(portable_simd)]
use std::{ptr::null, simd::u8x16};

use core_simd::*;
fn main() {
    let s1 = u8x4::from_array([1, 1, 0, 0]);
    let s2 = u8x4::from_array([1, 0, 0, 1]);
    println!("{:?}", s1 & s2);
    exec([64, 32, 16, 8, 4], 0, &[1], &[2])
}

fn exec<T>(register: T, i: i32, a: &[u8], b: &[u8]) {
    let size = a.len() as i32;
    let last = size - i;
    let count: Option<i32>;
    let next: Option<i32>;
    for x in register.iter() {
        if last > *x {
            count = Some(last / x);
            next = Some(last % x);
            break;
        }
    }

    if count.is_some() && next.is_some() {}
}
