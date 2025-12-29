const APOST: u8 = 39;
const LBRKT: u8 = 40;
const RBRKT: u8 = 41;
const COMMA: u8 = 44;
const L: u8 = 108;
const M: u8 = 109;
const D: u8 = 100;
const O: u8 = 111;
const N: u8 = 110;
const T: u8 = 116;
const U: u8 = 117;

fn get_num(text: &[u8], mut pos: usize, end: usize) -> (u16, u16) {
    let val1_start = pos;
    while pos < end && (vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]).contains(&text[pos]) {
        pos += 1;
    }

    if text[pos] == COMMA {
        true
    } else {
        return (0, 0);
    };

    let val2_start = pos + 1;
    let mut temp_pos = val2_start + 1;

    while temp_pos < end && (vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]).contains(&text[temp_pos])
    {
        temp_pos += 1;
    }

    if text[temp_pos] == RBRKT {
        true
    } else {
        return (0, 0);
    };

    (
        (val1_start..pos)
            .into_iter()
            .map(|x| unsafe {
                ((*text.get_unchecked(x) as u32 - 48u32) * 10u32.pow(((pos - 1) - x) as u32)) as u16
            })
            .sum(),
        (val2_start..temp_pos)
            .into_iter()
            .map(|x| unsafe {
                ((*text.get_unchecked(x) as u32 - 48u32) * 10u32.pow(((temp_pos - 1) - x) as u32))
                    as u16
            })
            .sum(),
    )
}

fn check_mem(memory: &[u8], pos: usize, end: usize) -> u32 {
    match (
        memory[pos - 3],
        memory[pos - 2],
        memory[pos - 1],
        memory[pos],
    ) {
        (M, U, L, LBRKT) => {
            let (val1, val2) = get_num(memory, pos + 1, end);
            val1 as u32 * val2 as u32
        }
        _ => 0,
    }
}

fn scan_mem(input: &[u8], end: usize) -> u32 {
    let mut sum = 0;
    let mut pos = 4;
    while pos < end {
        sum += check_mem(input, pos, end);
        pos += 1;
    }
    sum
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    scan_mem(input, input.len())
}

fn get_num2(text: &[u8], mut pos: usize, end: usize) -> (u16, u16) {
    let val1_start = pos;
    while pos < end && (vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]).contains(&text[pos]) {
        pos += 1;
    }

    if text[pos] == COMMA {
        true
    } else {
        return (0, 0);
    };

    let val2_start = pos + 1;
    let mut temp_pos = val2_start + 1;

    while temp_pos < end && (vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]).contains(&text[temp_pos])
    {
        temp_pos += 1;
    }

    if text[temp_pos] == RBRKT {
        true
    } else {
        return (0, 0);
    };

    (
        (val1_start..pos)
            .into_iter()
            .map(|x| unsafe {
                ((*text.get_unchecked(x) as u32 - 48u32) * 10u32.pow(((pos - 1) - x) as u32)) as u16
            })
            .sum(),
        (val2_start..temp_pos)
            .into_iter()
            .map(|x| unsafe {
                ((*text.get_unchecked(x) as u32 - 48u32) * 10u32.pow(((temp_pos - 1) - x) as u32))
                    as u16
            })
            .sum(),
    )
}

fn check_mem2(memory: &[u8], pos: usize, end: usize, sum: u32, do_flag: bool) -> (u32, bool) {
    match (
        memory[pos - 3],
        memory[pos - 2],
        memory[pos - 1],
        memory[pos],
    ) {
        (M, U, L, LBRKT) => {
            let (val1, val2) = get_num2(memory, pos + 1, end);
            (sum + val1 as u32 * val2 as u32, do_flag)
        }
        (D, O, N, APOST) => {
            if pos + 3 < end
                && (memory[pos + 1] == T && memory[pos + 2] == LBRKT && memory[pos + 3] == RBRKT)
            {
                (sum, false)
            } else {
                return (sum, true);
            }
        }
        _ => (sum, do_flag),
    }
}

fn do_check2(memory: &[u8]) -> bool {
    match (memory[0], memory[1], memory[2], memory[3]) {
        (D, O, LBRKT, RBRKT) => true,
        _ => false,
    }
}

fn scan_mem2(input: &[u8], end: usize) -> u32 {
    let mut sum: u32 = 0;
    let mut pos: usize = 4;
    let mut do_flag: bool = true;

    while pos < end {
        while pos < end && do_flag {
            (sum, do_flag) = check_mem2(input, pos, end, sum, do_flag);
            pos += 1;
        }
        while pos < end && !do_flag {
            do_flag = do_check2(&input[pos - 4..pos]);
            pos += 1;
        }
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    scan_mem2(input, input.len())
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use crate::day2::{part1, part2};

    #[test]
    fn test_day2() {
        let test: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(part1(test), 161);
        assert_eq!(part2(test), 48)
    }
}
