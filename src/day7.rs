const NEWLINE: u8 = 10;
const SPACE: u8 = 32;
const COLON: u8 = 58;

fn get_val(num: &[u8]) -> usize {
    (0..num.len())
            .into_iter()
            .map(|x| unsafe {
                (*num.get_unchecked(x) as usize - 48usize) * 10usize.pow(((num.len() - 1) - x) as u32)
            })
            .sum()
}

fn calc(comp_value: usize, current: usize, remaining: &[usize], use_concat: bool) -> bool {
    if remaining.is_empty() {
        return comp_value == current;
    }

    let next = remaining[0];
    let mut result = false;

    if next + current <= comp_value {
        result = result || calc(comp_value, next + current, &remaining[1..], use_concat);
    }

    if !result && next * current <= comp_value {
        result = result || calc(comp_value, next * current, &remaining[1..], use_concat);
    }

    if use_concat {
        let concat = current * 10usize.pow(usize::ilog10(next) + 1) + next;

        if !result && concat <= comp_value {
            result = result || calc(comp_value, concat, &remaining[1..], use_concat);
        }
    }
    result
}

fn evaluate_input(text: &[u8], mut pos: usize, end: usize) -> usize {
    let mut sum: usize = 0;
    let mut val_start: usize = 0;
    let mut comp_value: usize = 0;
    let mut val: Vec<usize> = Vec:: with_capacity(16);

    'outer: loop {
        match text[pos] {
            NEWLINE => {
                val.push(get_val(&text[val_start..pos]));
                pos += 1;
                val_start = pos;
                sum += if calc(comp_value, val[0], &val[1..], false) {
                    comp_value
                } else {
                    0
                };
                val = Vec:: with_capacity(16);
            }
            COLON => {
                comp_value = get_val(&text[val_start..pos]);
                pos += 2;
                val_start = pos;
            }
            SPACE => {
                val.push(get_val(&text[val_start..pos]));
                pos += 1;
                val_start = pos
            }
            _ => {
                pos += 1
            }
        }
        if pos == end {
            val.push(get_val(&text[val_start..pos]));
            sum += if calc(comp_value, val[0], &val[1..], false) {
                comp_value
            } else {
                0
            };
            break 'outer;
        }
    }
    sum
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    evaluate_input(input, 0, input.len())
}

fn evaluate_input2(text: &[u8], mut pos: usize, end: usize) -> usize {
    let mut sum: usize = 0;
    let mut val_start: usize = 0;
    let mut comp_value: usize = 0;
    let mut val: Vec<usize> = Vec:: with_capacity(16);

    'outer: loop {
        match text[pos] {
            NEWLINE => {
                val.push(get_val(&text[val_start..pos]));
                pos += 1;
                val_start = pos;
                sum += if calc(comp_value, val[0], &val[1..], true) {
                    comp_value
                } else {
                    0
                };
                val = Vec:: with_capacity(16);
            }
            COLON => {
                comp_value = get_val(&text[val_start..pos]);
                pos += 2;
                val_start = pos;
            }
            SPACE => {
                val.push(get_val(&text[val_start..pos]));
                pos += 1;
                val_start = pos
            }
            _ => {
                pos += 1
            }
        }
        if pos == end {
            val.push(get_val(&text[val_start..pos]));
            sum += if calc(comp_value, val[0], &val[1..], true) {
                comp_value
            } else {
                0
            };
            break 'outer;
        }
    }
    sum
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    evaluate_input2(input, 0, input.len())
}