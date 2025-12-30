use std::collections::HashSet;
use std::ops::ControlFlow;

enum Direction {
    North,
    East,
    South,
    West,
}

fn get_step(step: &[u8]) -> (u8, i16) {
    if step[0] == b' ' {
        (
            step[1],
            step[2..]
                .iter()
                .fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32) as i16,
        )
    } else {
        (
            step[0],
            step[1..]
                .iter()
                .fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32) as i16,
        )
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i16 {
    let input = input.as_bytes();
    let (final_x, final_y, _) = input.split(|&b| b == b',').fold(
        (0_i16, 0_i16, Direction::North),
        |(mut x, mut y, dir), step| {
            let (turn, dist) = get_step(step);

            let next_dir = match (&dir, turn) {
                (Direction::North, b'L') => Direction::West,
                (Direction::North, b'R') => Direction::East,
                (Direction::East, b'L') => Direction::North,
                (Direction::East, b'R') => Direction::South,
                (Direction::South, b'L') => Direction::East,
                (Direction::South, b'R') => Direction::West,
                (Direction::West, b'L') => Direction::South,
                (Direction::West, b'R') => Direction::North,
                _ => dir,
            };

            match next_dir {
                Direction::North => y += dist,
                Direction::East => x += dist,
                Direction::South => y -= dist,
                Direction::West => x -= dist,
            }

            (x, y, next_dir)
        },
    );

    final_x.abs() + final_y.abs()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i16 {
    let input = input.as_bytes();
    let mut visited: HashSet<(i16, i16)> = HashSet::with_capacity(152);
    visited.insert((0, 0));

    let result = input.split(|&b| b == b',').try_fold(
        (0_i16, 0_i16, Direction::North),
        |(mut x, mut y, dir), step| {
            let (turn, dist) = get_step(step);

            let next_dir = match (&dir, turn) {
                (Direction::North, b'L') => Direction::West,
                (Direction::North, b'R') => Direction::East,
                (Direction::East, b'L') => Direction::North,
                (Direction::East, b'R') => Direction::South,
                (Direction::South, b'L') => Direction::East,
                (Direction::South, b'R') => Direction::West,
                (Direction::West, b'L') => Direction::South,
                (Direction::West, b'R') => Direction::North,
                _ => dir,
            };

            // CRITICAL: Move 1 unit at a time to check for intersections
            for _ in 0..dist {
                match next_dir {
                    Direction::North => y += 1,
                    Direction::East => x += 1,
                    Direction::South => y -= 1,
                    Direction::West => x -= 1,
                }

                if !visited.insert((x, y)) {
                    return ControlFlow::Break(x.abs() + y.abs());
                }
            }

            ControlFlow::Continue((x, y, next_dir))
        },
    );

    match result {
        ControlFlow::Break(dist) => dist,
        ControlFlow::Continue(_) => 0,
    }
}
