#![feature(portable_simd)]
#![allow(non_camel_case_types)]

use core_simd::{u8x16, u8x32, u8x4, u8x64, u8x8, Simd};

fn main() {
    const s1: [u8; 8] = [1, 1, 0, 0, 1, 0, 1, 0];
    const s2: [u8; 8] = [1, 1, 0, 1, 1, 0, 0, 0];
    let mut result: [u8; 8] = [0; 8];
    println!("{:?}", simd(&s1, &s2, 0, result));
}

fn simd<const LANES: usize>(
    s1: &[u8; LANES],
    s2: &[u8; LANES],
    current: i32,
    mut result: [u8; LANES],
) -> [u8; LANES] {
    if s1.len() == 1 {
        return *s1;
    }
    const simd_scope: [i32; 5] = [64, 32, 16, 8, 4];
    let surplus = s1.len() as i32 - current;
    let mut interval = None;
    for item in simd_scope {
        if item <= surplus {
            interval = Some(item);
            break;
        }
    }
    let interval = match interval {
        None => 1,
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
        _ => {}
    }

    if next > result.len() {
        return result;
    }

    return simd(s1, s2, next as i32, result);
}

fn simd_u8x32<const LANES: usize>(
    input: Simd<u8, 32>,
    mut values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.to_array(), idx);
}

fn simd_u8x64<const LANES: usize>(
    input: Simd<u8, 64>,
    mut values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.to_array(), idx);
}

fn simd_u8x16<const LANES: usize>(
    input: Simd<u8, 16>,
    mut values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.to_array(), idx);
}

fn simd_u8x8<const LANES: usize>(
    input: Simd<u8, 8>,
    mut values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.to_array(), idx);
}

fn simd_u8x4<const LANES: usize>(
    input: Simd<u8, 4>,
    mut values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.to_array(), idx);
}

fn push<const LANES: usize, const LANES2: usize>(
    mut values: [u8; LANES],
    array: [u8; LANES2],
    mut idx: i32,
) -> [u8; LANES] {
    for item in array.iter() {
        values[idx as usize] = *item;
        idx += 1;
    }
    return values;
}
