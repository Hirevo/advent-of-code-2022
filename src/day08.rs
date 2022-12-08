use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day08.txt";

type InputType = Vec<Vec<i32>>; // we choose a signed integer type (i32) because we'll compare it to a `-1` later.

fn part1(input: &InputType, width: usize, height: usize) -> Result<(), Error> {
    let mut set = HashSet::new();

    for y in 0..height {
        let mut max = -1;
        for x in 0..width {
            let value = input[y][x];
            if value > max {
                set.insert((x, y));
                max = value;
            }
        }

        max = -1;
        for x in (0..width).rev() {
            let value = input[y][x];
            if value > max {
                set.insert((x, y));
                max = value;
            }
        }
    }

    for x in 0..width {
        let mut max = -1;
        for y in 0..height {
            let value = input[y][x];
            if value > max {
                set.insert((x, y));
                max = value;
            }
        }

        max = -1;
        for y in (0..height).rev() {
            let value = input[y][x];
            if value > max {
                set.insert((x, y));
                max = value;
            }
        }
    }

    let answer = set.len();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType, width: usize, height: usize) -> Result<(), Error> {
    let mut answer = 0;

    for y in 0..height {
        for x in 0..width {
            let value = input[y][x];

            let mut one_more; // to account for the tree that stopped our view.

            one_more = true;
            let north = (0..y)
                .rev()
                .take_while(|&y| {
                    (one_more && input[y][x] < value) || std::mem::replace(&mut one_more, false)
                })
                .count();

            one_more = true;
            let east = ((x + 1)..width)
                .take_while(|&x| {
                    (one_more && input[y][x] < value) || std::mem::replace(&mut one_more, false)
                })
                .count();

            one_more = true;
            let south = ((y + 1)..height)
                .take_while(|&y| {
                    (one_more && input[y][x] < value) || std::mem::replace(&mut one_more, false)
                })
                .count();

            one_more = true;
            let west = (0..x)
                .rev()
                .take_while(|&x| {
                    (one_more && input[y][x] < value) || std::mem::replace(&mut one_more, false)
                })
                .count();

            answer = answer.max(north * east * south * west);
        }
    }

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .map(|line| line.bytes().map(|ch| (ch - b'0') as i32).collect())
        .collect();

    let width = input.first().expect("invalid input").len();
    let height = input.len();

    measured!(part1(&input, width, height))?;
    measured!(part2(&input, width, height))?;

    Ok(())
}
