use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day06.txt";

type InputType = Vec<u8>;

fn part1(input: &InputType) -> Result<(), Error> {
    let answer = input
        .windows(4)
        .map(|window| window.into_iter().collect::<HashSet<_>>())
        .position(|set| set.len() == 4)
        .map(|idx| idx + 4)
        .expect("answer not found");

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer = input
        .windows(14)
        .map(|window| window.into_iter().collect::<HashSet<_>>())
        .position(|set| set.len() == 14)
        .map(|idx| idx + 14)
        .expect("answer not found");

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input.trim().bytes().collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
