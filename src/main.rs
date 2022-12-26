#![feature(portable_simd)]
#![allow(non_camel_case_types)]

use std::time::Instant;

use core_simd::{u8x16, u8x32, u8x4, u8x64, u8x8, Simd};

fn main() {
    let cardinal: usize = 2000;
    for multiple in 1..11 {
        let len = cardinal * multiple;
        let s1: Vec<u8> = vec![0; len];
        let s2: Vec<u8> = vec![1; len];
        println!("{:?}", len);
        let start = Instant::now();
        let mut result: Vec<u8> = vec![0; len];
        for i in 0..len {
            result[i] = s1[i] & s2[i];
        }
        let duration = start.elapsed().as_micros();
        println!("Time elapsed in default is: {:?}", duration);
        let start = Instant::now();
        simd(&s1, &s2, 0, vec![0; len]);
        let duration = start.elapsed().as_micros();
        println!("Time elapsed in SIMD is: {:?}", duration);
        println!("===============================")
    }
}

fn simd(s1: &Vec<u8>, s2: &Vec<u8>, current: i32, mut result: Vec<u8>) -> Vec<u8> {
    if s1.len() == 1 {
        return vec![s1[0]];
    }
    const SIMD_SCOPE: [i32; 5] = [64, 32, 16, 8, 4];
    let surplus = s1.len() as i32 - current;
    let mut interval = None;
    for item in SIMD_SCOPE {
        if item <= surplus {
            interval = Some(item);
            break;
        }
    }
    let interval = match interval {
        None => s1.len() as i32 - current,
        Some(i) => i,
    };

    let next = (interval + current) as usize;

    match interval {
        64 => {
            result = simd_u8x64(
                u8x64::from_slice(&s1[current as usize..next])
                    & u8x64::from_slice(&s2[current as usize..next]),
                result,
                current,
            );
        }
        32 => {
            result = simd_u8x32(
                u8x32::from_slice(&s1[current as usize..next])
                    & u8x32::from_slice(&s2[current as usize..next]),
                result,
                current,
            );
        }
        16 => {
            result = simd_u8x16(
                u8x16::from_slice(&s1[current as usize..next])
                    & u8x16::from_slice(&s2[current as usize..next]),
                result,
                current,
            );
        }
        8 => {
            result = simd_u8x8(
                u8x8::from_slice(&s1[current as usize..next])
                    & u8x8::from_slice(&s2[current as usize..next]),
                result,
                current,
            );
        }
        4 => {
            result = simd_u8x4(
                u8x4::from_slice(&s1[current as usize..next])
                    & u8x4::from_slice(&s2[current as usize..next]),
                result,
                current,
            );
        }
        _ => {
            for i in current..next as i32 {
                result[i as usize] = s1[i as usize] & s2[i as usize];
            }
        }
    }

    if next >= result.len() {
        return result;
    }

    return simd(s1, s2, next as i32, result);
}

fn simd_u8x32(input: Simd<u8, 32>, values: Vec<u8>, idx: i32) -> Vec<u8> {
    return push(values, input.as_array(), idx);
}

fn simd_u8x64(input: Simd<u8, 64>, values: Vec<u8>, idx: i32) -> Vec<u8> {
    return push(values, input.as_array(), idx);
}

fn simd_u8x16(input: Simd<u8, 16>, values: Vec<u8>, idx: i32) -> Vec<u8> {
    return push(values, input.as_array(), idx);
}

fn simd_u8x8(input: Simd<u8, 8>, values: Vec<u8>, idx: i32) -> Vec<u8> {
    return push(values, input.as_array(), idx);
}

fn simd_u8x4(input: Simd<u8, 4>, values: Vec<u8>, idx: i32) -> Vec<u8> {
    return push(values, input.as_array(), idx);
}

fn push<const LANES: usize>(mut values: Vec<u8>, array: &[u8; LANES], mut idx: i32) -> Vec<u8> {
    for item in array.iter() {
        values[idx as usize] = *item;
        idx += 1;
    }
    return values;
}
