use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::vector::Vector;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day14.txt";

type InputType = HashSet<Vector<i32, 2>>;

#[rustfmt::skip]
static TO_CONSIDER: [Vector<i32, 2>; 3] = [
    Vector::new([ 0, 1]),
    Vector::new([-1, 1]),
    Vector::new([ 1, 1]),
];

fn place_sand(
    occupied: &mut InputType,
    max: Vector<i32, 2>,
    current: Vector<i32, 2>,
) -> Option<Vector<i32, 2>> {
    if current.y() > max.y() {
        return None;
    }

    for dir in TO_CONSIDER {
        let dir = current + dir;
        if !occupied.contains(&dir) {
            return place_sand(occupied, max, dir);
        }
    }

    Some(current)
}

fn part1(input: &InputType, _: Vector<i32, 2>, max: Vector<i32, 2>) -> Result<(), Error> {
    let origin = Vector::new([500, 0]);
    let mut occupied = input.clone();

    while let Some(coords) = place_sand(&mut occupied, max, origin) {
        occupied.insert(coords);
    }

    let answer = occupied.len() - input.len();

    println!("part1: {answer}");
    Ok(())
}

fn place_sand_floored(
    occupied: &mut InputType,
    max: Vector<i32, 2>,
    current: Vector<i32, 2>,
) -> Vector<i32, 2> {
    for dir in TO_CONSIDER {
        let dir = current + dir;
        if !occupied.contains(&dir) && dir.y() < max.y() + 2 {
            return place_sand_floored(occupied, max, dir);
        }
    }

    current
}

fn part2(input: &InputType, _min: Vector<i32, 2>, max: Vector<i32, 2>) -> Result<(), Error> {
    let origin = Vector::new([500, 0]);
    let mut occupied = input.clone();

    while !occupied.contains(&origin) {
        let coords = place_sand_floored(&mut occupied, max, origin);
        occupied.insert(coords);
    }

    let answer = occupied.len() - input.len();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input = {
        let mut occupied: InputType = HashSet::new();

        for line in input.lines() {
            let values: Vec<_> = line
                .split(" -> ")
                .map(|it| {
                    let (x, y) = it.split_once(',').expect("invalid input");
                    let x: i32 = x.parse().expect("invalid input");
                    let y: i32 = y.parse().expect("invalid input");
                    Vector::new([x, y])
                })
                .collect();

            for it in values.windows(2) {
                let mut start = it[0];
                let end = it[1];

                let mut diff = end - start;
                *diff.x_mut() = diff.x().clamp(-1, 1);
                *diff.y_mut() = diff.y().clamp(-1, 1);

                occupied.insert(start);

                while start != end {
                    start += diff;
                    occupied.insert(start);
                }
            }
        }

        occupied
    };

    let min = input
        .iter()
        .copied()
        .reduce(|acc, it| Vector::new([acc.x().min(it.x()), acc.y().min(it.y())]))
        .expect("empty input");

    let max = input
        .iter()
        .copied()
        .reduce(|acc, it| Vector::new([acc.x().max(it.x()), acc.y().max(it.y())]))
        .expect("empty input");

    measured!(part1(&input, min, max))?;
    measured!(part2(&input, min, max))?;

    Ok(())
}
