use std::simd::{Simd, cmp::SimdPartialEq, u8x16};

const A: Simd<u8, 16> = u8x16::splat(b'a');
const E: Simd<u8, 16> = u8x16::splat(b'e');
const I: Simd<u8, 16> = u8x16::splat(b'i');
const O: Simd<u8, 16> = u8x16::splat(b'o');
const U: Simd<u8, 16> = u8x16::splat(b'u');

const BAD_PAIRS: [[u8; 2]; 4] = [
        [b'a', b'b'],
        [b'c', b'd'],
        [b'p', b'q'],
        [b'x', b'y'],
];

#[inline(always)]
fn check_vowels(line: Simd<u8, 16>) -> u32 {
    let mut vowels = line.simd_eq(A).to_bitmask().count_ones();
    vowels += line.simd_eq(E).to_bitmask().count_ones();
    vowels += line.simd_eq(I).to_bitmask().count_ones();
    vowels += line.simd_eq(O).to_bitmask().count_ones();
    vowels + line.simd_eq(U).to_bitmask().count_ones()
}

#[inline(always)]
fn bad_strings(line: u8x16, shifted: u8x16) -> bool {
    for [i, j] in BAD_PAIRS {
        let match_i = line.simd_eq(u8x16::splat(i));
        let match_j = shifted.simd_eq(u8x16::splat(j));
        if (match_i & match_j).to_bitmask() & 0xFFFE != 0 { return true }
    }
    false
}

#[inline(always)]
fn repeats(line: u8x16, shifted: u8x16) -> bool {
    line.simd_eq(shifted).to_bitmask() & 0xFFFE != 0
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let mut good_strings = 0;
    
    let input = input.as_bytes();
    let mut index = 0_usize;
    while index < input.len() {
        index += 17;
        let simd_line= u8x16::from_slice(&input[index - 17..index - 1]);
        let shifted = simd_line.rotate_elements_left::<1>();

        let bad_strings = bad_strings(simd_line, shifted);
        if bad_strings { continue }
        let repeats = repeats(simd_line, shifted);
        if !repeats { continue }
        let num_vowels = check_vowels(simd_line);
        if num_vowels > 3 { good_strings += 1 } else { continue };
    }
    good_strings
}

fn has_repeat_pair(arr: &[u8]) -> bool {
    for i in 0..14 {
        let a = arr[i];
        let b = arr[i + 1];
        for j in i + 2..15 {
            if arr[j] == a && arr[j + 1] == b {
                return true;
            }
        }
    }
    false
}

fn has_repeat_skip(line: u8x16) -> bool {
    let shifted2 = line.rotate_elements_left::<2>();
    line.simd_eq(shifted2).to_bitmask() & 0x3FFF != 0
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let mut good_strings = 0;
    
    let input = input.as_bytes();
    let mut index = 0_usize;
    while index < input.len() {
        index += 17;
        let simd_line= u8x16::from_slice(&input[index - 17..index - 1]);

        if has_repeat_pair(&input[index - 17.. index - 1]) && has_repeat_skip(simd_line) { good_strings += 1 }
    }
    good_strings
}
