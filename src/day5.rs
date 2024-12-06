const NEWLINE: u8 = 10;
const COMMA: u8 = 44;

fn get_vals(line: &[u8]) -> (usize, usize) {
    (
        (0_usize..2_usize)
            .into_iter()
            .map(|x| unsafe {
                ((*line.get_unchecked(x) as usize - 48usize) * 10usize.pow(((2 - 1) - x) as u32))
                    as usize
            })
            .sum(),
        (3_usize..5_usize)
            .into_iter()
            .map(|x| unsafe {
                ((*line.get_unchecked(x) as usize - 48usize) * 10usize.pow(((5 - 1) - x) as u32))
                    as usize
            })
            .sum(),
    )
}

fn get_rules(
    input: &[u8],
    end: usize,
    mut line: usize,
    mut pos: usize,
) -> ([[bool; 100]; 100], usize) {
    let mut rules: [[bool; 100]; 100] = [[false; 100]; 100];

    while input[pos - 1] != NEWLINE && input[pos] != NEWLINE && pos < end {
        let (page, before) = get_vals(&input[line * 6..(line * 6) + 5]);
        rules[page][before] = true;
        pos += 1;
        line += 1;
    }

    (rules, pos)
}

fn check_order(line: &Vec<usize>, rules: [[bool; 100]; 100]) -> bool {
    for page in 0..line.len() - 1 {
        if !rules[page][page + 1] {
            return false;
        }
    }
    true
}

fn get_solution(
    input: &[u8],
    rules: [[bool; 100]; 100],
    mut pos: usize,
    end: usize,
    mut sum: u16,
) -> u16 {
    while pos < end {
        let mut line: Vec<usize> = Vec::with_capacity(23);
        while pos < end && input[pos] != NEWLINE {
            if input[pos] != COMMA {
                pos += 1;
                line.push(
                    (pos - 1..pos + 1)
                        .into_iter()
                        .map(|x| unsafe {
                            ((*input.get_unchecked(x) as usize - 48usize)
                                * 10usize.pow((pos - x) as u32))
                                as usize
                        })
                        .sum(),
                )
            } else {
            }
            pos += 1
        }
        sum += if check_order(&line, rules) {
            line[line.len() >> 1] as u16
        } else {
            0
        };
        pos += 1;
    }
    sum
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u16 {
    let input = input.as_bytes();
    let end = input.len();
    let (rules, pos) = get_rules(input, end, 0, 1);
    get_solution(&input[pos..end], rules, pos, end, 0)
}
