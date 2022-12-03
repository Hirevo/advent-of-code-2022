use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/dayXX.txt";

type InputType = String;

fn part1(_input: &InputType) -> Result<(), Error> {
    todo!()
}

fn part2(_input: &InputType) -> Result<(), Error> {
    todo!()
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
