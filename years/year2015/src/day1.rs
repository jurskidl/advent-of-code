#[aoc(day1, part1)]
pub fn part1(input: &str) -> i16 {
    input.bytes().fold(0_i16, |acc, b| match b {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => acc,
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut floor = 0_i16;

    for (i, b) in input.bytes().enumerate() {
        match b {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => continue,
        }

        if floor == -1 {
            return i + 1;
        }
    }
    0
}
