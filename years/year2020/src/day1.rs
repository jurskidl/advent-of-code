fn get_num(line: &[u8]) -> u32 {
    line[0..]
        .iter()
        .fold(0u32, |acc, &b| acc * 10 + (b - b'0') as u32)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let input = input
        .split(|&b| b == b'\n')
        .map(|line| get_num(line))
        .collect::<Vec<u32>>();
    for x in 0..(input.len()) {
        for y in x..(input.len()) {
            if input[x] + input[y] == 2020 {
                return (input[x] * input[y]) as u32;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let input = input
        .split(|&b| b == b'\n')
        .map(|line| get_num(line))
        .collect::<Vec<u32>>();
    for x in 0..(input.len()) {
        for y in x..(input.len()) {
            for z in y..(input.len()) {
                if input[x] + input[y] + input[z] == 2020 {
                    return (input[x] * input[y] * input[z]) as usize;
                }
            }
        }
    }
    unreachable!()
}
