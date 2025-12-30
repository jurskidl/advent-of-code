fn get_num(line: &[u8]) -> u32 {
    line[0..]
        .iter()
        .fold(0u32, |acc, &b| acc * 10 + (b - b'0') as u32)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u16 {
    let mut lines = input.as_bytes().split(|&b| b == b'\n');

    let (mut prev_val, current_val) = unsafe {
        (
            get_num(lines.next().unwrap_unchecked()),
            get_num(lines.next().unwrap_unchecked()),
        )
    };

    let mut count = if current_val > prev_val { 1u16 } else { 0u16 };

    for line in lines {
        let current_val = get_num(line);

        if current_val > prev_val {
            count += 1;
        }
        prev_val = current_val;
    }

    count
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u16 {
    let mut lines = input.as_bytes().split(|&b| b == b'\n');

    let (mut prev_prev_val, mut prev_val, current_val) = unsafe {
        (
            get_num(lines.next().unwrap_unchecked()),
            get_num(lines.next().unwrap_unchecked()),
            get_num(lines.next().unwrap_unchecked()),
        )
    };

    let mut prev_total = prev_prev_val + prev_val + current_val;
    let mut count = 0u16;

    for line in lines {
        let current_val = get_num(line);
        let current_total = current_val + prev_prev_val + prev_val;

        if current_total > prev_total {
            count += 1;
        }

        prev_total = current_total;
        prev_prev_val = prev_val;
        prev_val = current_val;
    }

    count
}
