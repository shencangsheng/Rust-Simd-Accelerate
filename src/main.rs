#![feature(portable_simd)]
#![allow(non_camel_case_types)]

use std::time::Instant;

use core_simd::{u8x16, u8x32, u8x4, u8x64, u8x8, Simd};

fn main() {
    const LANES: usize = 6000;
    const S1: [u8; LANES] = [1; LANES];
    const S2: [u8; LANES] = [0; LANES];
    let start = Instant::now();
    let mut result = [0; S1.len()];
    for i in 0..LANES {
        result[i] = S1[i] & S2[i];
    }
    let duration = start.elapsed();
    println!("{:?}", result.len());
    println!("Time elapsed in default is: {:?}", duration);
    let start = Instant::now();
    println!("{:?}", simd(&S1, &S2, 0, [0; S1.len()]).len());
    let duration = start.elapsed();
    println!("Time elapsed in SIMD is: {:?}", duration);
}

fn simd<const LANES: usize>(
    s1: &[u8; LANES],
    s2: &[u8; LANES],
    current: i32,
    mut result: [u8; LANES],
) -> [u8; LANES] {
    if s1.len() == 1 {
        return [s1[0]; LANES];
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
        None => LANES as i32 - current,
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

fn simd_u8x32<const LANES: usize>(
    input: Simd<u8, 32>,
    values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.as_array(), idx);
}

fn simd_u8x64<const LANES: usize>(
    input: Simd<u8, 64>,
    values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.as_array(), idx);
}

fn simd_u8x16<const LANES: usize>(
    input: Simd<u8, 16>,
    values: [u8; LANES],
    idx: i32,
) -> [u8; LANES] {
    return push(values, input.as_array(), idx);
}

fn simd_u8x8<const LANES: usize>(input: Simd<u8, 8>, values: [u8; LANES], idx: i32) -> [u8; LANES] {
    return push(values, input.as_array(), idx);
}

fn simd_u8x4<const LANES: usize>(input: Simd<u8, 4>, values: [u8; LANES], idx: i32) -> [u8; LANES] {
    return push(values, input.as_array(), idx);
}

fn push<const LANES: usize, const LANES2: usize>(
    mut values: [u8; LANES],
    array: &[u8; LANES2],
    mut idx: i32,
) -> [u8; LANES] {
    for item in array.iter() {
        values[idx as usize] = *item;
        idx += 1;
    }
    return values;
}
