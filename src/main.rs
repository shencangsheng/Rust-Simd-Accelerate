#![feature(portable_simd)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate arrayref;
use std::{collections::HashMap, fmt::Result, result};

use core_simd::{u8x16, u8x32, u8x4, u8x64, u8x8, Simd};

fn main() {
    let s1 = u8x4::from_array([1, 1, 0, 0]);
    let s2 = u8x4::from_array([1, 0, 0, 1]);
    println!("{:?}", s1 & s2);
    let result: &[u8] = &[];
    exec(&[32, 16, 8, 4], 0, &[0; 64], &[0; 64], result);
    println!("{:#?}", &result);
}

fn exec(register: &[i32], mut i: i32, a: &[u8], b: &[u8], result: &[u8]) {
    let size = a.len() as i32;
    let last = size - i;
    for x in register.iter() {
        if last > *x {
            let count = last / x;
            let next = last % x;
            for element in 0..count {
                match x {
                    32 => {
                        addArray(
                            result,
                            &(u8x32::from_array(*array_ref![&a, i as usize, 32])
                                & u8x32::from_array(*array_ref![&b, i as usize, 32]))
                            .to_array(),
                        );
                    }
                    16 => {
                        addArray(
                            result,
                            &(u8x16::from_array(*array_ref![&a, i as usize, 16])
                                & u8x16::from_array(*array_ref![&b, i as usize, 16]))
                            .to_array(),
                        );
                    }
                    8 => {
                        addArray(
                            result,
                            &(u8x8::from_array(*array_ref![&a, i as usize, 8])
                                & u8x8::from_array(*array_ref![&b, i as usize, 8]))
                            .to_array(),
                        );
                    }
                    4 => {
                        addArray(
                            result,
                            &(u8x4::from_array(*array_ref![&a, i as usize, 4])
                                & u8x4::from_array(*array_ref![&b, i as usize, 4]))
                            .to_array(),
                        );
                    }
                    _ => unreachable!(),
                }
                i += x - 1;
            }
        }
    }
}

fn addArray(mut s1: &[u8], s2: &[u8]) {}
