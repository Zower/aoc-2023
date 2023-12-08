use std::{
    collections::HashMap,
    io::Write,
    time::{Duration, Instant},
};

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

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let _ = lines.next().unwrap();

    let map = map(lines);

    let mut steps = 0;

    let mut starts = map
        .iter()
        .filter(|(&k, _)| k.ends_with("A"))
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();

    let mut io = std::io::stdout().lock();

    let mut now = Instant::now();

    for (i, instr) in instructions.enumerate() {
        let mut all_valid = true;

        for start_key in &mut starts {
            let start_value = &map[start_key];

            let path_to = match instr {
                'R' => start_value.right,
                'L' => start_value.left,
                _ => unreachable!(),
            };

            if &path_to[2..3] != "Z" {
                all_valid = false;
            }

            *start_key = path_to;
        }

        steps += 1;

        if all_valid {
            break;
        }

        if i % 10000 == 0 && now.elapsed() > Duration::from_secs(5) {
            let _ = io.write_fmt(format_args!("Checking for {starts:?}, {steps}\n"));

            now = Instant::now();
        }
    }

    Some(steps)
}

fn map<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Path<'a>> {
    let mut map = HashMap::new();

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
