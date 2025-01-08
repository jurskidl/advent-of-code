use std::arch::x86_64::{_mm256_madd_epi16, _mm256_maddubs_epi16};
use std::simd::prelude::*;

unsafe fn parse_numbers(two_lines:Simd<u8, 32>) -> (u32, u32, u32, u32) {
    let adj_chunk = simd_swizzle!(two_lines, [
        00, 01, 02, 03, 04, 05, 06, 07, //
        08, 09, 10, 11, 12, 13, 05, 05, //
        14, 15, 16, 17, 18, 19, 20, 21, //
        22, 23, 24, 25, 26, 27, 19, 19, //
    ])- u8x32::from_array([
        b'0', b'0', b'0', b'0', b'0', b' ', b' ', b' ', //
        b'0', b'0', b'0', b'0', b'0', b'\n', b' ', b' ', //
        b'0', b'0', b'0', b'0', b'0', b' ', b' ', b' ', //
        b'0', b'0', b'0', b'0', b'0', b'\n', b' ', b' ', //
    ]);
    let step1 = unsafe{ _mm256_maddubs_epi16(
        adj_chunk.into(),
        u8x32::from_array([
            10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, //
        ])
        .into(),
    ) };
    let step2: u16x16 = unsafe {_mm256_madd_epi16(
        step1,
        u16x16::from_array([
            100, 1, 1, 0, //
            100, 1, 1, 0, //
            100, 1, 1, 0, //
            100, 1, 1, 0, //
            ]).into(),
    )
    .into()};
    let step3 =
        simd_swizzle!(step2, [
        00, 02, 04, 06, //
        00, 02, 04, 06, //
        08, 10, 12, 14, //
        08, 10, 12, 14, //
        ]).into();

    let fin: u32x8 =
        _mm256_madd_epi16(
            step3,
            u16x16::from_array([
                10, 1, 10, 1, 0, 0, 0, 0, 
                10, 1, 10, 1, 0, 0, 0, 0
                ]).into(),
        )
        .into();

    (fin[0],fin[4],fin[1],fin[5])
}

fn scan_file(input: &[u8]) -> ([u32;1000], [u32;1000]) {
    let mut first: [u32; 1000] = [0; 1000];
    let mut second: [u32; 1000] = [0; 1000];

    let mut i = 0;
    let mut index = 1;
    while i < 13999 - 14 {
        let two_lines = unsafe{ (input.get_unchecked(i) as *const _ as *const u8x32).read_unaligned() };
        unsafe { 
            (
                *first.get_unchecked_mut(index-1),
                *first.get_unchecked_mut(index),
                *second.get_unchecked_mut(index-1),
                *second.get_unchecked_mut(index),
            ) = parse_numbers(two_lines)
        }
        i += 28;
        index += 2;
    }

    first.sort_unstable();
    second.sort_unstable();

    (first, second)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let (first, second) = scan_file(bytes);

    first
        .iter()
        .zip(second)
        .fold(0, |acc, (one, two)| acc + one.abs_diff(two))
}

unsafe fn parse_numbers2(two_lines:Simd<u8, 32>) -> (usize, usize, usize, usize) {
    let adj_chunk = simd_swizzle!(two_lines, [
        00, 01, 02, 03, 04, 05, 06, 07, //
        08, 09, 10, 11, 12, 13, 05, 05, //
        14, 15, 16, 17, 18, 19, 20, 21, //
        22, 23, 24, 25, 26, 27, 19, 19, //
    ])- u8x32::from_array([
        b'0', b'0', b'0', b'0', b'0', b' ', b' ', b' ', //
        b'0', b'0', b'0', b'0', b'0', b'\n', b' ', b' ', //
        b'0', b'0', b'0', b'0', b'0', b' ', b' ', b' ', //
        b'0', b'0', b'0', b'0', b'0', b'\n', b' ', b' ', //
    ]);
    let step1 = unsafe{ _mm256_maddubs_epi16(
        adj_chunk.into(),
        u8x32::from_array([
            10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, //
        ])
        .into(),
    ) };
    let step2: u16x16 = unsafe {_mm256_madd_epi16(
        step1,
        u16x16::from_array([
            100, 1, 1, 0, //
            100, 1, 1, 0, //
            100, 1, 1, 0, //
            100, 1, 1, 0, //
            ]).into(),
    )
    .into()};
    let step3 =
        simd_swizzle!(step2, [
        00, 02, 04, 06, //
        00, 02, 04, 06, //
        08, 10, 12, 14, //
        08, 10, 12, 14, //
        ]).into();

    let fin: u32x8 =
        _mm256_madd_epi16(
            step3,
            u16x16::from_array([
                10, 1, 10, 1, 0, 0, 0, 0, 
                10, 1, 10, 1, 0, 0, 0, 0
                ]).into(),
        )
        .into();

    (fin[0] as usize,fin[4] as usize,fin[1] as usize,fin[5] as usize)
}

fn scan_file2(input: &[u8]) -> ([usize;1000], [usize;1000]) {
    let mut first: [usize; 1000] = [0; 1000];
    let mut second: [usize; 1000] = [0; 1000];

    let mut i = 0;
    let mut index = 1;
    while i < 13999 - 14 {
        let two_lines = unsafe{ (input.get_unchecked(i) as *const _ as *const u8x32).read_unaligned() };
        unsafe { 
            (
                *first.get_unchecked_mut(index-1),
                *first.get_unchecked_mut(index),
                *second.get_unchecked_mut(index-1),
                *second.get_unchecked_mut(index),
            ) = parse_numbers2(two_lines)
        }
        i += 28;
        index += 2;
    }

    first.sort_unstable();
    second.sort_unstable();

    (first, second)
}

fn get_freq(
    first: &[usize; 1000],
    second: &[usize; 1000],
    mut pos1: usize,
    mut pos2: usize,
    mut sum: usize,
) -> usize {
    while pos1 < 1000 && pos2 < 1000 {
        let mut count1 = 1;
        let mut count2 = 0;
        match first[pos1] {
            x if x > second[pos2] => {
                while pos2 < 1000 && first[pos1] > second[pos2] {
                    pos2 += 1
                }
            }
            x if x < second[pos2] => {
                while pos1 < 1000 && first[pos1] < second[pos2] {
                    pos1 += 1
                }
            }
            _ => {
                while pos2 < 1000 && first[pos1] == second[pos2] {
                    count2 += 1;
                    pos2 += 1;
                }
                while pos1 < 999 && first[pos1] == first[pos1 + 1] {
                    count1 += 1;
                    pos1 += 1;
                }
            }
        }
        sum += first[pos1] * count1 * count2;
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    let (first, second) = scan_file2(bytes);
    get_freq(&first, &second, 0, 0, 0)
}

mod tests {
    #[allow(unused)]
    use crate::day1::{part1, part2};
    #[allow(unused)]
    use std::fs::read_to_string;

    #[test]
    fn test_day1() {
        let input = read_to_string("./input/2024/day1.txt").unwrap();
        let input = input.as_str();

        println!("{}", part1(input));
        assert_eq!(part1(input), 1830467);
        assert_eq!(part2(input), 26674158)
    }
}
