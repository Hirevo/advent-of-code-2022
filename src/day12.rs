use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day12.txt";

type InputType = Vec<i32>;
type CoordType = (usize, usize);

fn part1(
    input: &InputType,
    size: CoordType,
    start: CoordType,
    end: CoordType,
) -> Result<(), Error> {
    let successors = |&(x, y): &CoordType| {
        let current = input[y * size.0 + x];

        // TODO: this should definitely be a `SmallVec<T>`, to avoid allocations
        let mut successors = Vec::with_capacity(4);

        if x > 0 && input[y * size.0 + x - 1] - current < 2 {
            successors.push((x - 1, y));
        }
        if y > 0 && input[(y - 1) * size.0 + x] - current < 2 {
            successors.push((x, y - 1));
        }
        if x < (size.0 - 1) && input[y * size.0 + x + 1] - current < 2 {
            successors.push((x + 1, y));
        }
        if y < (size.1 - 1) && input[(y + 1) * size.0 + x] - current < 2 {
            successors.push((x, y + 1));
        }

        successors
    };

    let success = |coord: &CoordType| *coord == end;

    let found = pathfinding::directed::bfs::bfs(&start, successors, success);

    let answer = found.map_or(0, |path| path.len() - 1);

    println!("part1: {answer}");
    Ok(())
}

fn part2(
    input: &InputType,
    size: CoordType,
    _start: CoordType,
    end: CoordType,
) -> Result<(), Error> {
    let successors = |&(x, y): &CoordType| {
        let current = input[y * size.0 + x];

        // TODO: this should definitely be a `SmallVec<T>`, to avoid allocations
        let mut successors = Vec::with_capacity(4);

        if x > 0 && current - input[y * size.0 + x - 1] < 2 {
            successors.push((x - 1, y));
        }
        if y > 0 && current - input[(y - 1) * size.0 + x] < 2 {
            successors.push((x, y - 1));
        }
        if x < (size.0 - 1) && current - input[y * size.0 + x + 1] < 2 {
            successors.push((x + 1, y));
        }
        if y < (size.1 - 1) && current - input[(y + 1) * size.0 + x] < 2 {
            successors.push((x, y + 1));
        }

        successors
    };

    let success = |coord: &CoordType| input[coord.1 * size.0 + coord.0] == 0;

    let found = pathfinding::directed::bfs::bfs(&end, successors, success);

    let answer = found.map_or(0, |path| path.len() - 1);

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let height = input.lines().count();

    let (input, start, end) = {
        let mut start = None::<CoordType>;
        let mut end = None::<CoordType>;

        let mut values: InputType = Vec::with_capacity(height);

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.bytes().enumerate() {
                match ch {
                    b'S' => {
                        start = Some((x, y));
                        values.push(0);
                    }
                    b'E' => {
                        end = Some((x, y));
                        values.push((b'z' - b'a') as i32);
                    }
                    ch if (b'a'..=b'z').contains(&ch) => {
                        values.push((ch - b'a') as i32);
                    }
                    ch => {
                        panic!(
                            "found unexpected `{ch}`, expected an alphabetic lowercase character"
                        )
                    }
                }
            }
        }

        let start = start.expect("could not find start node");
        let end = end.expect("could not find end node");

        (values, start, end)
    };

    let width = input.len() / height;

    let size = (width, height);

    measured!(part1(&input, size, start, end))?;
    measured!(part2(&input, size, start, end))?;

    Ok(())
}
