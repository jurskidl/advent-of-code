fn parse_num(num: &[u8]) -> u32 {
    return if num.len() < 2 {
        (num[0] - b'0') as u32
    } else {
        10 * num[0] as u32 + num[1] as u32 - 11 * b'0' as u32 
    }
}

fn parse_line(line: &[u8]) -> u32 {
    let mut num_end = 0;
    
    let num_start = num_end;

    while line[num_end] != b'x' && num_end < line.len() {
        num_end += 1
    }

    let l = parse_num(&line[num_start..num_end]);

    num_end += 1;

    let num_start = num_end;

    while line[num_end] != b'x' && num_end < line.len() {
        num_end += 1
    }

    let w = parse_num(&line[num_start..num_end]);    

    num_end += 1;

    let num_start = num_end;

    let h = parse_num(&line[num_start..line.len()]);
    
    let lw = l*w;
    let lh = l*h;
    let wh = w*h;

    ((lw + lh + wh) << 1) + lw.min(lh).min(wh)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut end_line = 0;
    let mut sqft = 0_u32;

    let bytes = input.as_bytes();

    while end_line < bytes.len() {
        let start_line = end_line;
        while end_line < bytes.len() && bytes[end_line] != b'\n'  {
            end_line += 1;
        }

        sqft += parse_line(&bytes[start_line..end_line]);
        end_line += 1;
    }

    sqft
}

fn parse_line2(line: &[u8]) -> u32 {
    let mut num_end = 0;
    
    let num_start = num_end;

    while line[num_end] != b'x' && num_end < line.len() {
        num_end += 1
    }

    let l = parse_num(&line[num_start..num_end]);

    num_end += 1;

    let num_start = num_end;

    while line[num_end] != b'x' && num_end < line.len() {
        num_end += 1
    }

    let w = parse_num(&line[num_start..num_end]);    

    num_end += 1;

    let num_start = num_end;

    let h = parse_num(&line[num_start..line.len()]);
    
    l * w * h + 2 * (l + w + h) - 2 * l.max(w).max(h)
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut end_line = 0;
    let mut ft = 0_u32;

    let bytes = input.as_bytes();

    while end_line < bytes.len() {
        let start_line = end_line;
        while end_line < bytes.len() && bytes[end_line] != b'\n'  {
            end_line += 1;
        }

        ft += parse_line2(&bytes[start_line..end_line]);
        end_line += 1;
    }

    ft
}
