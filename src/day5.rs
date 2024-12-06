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

// const UPDATE_MAX_SIZE: usize = 23; // real size is 23, we store 2 extra values. 1 for the middle value and 1 for the size of the update
// const UPDATES_LEN: usize = 220;
// const MAX_SIZE: usize = 100;
// type AdjMatrix = [[bool; MAX_SIZE]; MAX_SIZE];

// fn parse(input: &str) -> (AdjMatrix, [[u8; UPDATE_MAX_SIZE + 2]; UPDATES_LEN]) {
//     let mut it = input.split("\n\n");
//     let mut matrix = [[false; MAX_SIZE]; MAX_SIZE];

//     let mut updates = [[0; UPDATE_MAX_SIZE + 2]; UPDATES_LEN];
//     it.next().unwrap().lines().enumerate().for_each(|(i, l)| {
//         let mut peekable = l.split(',').peekable();
//         let mut j = 0;
//         while let Some(v) = peekable.peek() {
//             updates[i][j] = v.parse().unwrap();
//             j += 1;
//             peekable.next();
//         }
//         updates[i][UPDATE_MAX_SIZE] = updates[i][j / 2];
//         updates[i][UPDATE_MAX_SIZE + 1] = j as u8;
//     });

//     (matrix, updates)
// }

// fn is_update_correct_order(update: &[u8], matrix: &AdjMatrix) -> bool {
//     for i in 0..UPDATE_MAX_SIZE - 1 {
//         let a = update[i];
//         for &b in &update[i + 1..UPDATE_MAX_SIZE] {
//             if b == 0 {
//                 break;
//             }
//             if matrix[b as usize][a as usize] {
//                 return false;
//             }
//         }
//     }

//     true
// }

// pub fn part1(input: &str) -> u16 {
//     let (matrix, updates) = parse(input);

//     updates.into_iter().fold(0, |acc, update| {
//         acc + if is_update_correct_order(&update, &matrix) {
//             update[UPDATE_MAX_SIZE] as u16
//         } else {
//             0
//         }
//     })
// }

// fn sort_update(update: &mut [u8], matrix: &AdjMatrix) {
//     let len = update[UPDATE_MAX_SIZE + 1] as usize;
//     for _i in 0..len {
//         for j in 0..len - 1 {
//             let a = update[j] as usize;
//             let b = update[j + 1] as usize;
//             if matrix[b][a] {
//                 update.swap(j, j + 1);
//             }
//         }
//     }
//     update[UPDATE_MAX_SIZE] = update[len / 2];
// }

// pub fn part2(input: &str) -> u16 {
//     let (matrix, updates) = parse(input);

//     updates.into_iter().fold(0, |acc, mut update| {
//         if is_update_correct_order(&update, &matrix) {
//             return acc;
//         }
//         sort_update(&mut update, &matrix);
//         acc + update[UPDATE_MAX_SIZE] as u16
//     })
// }
