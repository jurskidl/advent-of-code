#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            unsafe {
            (
                u32::from_str_radix(&line[0..5], 10).unwrap_unchecked(),
                u32::from_str_radix(&line[8..13], 10).unwrap_unchecked(),
            )
        }}).unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .fold(0, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs))
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let (left, right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| {
            unsafe {
            (
                usize::from_str_radix(&line[0..5], 10).unwrap_unchecked(),
                usize::from_str_radix(&line[8..13], 10).unwrap_unchecked(),
            )
        }}).unzip();
    
    let mut freq: [usize; 89999] = [0; 89999]; 
    right.into_iter().for_each(|value: usize| freq[value-10000]+=1);

    left.iter()
        .fold(0, |acc: usize, value: &usize| acc + *value * freq[*value-10000])

}