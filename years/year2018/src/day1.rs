use std::collections::HashSet;

fn get_num(line: &[u8]) -> i32 {
    let sign = if line[0] == b'+' { 1 } else { -1 };
    sign * line[1..]
        .iter()
        .fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();
    input
        .split(|&b| b == b'\n')
        .fold(0, |acc, line| acc + get_num(line))
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut seen_freq = HashSet::new();
    let mut current_freq = 0;
    seen_freq.insert(0);

    let adjusts: Vec<i32> = input
        .split(|&b| b == b'\n')
        .map(|line| get_num(line))
        .collect();

    adjusts
        .iter()
        .cycle()
        .find_map(|&num| {
            current_freq += num;
            if !seen_freq.insert(current_freq) {
                Some(current_freq)
            } else {
                None
            }
        })
        .unwrap_or(0)
}
