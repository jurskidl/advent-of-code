fn get_num(line: &[u8]) -> i32 {
    line.iter()
        .fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();
    input
        .split(|&b| b == b'\n')
        .fold(0, |acc, line| acc + (get_num(line) / 3) - 2)
}

fn get_recursive_fuel(weight: i32) -> i32 {
    let fuel = (weight / 3) - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + get_recursive_fuel(fuel)
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let input = input.as_bytes();

    input
        .split(|&b| b == b'\n')
        .map(|line| get_num(line))
        .map(|module_weight| get_recursive_fuel(module_weight))
        .sum()
}
