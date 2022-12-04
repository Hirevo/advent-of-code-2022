use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day04.txt";

type InputType = Vec<((u32, u32), (u32, u32))>;

fn part1(input: &InputType) -> Result<(), Error> {
    let answer = input
        .iter()
        .copied()
        .filter(|&(lhs, rhs)| {
            let intersection = (lhs.0.max(rhs.0), lhs.1.min(rhs.1));
            intersection == lhs || intersection == rhs
        })
        .count();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer = input
        .iter()
        .copied()
        .filter(|&(lhs, rhs)| lhs.1.min(rhs.1) >= lhs.0.max(rhs.0))
        .count();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .flat_map(|line| {
            let (lhs, rhs) = line.split_once(',')?;

            let lhs = lhs
                .split_once('-')
                .and_then(|(lhs, rhs)| lhs.parse().ok().zip(rhs.parse().ok()));
            let rhs = rhs
                .split_once('-')
                .and_then(|(lhs, rhs)| lhs.parse().ok().zip(rhs.parse().ok()));

            lhs.zip(rhs)
        })
        .collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
