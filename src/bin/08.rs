use std::{
    io::Write,
    time::{Duration, Instant},
};

use ahash::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Path<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let _ = lines.next().unwrap();

    let map = map(lines);

    let mut steps = 0;
    let mut k = "AAA";

    for instr in instructions {
        let v = &map[k];

        if k == "ZZZ" {
            break;
        }

        k = match instr {
            'R' => v.right,
            'L' => v.left,
            _ => unreachable!(),
        };

        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let _ = lines.next().unwrap();

    let map: std::collections::HashMap<&str, Path<'_>, ahash::RandomState> = map(lines);

    let mut steps: u128 = 0;

    let mut starts = map
        .iter()
        .filter(|(&k, _)| k.ends_with("A"))
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();

    for instr in instructions {
        steps += 1;

        let mut mapped = starts.iter_mut().map(|key| {
            let value = &map[key];

            let path_to = if instr == 'R' {
                value.right
            } else {
                value.left
            };

            *key = path_to;

            unsafe { path_to.get_unchecked(2..3) == "Z" }
        });

        if mapped.all(|t| t) {
            break;
        }

        for _ in mapped {}
    }

    Some(steps)
}

fn map<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Path<'a>> {
    let mut map = HashMap::with_hasher(ahash::RandomState::new());

    for line in lines {
        let mut split = line.split_whitespace();

        let k = split.next().unwrap();

        let _ = split.next();

        let left = &split.next().unwrap()[1..4];
        let right = &split.next().unwrap()[0..3];

        map.insert(k, Path { left, right });
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));

        assert_eq!(result, Some(6));
    }
}
