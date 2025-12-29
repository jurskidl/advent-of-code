fn get_num(line_buffer: &[u8]) -> i16 {
    let sign = if line_buffer[0] == b'L' { -1 } else { 1 };
    let turn = line_buffer[1..]
        .iter()
        .fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32);

    (sign * turn as i32) as i16
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i16 {
    let bytes = input.as_bytes();
    let num_zero = bytes
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .scan(50_i16, |acc, line| {
            *acc = (*acc + get_num(line)).rem_euclid(100);
            Some(*acc)
        })
        .filter(|&x| x == 0)
        .count() as i16;

    num_zero
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u16 {
    let bytes = input.as_bytes();
    let pass_zero = bytes
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .scan(50_i16, |acc, line| {
            let movement = get_num(line);
            let end = *acc + movement;

            let pass_zero = if movement.is_negative() {
                ((*acc - 1).div_euclid(100) - (end - 1).div_euclid(100)) as u16
            } else {
                end.div_euclid(100) as u16
            };

            *acc = (*acc + movement).rem_euclid(100);
            Some(pass_zero)
        })
        .sum::<u16>();
    pass_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_p2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(part2(input), 6);
    }
}
