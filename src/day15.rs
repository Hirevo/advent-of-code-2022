use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

use chumsky::prelude::*;
use num_traits::{PrimInt, Signed};

use crate::interval::{Difference, Interval};
use crate::measured;
use crate::vector::Vector;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day15.txt";

type InputType = Vec<(Vector<i32, 2>, Vector<i32, 2>)>;

fn simplify_intervals<T: PrimInt>(intervals: &[Interval<T>]) -> Vec<Interval<T>> {
    let mut intervals = intervals.to_vec();
    intervals.sort_unstable_by_key(|it| it.lo);

    let mut output = Vec::new();

    let current = intervals.into_iter().reduce(|current, other| {
        if current.overlaps_with(&other) || other.lo - current.hi == T::one() {
            current.union(&other)
        } else {
            output.push(current);
            other.clone()
        }
    });

    if let Some(current) = current {
        output.push(current);
    }

    output
}

fn part1(input: &InputType) -> Result<(), Error> {
    let row_y = 2_000_000;

    let intervals: Vec<Interval<i32>> = input
        .iter()
        .filter_map(|(sensor, beacon)| {
            let reach = sensor.manhattan_distance(*beacon);
            let reach = reach - (row_y - sensor.y()).abs();

            (!reach.is_negative()).then(|| Interval::new(sensor.x() - reach, sensor.x() + reach))
        })
        .collect();

    let simplified = simplify_intervals(&intervals);

    let splitted: Vec<Interval<i32>> = {
        let mut splitted = Vec::with_capacity(simplified.len());

        for interval in simplified {
            for (_, beacon) in input {
                if beacon.y() != row_y {
                    continue;
                }

                match interval.split_at(beacon.x()) {
                    Difference::Two(left, right) => {
                        splitted.push(left);
                        splitted.push(right);
                    }
                    Difference::One(it) => {
                        splitted.push(it);
                    }
                    Difference::None => {}
                }
            }
        }

        splitted
    };

    let simplified = simplify_intervals(&splitted);

    let answer: i32 = simplified.into_iter().map(|it| it.length() + 1).sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let size = 4_000_000;

    let answer = (0..size)
        .map(|row| {
            let intervals: Vec<Interval<i32>> = input
                .iter()
                .filter_map(|(sensor, beacon)| {
                    let reach = sensor.manhattan_distance(*beacon);
                    let reach = reach - (row - sensor.y()).abs();

                    (!reach.is_negative())
                        .then(|| Interval::new(sensor.x() - reach, sensor.x() + reach))
                })
                .collect();

            simplify_intervals(&intervals)
        })
        .enumerate()
        .find(|(_, it)| it.len() == 2)
        .map(|(y, it)| y as u64 + (it[0].hi + 1) as u64 * 4_000_000)
        .expect("found no answer");

    println!("part2: {answer}");
    Ok(())
}

fn integer<O>() -> impl Parser<char, O, Error = Simple<char>>
where
    O: PrimInt + Signed + FromStr,
    O::Err: Debug,
{
    just('-')
        .or_not()
        .then(text::int(10).from_str::<O>().unwrapped())
        .map(|(minus, int)| minus.map_or(int, |_| -int))
}

fn parser() -> impl Parser<char, (Vector<i32, 2>, Vector<i32, 2>), Error = Simple<char>> {
    just("Sensor at x=")
        .ignore_then(integer())
        .then_ignore(just(", y="))
        .then(integer())
        .map(|(x, y)| Vector::new([x, y]))
        .then(
            just(": closest beacon is at x=")
                .ignore_then(integer())
                .then_ignore(just(", y="))
                .then(integer())
                .map(|(x, y)| Vector::new([x, y])),
        )
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Result<InputType, _> = input
        .lines()
        .map(|section| parser().parse(section))
        .collect();

    let input = match input {
        Ok(input) => input,
        Err(err) => {
            eprintln!("{err:?}");
            std::process::exit(1);
        }
    };

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
