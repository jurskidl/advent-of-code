use rayon::prelude::*;
use std::simd::{
        u32x8,
        prelude::SimdPartialEq,
        Simd};

#[inline(always)]
fn rotate_left_x8(x: u32x8, n: u32) -> u32x8 {
    (x << u32x8::splat(n)) | (x >> u32x8::splat(32 - n))
}

fn md5_x8(messages: &[[u8; 64]; 8]) -> Simd<u32, 8> {
    const S: [u32; 64] = [
         7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
         5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
         4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
         6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a,
        0xa8304613, 0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340,
        0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8,
        0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92,
        0xffeff47d, 0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    // Load all 16 message words for each of the 8 lanes into 16 SIMD vectors.
    // m[w] holds word w from all 8 messages simultaneously.
    let mut m = [u32x8::splat(0); 16];
    for w in 0..16 {
        m[w] = u32x8::from_array(std::array::from_fn(|lane| {
            u32::from_le_bytes(messages[lane][w * 4..w * 4 + 4].try_into().unwrap())
        }));
    }

    let mut a = u32x8::splat(0x67452301);
    let mut b = u32x8::splat(0xefcdab89);
    let mut c = u32x8::splat(0x98badcfe);
    let mut d = u32x8::splat(0x10325476);

    for i in 0u32..64 {
        let (f, g) = match i {
            0..=15  => ((b & c) | (!b & d),      i),
            16..=31 => ((d & b) | (!d & c),      (5 * i + 1) % 16),
            32..=47 => (b ^ c ^ d,               (3 * i + 5) % 16),
            _       => (c ^ (b | !d),            (7 * i) % 16),
        };

        let f = f + a + u32x8::splat(K[i as usize]) + m[g as usize];

        a = d;
        d = c;
        c = b;
        b = b + rotate_left_x8(f, S[i as usize]);
    }

    // Finalize a first and check before computing b, c, d
    a + u32x8::splat(0x67452301)
}

fn md5x8_d1(messages: &[[u8; 64]; 8]) -> Option<usize> {
    let a = md5_x8(messages);
    let hits = (a & u32x8::splat(0x00f0ffff)).simd_eq(u32x8::splat(0));
    if hits.any() {
        Some(hits.to_bitmask().trailing_zeros() as usize)
    } else {
        None
    }
}

/// Pad a variable-length message into a fixed 64-byte MD5 block.
/// Only valid for messages that fit in a single MD5 block (< 56 bytes).
fn pad_single_block(msg: &[u8]) -> [u8; 64] {
    assert!(msg.len() < 56, "message too long for single-block padding");
    let mut block = [0u8; 64];
    block[..msg.len()].copy_from_slice(msg);
    block[msg.len()] = 0x80;
    let bit_len = (msg.len() as u64) * 8;
    block[56..64].copy_from_slice(&bit_len.to_le_bytes());
    block
}

fn write_u64_decimal(buf: &mut Vec<u8>, mut n: u64) {
    let start = buf.len();
    if n == 0 {
        buf.push(b'0');
        return;
    }
    while n > 0 {
        buf.push(b'0' + (n % 10) as u8);
        n /= 10;
    }
    buf[start..].reverse();
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    let prefix = input.trim_end().as_bytes();
    assert!(prefix.len() < 50, "prefix too long");
    
    // Each thread processes 8192 candidates at a time
    (0u64..)
    .map(|chunk| chunk * 8192 + 1)
    .find_map(|start| {
        (0..1024)
            .into_par_iter()
            .map(|i| start + i as u64 * 8)
            .find_map_first(|n| {
                let blocks: [[u8; 64]; 8] = std::array::from_fn(|lane| {
                    let mut msg = prefix.to_vec();
                    write_u64_decimal(&mut msg, n + lane as u64);
                    pad_single_block(&msg)
                });

                md5x8_d1(&blocks).map(|lane| n + lane as u64)
            })
    })
    .expect("Failed to find suitable solution")
}

fn md5x8_d2(messages: &[[u8; 64]; 8]) -> Option<usize> {
    let a = md5_x8(messages);
    let hits = (a & u32x8::splat(0x00ffffff)).simd_eq(u32x8::splat(0));

    if hits.any() {
        Some(hits.to_bitmask().trailing_zeros() as usize)
    } else {
        None
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let prefix = input.trim_end().as_bytes();
    assert!(prefix.len() < 50, "prefix too long");
    
    // Each thread processes 8192 candidates at a time
    (0u64..)
    .map(|chunk| chunk * 8192 + 1)
    .find_map(|start| {
        (0..1024)
            .into_par_iter()
            .map(|i| start + i as u64 * 8)
            .find_map_first(|n| {
                let blocks: [[u8; 64]; 8] = std::array::from_fn(|lane| {
                    let mut msg = prefix.to_vec();
                    write_u64_decimal(&mut msg, n + lane as u64);
                    pad_single_block(&msg)
                });
                md5x8_d2(&blocks).map(|lane| n + lane as u64)
            })
    })
    .expect("Failed to find suitable solution")
}
