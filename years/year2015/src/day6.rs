use std::simd::{u64x8};

pub enum Command {
    TurnOff,
    TurnOn,
    Toggle
}

#[inline(always)]
fn get_command(line: &[u8]) -> Command {
    match (line[0], line[1]) {
        (b'o', b'f') => Command::TurnOff,
        (b'o', b'n') => Command::TurnOn,
        _ => Command::Toggle,
    }
}

#[inline(always)]
fn parse_num(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |number, &digit| {
        number * 10 + (digit - b'0') as usize
    })
}

#[inline(always)]
fn get_indexes(line: &[u8]) -> (usize, usize, usize, usize, usize) {
    let mut comma1 = None;
    let mut space1 = None;
    let mut comma2 = None;
    let mut endline = None;

    for (index, &byte) in line.iter().enumerate() {
        match byte {
            b',' if comma1.is_none() => comma1 = Some(index),
            b' ' if space1.is_none() => space1 = Some(index),
            b',' => comma2 = Some(index),
            b'\n' => {
                endline = Some(index);
                break;
            }

            _ => {}
        }
    }

    let comma1 = comma1.expect("missing first comma");
    let space1 = space1.expect("missing ' through '");
    let comma2 = comma2.expect("missing second comma");
    let endline = endline.unwrap_or(line.len());

    let x2_start = space1 + b" through ".len();

    (
        parse_num(&line[..comma1]),
        parse_num(&line[comma1 + 1..space1]),
        parse_num(&line[x2_start..comma2]),
        parse_num(&line[comma2 + 1..endline]),
        endline,
    )
}

fn build_mask(
    x1: usize,
    x2: usize,
    y: usize,
) -> (usize, usize, u64, u64) {
    let first_bit = y * 1000 + x1;
    let last_bit = y * 1000 + x2;

    let first_64 = first_bit / 64;
    let last_64 = last_bit / 64;

    let first_mask = u64::MAX << (first_bit % 64);

    let last_offset = last_bit % 64;
    let last_mask = if last_offset == 63 {
        u64::MAX
    } else {
        (1_u64 << (last_offset + 1)) - 1
    };

    (
        first_64,
        last_64,
        first_mask,
        last_mask,
    )
}

fn turn_off(line: &[u8], grid: &mut [u64; 15625]) -> usize {
    let (x1, y1, x2, y2, endline) = get_indexes(line);

    for y in y1..=y2 {
        let (first_64, last_64, first_mask, last_mask) =
            build_mask(x1, x2, y);

        if first_64 == last_64 {
            grid[first_64] &= !(first_mask & last_mask);
            continue;
        }

        grid[first_64] &= !first_mask;
        grid[last_64] &= !last_mask;

        let middle = &mut grid[first_64 + 1..last_64];
        let mut chunks = middle.chunks_exact_mut(8);

        for chunk in &mut chunks {
            u64x8::splat(0).copy_to_slice(chunk);
        }

        for remainder in chunks.into_remainder() {
            *remainder = 0;
        }
    }
    endline
}

fn turn_on(line: &[u8], grid: &mut [u64; 15625]) -> usize {
    let (x1, y1, x2, y2, endline) = get_indexes(line);

    for y in y1..=y2 {
        let (first_64, last_64, first_mask, last_mask) =
            build_mask(x1, x2, y);

        if first_64 == last_64 {
            grid[first_64] |= first_mask & last_mask;
            continue;
        }

        grid[first_64] |= first_mask;
        grid[last_64] |= last_mask;

        let middle = &mut grid[first_64 + 1..last_64];
        let mut chunks = middle.chunks_exact_mut(8);

        for chunk in &mut chunks {
            u64x8::splat(u64::MAX).copy_to_slice(chunk);
        }

        for remainder in chunks.into_remainder() {
            *remainder = u64::MAX;
        }
    }
    endline
}

fn toggle(line: &[u8], grid: &mut [u64; 15625]) -> usize {
    let (x1, y1, x2, y2, endline) = get_indexes(line);

    for y in y1..=y2 {
        let (first_64, last_64, first_mask, last_mask) =
            build_mask(x1, x2, y);

        if first_64 == last_64 {
            grid[first_64] ^= first_mask & last_mask;
            continue;
        }

        grid[first_64] ^= first_mask;
        grid[last_64] ^= last_mask;

        let middle = &mut grid[first_64 + 1..last_64];
        let mut chunks = middle.chunks_exact_mut(8);

        for chunk in &mut chunks {
            let values = u64x8::from_slice(chunk);
            (!values).copy_to_slice(chunk);
        }

        for remainder in chunks.into_remainder() {
            *remainder = !*remainder;
        }
    }
    endline
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let mut index = 0;

    let mut grid = [0u64; 15625];

    let input = input.as_bytes();
    while index < input.len() {
        let command = get_command(&input[index + 5..index + 7]);

        let consumed = match command {
            Command::TurnOff => {
                let endline = turn_off(&input[index + 9..], &mut grid);
                9 + endline
            }
            Command::TurnOn => {
                let endline = turn_on(&input[index + 8..], &mut grid);
                8 + endline
            }
            Command::Toggle => {
                let endline = toggle(&input[index + 7..], &mut grid);
                7 + endline
            }
        };

        index += consumed + 1;
    }

    grid.iter().map(|word| word.count_ones()).sum()
}

// #[aoc(day6, part2)]
// pub fn part2(input: &str) -> u32 {
//     0
// }