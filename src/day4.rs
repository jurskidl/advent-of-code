const X: u8 = 88;
const M: u8 = 77;
const A: u8 = 65;
const S: u8 = 83;
const SIZE: usize = 141;

fn inline_left(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[pos+(line*SIZE)-1],input[pos+(line*SIZE)-2],input[pos+(line*SIZE)-3]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn inline_right(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[pos+(line*SIZE)+1],input[pos+(line*SIZE)+2],input[pos+(line*SIZE)+3]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn up(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[pos+((line-1)*SIZE)],input[pos+((line-2)*SIZE)],input[pos+((line-3)*SIZE)]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn down(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[pos+((line+1)*SIZE)],input[pos+((line+2)*SIZE)],input[pos+((line+3)*SIZE)]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn up_right(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos+1)+((line-1)*SIZE)],input[(pos+2)+((line-2)*SIZE)],input[(pos+3)+((line-3)*SIZE)]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn up_left(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos-1)+((line-1)*SIZE)],input[(pos-2)+((line-2)*SIZE)],input[(pos-3)+((line-3)*SIZE)]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn down_right(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos+1)+((line+1)*SIZE)],input[(pos+2)+((line+2)*SIZE)],input[(pos+3)+((line+3)*SIZE)]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn down_left(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos-1)+((line+1)*SIZE)],input[(pos-2)+((line+2)*SIZE)],input[(pos-3)+((line+3)*SIZE)]) {
        (M,A,S) => { 1 }
        _ => { 0 }
    }
}

fn search_line (input: &[u8], line: usize, mut pos: usize, mut matches: u16) -> u16 {
    while pos < SIZE && pos+(line*SIZE) < SIZE.pow(2)-(SIZE+1) {
        if input[pos+(line*SIZE)] != X {} else{
            match (line, pos) {
                (x, y) if x < 3 && y < 3 => {
                    matches += inline_right(input, line, pos);
                    matches += down(input, line, pos);
                    matches += down_right(input, line, pos)
                }
                (x, y) if x > (SIZE-5) && y > (SIZE-5) => {
                    matches += inline_left(input, line, pos);
                    matches += up(input, line, pos);
                    matches += up_left(input, line, pos);
                }
                (x, y) if x < 3 && y > (SIZE-5) => {
                    matches += inline_left(input, line, pos);
                    matches += down(input, line, pos);
                    matches += down_left(input, line, pos)
                }
                (x, y) if x > (SIZE-5) && y < 3 => {
                    matches += inline_right(input, line, pos);
                    matches += up(input, line, pos);
                    matches += up_right(input, line, pos)
                }
                (x, _) if x < 3 => {
                    matches += inline_left(input, line, pos);
                    matches += inline_right(input, line, pos);
                    matches += down(input, line, pos);
                    matches += down_left(input, line, pos);
                    matches += down_right(input, line, pos)
                }
                (_, y) if y < 3 => {
                    matches += inline_right(input, line, pos);
                    matches += up(input, line, pos);
                    matches += down(input, line, pos);
                    matches += up_right(input, line, pos);
                    matches += down_right(input, line, pos)
                }
                (x, _) if x > (SIZE-5) => {
                    matches += inline_left(input, line, pos);
                    matches += inline_right(input, line, pos);
                    matches += up(input, line, pos);
                    matches += up_left(input, line, pos);
                    matches += up_right(input, line, pos)
                }
                (_, y) if y > (SIZE-5) => {
                    matches += inline_left(input, line, pos);
                    matches += up(input, line, pos);
                    matches += down(input, line, pos);
                    matches += up_left(input, line, pos);
                    matches += down_left(input, line, pos)
                }
                _ => {
                    matches += inline_left(input, line, pos);
                    matches += inline_right(input, line, pos);
                    matches += up(input, line, pos);
                    matches += down(input, line, pos);
                    matches += up_left(input, line, pos);
                    matches += up_right(input, line, pos);
                    matches += down_left(input, line, pos);
                    matches += down_right(input, line, pos)
                }
            }
        }
        pos+=1
    }
    matches
}

fn search_input(input: &[u8], mut line: usize, mut matches: u16) -> u16 {
    while line < SIZE-1 {
        matches += search_line(input, line, 0, 0);
        line += 1;
    }
    matches
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u16 {
    let input = input.as_bytes();
    search_input(input, 0, 0)
}

fn x_mm(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos+1)+((line+1)*SIZE)],input[(pos+2)+((line+2)*SIZE)],input[pos+((line+2)*SIZE)]) {
        (A,S,S) => {1}
        _ => {0}
    }
}

fn x_ms(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos+1)+((line+1)*SIZE)],input[(pos+2)+((line+2)*SIZE)],input[pos+((line+2)*SIZE)]) {
        (A,S,M) => {1}
        _ => {0}
    }
}

fn x_sm(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos+1)+((line+1)*SIZE)],input[(pos+2)+((line+2)*SIZE)],input[pos+((line+2)*SIZE)]) {
        (A,M,S) => {1}
        _ => {0}
    }
}

fn x_ss(input: &[u8], line: usize, pos: usize) -> u16 {
    match (input[(pos+1)+((line+1)*SIZE)],input[(pos+2)+((line+2)*SIZE)],input[pos+((line+2)*SIZE)]) {
        (A,M,M) => {1}
        _ => {0}
    }
}

fn search_line2(input: &[u8], line: usize, mut pos: usize, mut matches: u16) -> u16 {
    while pos < SIZE-2 && pos+(line*SIZE) < SIZE.pow(2)-(SIZE+3) {
        matches += match (input[pos+(line*SIZE)],input[(pos+(line*SIZE))+2]) {
            (M,M) => { x_mm(input, line, pos) }
            (M,S) => { x_ms(input, line, pos) }
            (S,M) => { x_sm(input, line, pos) }
            (S,S) => { x_ss(input, line, pos) }
            _ => {0}
        };
        pos+=1
    }
    matches
}

fn search_input2(input: &[u8], mut line: usize, mut matches: u16) -> u16 {
    while line < SIZE-3 {
        matches += search_line2(input, line, 0, 0);
        line += 1;
    }
    matches
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u16 {
    let input = input.as_bytes();
    search_input2(input, 0, 0)
}