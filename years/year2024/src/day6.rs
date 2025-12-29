const POUND: u8 = 35;
const CARAT: u8 = 94;
const NEWLINE: u8 = 10;
const SIZE: usize = 130;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn process_map(
    input: &[u8],
    end: usize,
    mut line: usize,
    mut pos: usize,
) -> ([[bool; SIZE]; SIZE], [usize; 2]) {
    let mut obstacles: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut guard: [usize; 2] = [0; 2];

    while ((line * (SIZE + 1)) + pos) < end {
        match input[(line * (SIZE + 1)) + pos] {
            POUND => obstacles[line][pos] = true,
            CARAT => guard = [line, pos],
            NEWLINE => {
                pos = 0;
                line += 1
            }
            _ => {}
        }
        pos += 1
    }

    (obstacles, guard)
}

fn get_pathing(obstacles: &[[bool; SIZE]; SIZE], mut guard: [usize; 2]) -> usize {
    let mut pathing: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut direction = Direction::Up;

    while guard[0] > 0 && guard[0] < SIZE && guard[1] > 0 && guard[1] < SIZE {
        match direction {
            Direction::Up => {
                while guard[0] > 0 && !obstacles[guard[0] - 1][guard[1]] {
                    pathing[guard[0]][guard[1]] = true;
                    guard[0] -= 1;
                }
                direction = Direction::Right;
            }
            Direction::Right => {
                while guard[1] < SIZE && !obstacles[guard[0]][guard[1] + 1] {
                    pathing[guard[0]][guard[1]] = true;
                    guard[1] += 1;
                }
                direction = Direction::Down;
            }
            Direction::Down => {
                while guard[0] < SIZE && !obstacles[guard[0] + 1][guard[1]] {
                    pathing[guard[0]][guard[1]] = true;
                    guard[0] += 1;
                }
                direction = Direction::Left;
            }
            _ => {
                while guard[0] < SIZE && !obstacles[guard[0]][guard[1] - 1] {
                    pathing[guard[0]][guard[1]] = true;
                    guard[1] -= 1;
                }
                direction = Direction::Up;
            }
        }
    }

    // add one since we have the space we are currently standing on
    pathing.into_iter().flatten().filter(|x| *x == true).count() + 1
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let (obstacles, guard) = process_map(input, input.len(), 0, 0);
    get_pathing(&obstacles, guard)
}
