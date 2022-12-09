use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::vector::Vector;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day09.txt";

type InputType = Vec<(Direction, usize)>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// We make use of const-generics to parameterize the rope size.
// This allows the compiler to potentially statically optimize
// for each of the rope sizes we use it with.
fn solve<const N: usize>(input: &InputType) -> usize {
    let mut set = HashSet::new();

    let mut rope = [Vector::<i32, 2>::default(); N];

    for (dir, amount) in input.iter().copied() {
        #[rustfmt::skip]
        let dir: Vector<i32, 2> = match dir {
            Direction::Up    => [ 0, -1].into(),
            Direction::Down  => [ 0,  1].into(),
            Direction::Left  => [-1,  0].into(),
            Direction::Right => [ 1,  0].into(),
        };

        for _ in 0..amount {
            rope[0] += dir;

            for idx in 1..rope.len() {
                let head = rope[idx - 1];
                let tail = &mut rope[idx];

                let diff = head - *tail;

                if diff.x().abs() > 1 || diff.y().abs() > 1 {
                    let mut off = Vector::<i32, 2>::default();

                    if diff.x().abs() > 1 {
                        *off.x_mut() = diff.x() / diff.x().abs();
                    }

                    if diff.y().abs() > 1 {
                        *off.y_mut() = diff.y() / diff.y().abs();
                    }

                    *tail = *tail + diff - off;
                }
            }

            set.insert(rope.last().copied().unwrap());
        }
    }

    set.len()
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer = solve::<2>(input);
    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer = solve::<10>(input);
    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input = input
        .lines()
        .map(|line| {
            let (cmd, amount) = line.split_once(' ').expect("invalid input");
            let amount = amount.parse().expect("invalid input");

            match cmd {
                "U" => (Direction::Up, amount),
                "D" => (Direction::Down, amount),
                "L" => (Direction::Left, amount),
                "R" => (Direction::Right, amount),
                _ => {
                    panic!("invalid input");
                }
            }
        })
        .collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
