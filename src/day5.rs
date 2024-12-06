const NEWLINE: u8 = 10;
const NUMRULES: usize = 1176;

fn get_vals(line: &[u8]) -> (usize,usize) {
    ((0_usize..2_usize).into_iter().map(|x| 
        unsafe{ 
            ((*line.get_unchecked(x) as usize - 48usize) * 10usize.pow(((2-1) - x) as u32)) as usize
        }
    ).sum(),
    (3_usize..5_usize).into_iter().map(|x| 
        unsafe{ 
            ((*line.get_unchecked(x) as usize - 48usize) * 10usize.pow(((5-1) - x) as u32)) as usize
        }
    ).sum())
}

fn get_rules(input: &[u8], mut pos: usize) -> [Vec<usize>; 100] {
    let mut pages: Vec<usize> = Vec::with_capacity(100);
    let mut rules: [Vec<usize>; 100] = vec![pages; 100].try_into().expect("failed into array of vectors");

    for line in 0..NUMRULES {
        let (pages, before) = get_vals(&input[line*6..(line*6)+5]);
        rules[pages].push(before)
    }
    
    (0..100).into_iter().for_each(|x| rules[x].sort_unstable());

    println!("{:?}",rules);
    rules
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u16 {
    let input = input.as_bytes();
    let pages = get_rules(input, 0);

    10
}