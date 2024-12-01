#[aoc(day1, part1)]
pub fn day1(input: &str) -> u32 {
    
    let mut left: [u32; 1000] = [0;1000];
    let mut right: [u32; 1000] = [0;1000];

    let mut count = 0;

    let _: Vec<_> = input
        .lines()
        .map(|line| {
            unsafe {
                left[count] = u32::from_str_radix(&line[0..5], 10).unwrap_unchecked();
                right[count] = u32::from_str_radix(&line[8..13], 10).unwrap_unchecked();
            }
            count +=1;
        }).collect();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .fold(0, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs))
}