fn process_line(line_buffer: &[u8]) -> u32 {
    let len = line_buffer.len();
    let mut pos = 0;
    while pos < len && !line_buffer[pos].is_ascii_digit() {
        pos += 1;
    }
    let first_num = (line_buffer[pos] - b'0') as u32;

    let mut pos = len - 1;
    while pos > 0 && !line_buffer[pos].is_ascii_digit() {
        pos -= 1;
    }
    let second_num = (line_buffer[pos] - b'0') as u32;

    first_num * 10 + second_num
}

// ottffssen

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| process_line(line))
        .sum()
}

fn process_line_2(line_buffer: &[u8]) -> u32 {
    let len = line_buffer.len();

    let mut pos = 0;
    let first_num = loop {
        if pos >= len {
            break 0;
        } // Fallback if no digit found

        let val = match (line_buffer[pos], &line_buffer[pos..]) {
            (b'0'..=b'9', _) => Some((line_buffer[pos] - b'0') as u32),
            (b'o', s) if s.starts_with(b"one") => Some(1),
            (b't', s) if s.starts_with(b"two") => Some(2),
            (b't', s) if s.starts_with(b"three") => Some(3),
            (b'f', s) if s.starts_with(b"four") => Some(4),
            (b'f', s) if s.starts_with(b"five") => Some(5),
            (b's', s) if s.starts_with(b"six") => Some(6),
            (b's', s) if s.starts_with(b"seven") => Some(7),
            (b'e', s) if s.starts_with(b"eight") => Some(8),
            (b'n', s) if s.starts_with(b"nine") => Some(9),
            _ => None,
        };

        if let Some(digit) = val {
            break digit; // This returns the u32 value to first_num
        }
        pos += 1;
    };

    let mut pos = len - 1;
    let second_num = loop {
        let val = match (line_buffer[pos], &line_buffer[pos..]) {
            (b'0'..=b'9', _) => Some((line_buffer[pos] - b'0') as u32),
            (b'o', s) if s.starts_with(b"one") => Some(1),
            (b't', s) if s.starts_with(b"two") => Some(2),
            (b't', s) if s.starts_with(b"three") => Some(3),
            (b'f', s) if s.starts_with(b"four") => Some(4),
            (b'f', s) if s.starts_with(b"five") => Some(5),
            (b's', s) if s.starts_with(b"six") => Some(6),
            (b's', s) if s.starts_with(b"seven") => Some(7),
            (b'e', s) if s.starts_with(b"eight") => Some(8),
            (b'n', s) if s.starts_with(b"nine") => Some(9),
            _ => None,
        };

        if let Some(digit) = val {
            break digit; // This returns the u32 value to first_num
        }

        if pos == 0 {
            break 0;
        }

        pos -= 1;
    };

    first_num * 10 + second_num
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| process_line_2(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5\ntreb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_p2() {
        let input = "two1nine\neightwothree\nabcone2threexy\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
