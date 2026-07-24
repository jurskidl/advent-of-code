use std::{simd::{
            prelude::SimdPartialEq, 
            u8x64,
        }};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i16 {
    
    let bytes = input.as_bytes();
    let left_paren = u8x64::splat(b'(');
    let right_paren = u8x64::splat(b')');

    // Process 64-byte chunks in parallel using SIMD
    let chunks = bytes.chunks_exact(64);
    let remainder = chunks.remainder();

    let simd_sum: i16 = chunks.map(|chunk| {
        let simd_slice = u8x64::from_slice(chunk);

        // Count left parentheses
        let left_mask = simd_slice.simd_eq(left_paren).to_bitmask();
        let opens = left_mask.count_ones() as i16;

        // Count right parentheses
        let right_mask = simd_slice.simd_eq(right_paren).to_bitmask();
        let closes = right_mask.count_ones() as i16;

        opens - closes
    }).sum();

    // Fall back to standard iteration for any trailing bytes (< 64 bytes)
    let remainder_sum: i16 = remainder.iter().fold(0_i16, |acc, &b| match b {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => acc,
    });

    simd_sum + remainder_sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut floor = 0_i16;

    for (i, b) in input.bytes().enumerate() {
        if b == b'(' {floor += 1} else {floor -= 1}

        if floor == -1 {
            return i + 1;
        }
    }
    0
}
