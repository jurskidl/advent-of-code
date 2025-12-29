const NEWLINE: u8 = 10;
const SPACE: u8 = 32;

fn get_twovals(line: &[u8], mut pos: usize, end: usize) -> (u8, u8, usize) {
    let val1_start = pos;
    while pos < end && line[pos] != SPACE {
        pos += 1;
    }
    let val2_start = pos + 1;
    let mut temp_pos = val2_start;
    while temp_pos < end && line[temp_pos] != SPACE {
        temp_pos += 1;
    }

    let val1 = line[val1_start..pos]
        .iter()
        .fold(0u8, |acc, &b| acc * 10 + (b - b'0') as u8);

    let val2 = line[val2_start..temp_pos]
        .iter()
        .fold(0u8, |acc, &b| acc * 10 + (b - b'0') as u8);

    (
        val1,
        val2,
        if temp_pos != end {
            val2_start
        } else {
            temp_pos
        },
    )
}

fn get_lines(line: &[u8]) -> bool {
    let end = line.len();

    let (mut prev_val, mut val, mut pos) = get_twovals(line, 0, end);
    let diff = val as i8 - prev_val as i8;
    if diff.abs() > 3 || diff == 0 {
        return false;
    }
    let prev_sign = if diff.signum() == 1 { true } else { false };

    while pos < end {
        (prev_val, val, pos) = get_twovals(line, pos, end);
        let diff = val as i8 - prev_val as i8;
        let sign = if diff.signum() == 1 { true } else { false };
        if diff.abs() > 3 || diff == 0 || prev_sign != sign {
            return false;
        }
    }
    true
}

fn get_safes(buffer: &[u8], end: usize) -> u16 {
    let mut safes = [false; 1000];
    let mut pos = 0;
    let mut start_line = 0;

    for lines in 0..1000 {
        while pos < end && buffer[pos] != NEWLINE {
            pos += 1
        }

        let line = &buffer[start_line..pos];
        let line = get_lines(line);
        safes[lines] = line;
        pos += 1;
        start_line = pos;
    }

    safes.into_iter().fold(0, |acc, x| acc + x as u16)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u16 {
    let input: &[u8] = input.as_bytes();

    get_safes(input, input.len())
}

fn get_twoval(line: &[u8], mut pos: usize, end: usize) -> (u8, u8, usize) {
    let val1_start = pos;
    while pos < end && line[pos] != SPACE {
        pos += 1;
    }
    let val2_start = pos + 1;
    let mut temp_pos = val2_start;
    while temp_pos < end && line[temp_pos] != SPACE {
        temp_pos += 1;
    }

    let val1 = line[val1_start..pos]
        .iter()
        .fold(0u8, |acc, &b| acc * 10 + (b - b'0') as u8);

    let val2 = line[val2_start..temp_pos]
        .iter()
        .fold(0u8, |acc, &b| acc * 10 + (b - b'0') as u8);

    (
        val1,
        val2,
        if temp_pos != end {
            val2_start
        } else {
            temp_pos
        },
    )
}

fn is_safe_dampened<const MIN: i8, const MAX: i8>(deltas: &Vec<i8>) -> bool {
    let mut dampened = false;
    let mut i = 0;
    while i < deltas.len() {
        let this = deltas[i];
        if this < MIN || this > MAX {
            if dampened {
                return false;
            }
            if i + 1 == deltas.len() {
                return true;
            }
            let next = deltas[i + 1];
            let merged = this + next;
            if merged >= MIN && merged <= MAX {
                dampened = true;
                i += 1;
            } else if next < MIN || next > MAX {
                return false;
            } else {
                if i == 0 {
                    dampened = true;
                } else {
                    let prev = deltas[i - 1];
                    let merged = this + prev;
                    if merged >= MIN && merged <= MAX {
                        dampened = true;
                    } else {
                        return false;
                    }
                }
            }
        }
        i += 1;
    }
    return true;
}

fn get_line(line: &[u8]) -> Vec<i8> {
    let mut diffs: Vec<i8> = Vec::with_capacity(8);
    let end = line.len();

    let (mut prev_val, mut val, mut pos) = get_twoval(line, 0, end);
    diffs.push(val as i8 - prev_val as i8);

    while pos < end {
        (prev_val, val, pos) = get_twoval(line, pos, end);
        diffs.push(val as i8 - prev_val as i8);
    }

    diffs
}

fn get_safe(buffer: &[u8], end: usize) -> u16 {
    let mut safe: u16 = 0;
    let mut pos = 0;
    let mut start_line = 0;

    for _lines in 0..1000 {
        while pos < end && buffer[pos] != NEWLINE {
            pos += 1
        }

        let line = &buffer[start_line..pos];
        let line = get_line(line);
        if is_safe_dampened::<1, 3>(&line) || is_safe_dampened::<-3, -1>(&line) {
            safe += 1;
        }
        pos += 1;
        start_line = pos;
    }
    safe
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u16 {
    let input: &[u8] = input.as_bytes();

    get_safe(input, input.len())
}

mod tests {
    #[allow(unused)]
    use crate::day2::{part1, part2};

    #[test]
    fn test_day2() {
        let test: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part1(test), 2);
        assert_eq!(part2(test), 4)
    }
}
