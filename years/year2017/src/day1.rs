#[aoc(day1, part1)]
pub fn part1(input: &str) -> i16 {
    let input = input.as_bytes();
    let len = input.len();
    let mut sum = 0;

    for i in 0..len - 1 {
        if input[i] == input[i + 1] {
            sum += (input[i] - b'0') as i16;
        }
    }

    if input[0] == input[len - 1] {
        sum += (input[0] - b'0') as i16;
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i16 {
    let len = input.len();
    let half_len = len / 2;
    let input = input.as_bytes();

    (0..len).into_iter().fold(0, |acc, i| {
        let target_idx = (i + half_len) % len;
        if input[i] == input[target_idx] {
            acc + (input[i] - b'0') as i16
        } else {
            acc
        }
    })
}
