fn get_num(line: &[u8]) -> u32 {
    line.iter()
        .fold(0u32, |acc, &b| acc * 10 + (b - b'0') as u32)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut max_calories = 0u32;
    let mut current_elf_sum = 0u32;

    let mut pos = 1;
    let mut line_start = 0;

    while pos < input.len() {
        match (input[pos - 1], input[pos]) {
            (b'\n', b'\n') => {
                current_elf_sum += get_num(&input[line_start..pos]);
                max_calories = max_calories.max(current_elf_sum);
                current_elf_sum = 0;
                pos += 1;
                line_start = pos;
            }
            (_, b'\n') => {
                current_elf_sum += get_num(&input[line_start..pos]);
                pos += 1;
                line_start = pos;
            }
            _ => {
                pos += 1;
            }
        }
    }

    current_elf_sum += get_num(&input[line_start..pos - 1]);
    max_calories.max(current_elf_sum)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut top_three = [0u32; 3];
    let input = input.as_bytes();
    let mut current_elf_sum = 0u32;

    let mut pos = 1;
    let mut line_start = 0;

    while pos < input.len() {
        match (input[pos - 1], input[pos]) {
            (b'\n', b'\n') => {
                current_elf_sum += get_num(&input[line_start..pos]);
                top_three.sort_unstable();
                top_three[0] = top_three[0].max(current_elf_sum);
                current_elf_sum = 0;
                pos += 1;
                line_start = pos;
            }
            (_, b'\n') => {
                current_elf_sum += get_num(&input[line_start..pos]);
                pos += 1;
                line_start = pos;
            }
            _ => {
                pos += 1;
            }
        }
    }

    current_elf_sum += get_num(&input[line_start..pos - 1]);
    top_three.sort_unstable();
    top_three[0] = top_three[0].max(current_elf_sum);
    top_three.iter().sum()
}
