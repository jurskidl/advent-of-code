fn parse_line(line: &[u8]) -> (u32, u32) {
    (
        (0_usize..5_usize)
            .into_iter()
            .map(|x| unsafe {
                ((*line.get_unchecked(x) as u32 - 48u32) * 10u32.pow(((5 - 1) - x) as u32)) as u32
            })
            .sum(),
        (8_usize..13_usize)
            .into_iter()
            .map(|x| unsafe {
                ((*line.get_unchecked(x) as u32 - 48u32) * 10u32.pow(((13 - 1) - x) as u32)) as u32
            })
            .sum(),
    )
}

fn scan_file(input: &[u8]) -> ([u32; 1000], [u32; 1000]) {
    let mut first: [u32; 1000] = [0; 1000];
    let mut second: [u32; 1000] = [0; 1000];

    for line in 0..1000 {
        unsafe {
            (
                *first.get_unchecked_mut(line),
                *second.get_unchecked_mut(line),
            ) = parse_line(&input[(line * 14)..(line * 14) + 13]);
        }
    }

    first.sort_unstable();
    second.sort_unstable();

    (first, second)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let (first, second) = scan_file(bytes);

    first
        .iter()
        .zip(second)
        .fold(0, |acc, (one, two)| acc + one.abs_diff(two))
}

fn parse_line2(line: &[u8]) -> (usize, usize) {
    (
        (0_usize..5_usize)
            .into_iter()
            .map(|x| unsafe {
                ((*line.get_unchecked(x) as usize - 48usize) * 10usize.pow(((5 - 1) - x) as u32))
                    as usize
            })
            .sum(),
        (8_usize..13_usize)
            .into_iter()
            .map(|x| unsafe {
                ((*line.get_unchecked(x) as usize - 48usize) * 10usize.pow(((13 - 1) - x) as u32))
                    as usize
            })
            .sum(),
    )
}

fn scan_file2(input: &[u8]) -> ([usize; 1000], [usize; 1000]) {
    let mut first: [usize; 1000] = [0; 1000];
    let mut second: [usize; 1000] = [0; 1000];

    for line in 0..1000 {
        unsafe {
            (
                *first.get_unchecked_mut(line),
                *second.get_unchecked_mut(line),
            ) = parse_line2(&input[(line * 14)..(line * 14) + 13]);
        }
    }

    first.sort_unstable();
    second.sort_unstable();

    (first, second)
}

fn get_freq(
    first: &[usize; 1000],
    second: &[usize; 1000],
    mut pos1: usize,
    mut pos2: usize,
    mut sum: usize,
) -> usize {
    while pos1 < 1000 && pos2 < 1000 {
        let mut count1 = 1;
        let mut count2 = 0;
        match first[pos1] {
            x if x > second[pos2] => {
                while pos2 < 1000 && first[pos1] > second[pos2] {
                    pos2 += 1
                }
            }
            x if x < second[pos2] => {
                while pos1 < 1000 && first[pos1] < second[pos2] {
                    pos1 += 1
                }
            }
            _ => {
                while pos2 < 1000 && first[pos1] == second[pos2] {
                    count2 += 1;
                    pos2 += 1;
                }
                while pos1 < 999 && first[pos1] == first[pos1 + 1] {
                    count1 += 1;
                    pos1 += 1;
                }
            }
        }
        sum += first[pos1] * count1 * count2;
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    let (first, second) = scan_file2(bytes);
    get_freq(&first, &second, 0, 0, 0)
}

mod tests {
    #[allow(unused)]
    use crate::day1::{part1, part2};
    #[allow(unused)]
    use std::fs::read_to_string;

    #[test]
    fn test_day1() {
        let input = read_to_string("./input/2024/day1.txt").unwrap();
        let input = input.as_str();

        println!("{}", part1(input));
        assert_eq!(part1(input), 1830467);
        assert_eq!(part2(input), 26674158)
    }
}
