#![feature(portable_simd)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate arrayref;
use core_simd::{u8x16, u8x32, u8x4, u8x8};

fn main() {
    let s1: [u8; 4] = [1, 1, 0, 0];
    let s2: [u8; 4] = [1, 1, 0, 1];
    let mut result: Vec<u8> = Vec::new();
    simd(&s1, &s2, s1.len() as i32, 0, 32, &mut result);
    println!("{:?}", result);
}

fn simd(s1: &[u8], s2: &[u8], size: i32, mut current: i32, channel: i32, result: &mut Vec<u8>) {
    if channel <= 1 {
        return;
    } else {
        let mut t_current = current + channel;
        while t_current <= size {
            match channel {
                32 => {
                    concat_array(
                        result,
                        simd_u8x32(
                            *array_ref![&s1, current as usize, 32],
                            *array_ref![&s2, current as usize, 32],
                        ),
                    );
                }
                16 => {
                    concat_array(
                        result,
                        simd_u8x16(
                            *array_ref![&s1, current as usize, 16],
                            *array_ref![&s2, current as usize, 16],
                        ),
                    );
                }
                8 => {
                    concat_array(
                        result,
                        simd_u8x8(
                            *array_ref![&s1, current as usize, 8],
                            *array_ref![&s2, current as usize, 8],
                        ),
                    );
                }
                4 => {
                    concat_array(
                        result,
                        simd_u8x4(
                            *array_ref![&s1, current as usize, 4],
                            *array_ref![&s2, current as usize, 4],
                        ),
                    );
                }
                _ => println!("Ain't special"),
            };
            current = t_current;
            t_current = t_current + channel;
        }
        simd(s1, s2, size, current, channel / 2, result);
    }
}

fn simd_u8x4(s1: [u8; 4], s2: [u8; 4]) -> [u8; 4] {
    return *(u8x4::from_array(s1) & u8x4::from_array(s2)).as_mut_array();
}

fn simd_u8x8(s1: [u8; 8], s2: [u8; 8]) -> [u8; 8] {
    return *(u8x8::from_array(s1) & u8x8::from_array(s2)).as_array();
}

fn simd_u8x16(s1: [u8; 16], s2: [u8; 16]) -> [u8; 16] {
    return *(u8x16::from_array(s1) & u8x16::from_array(s2)).as_array();
}

fn simd_u8x32(s1: [u8; 32], s2: [u8; 32]) -> [u8; 32] {
    return *(u8x32::from_array(s1) & u8x32::from_array(s2)).as_array();
}

fn concat_array<const LANES: usize>(s1: &mut Vec<u8>, s2: [u8; LANES]) {
    for i in s2.iter() {
        s1.push(*i);
    }
}
