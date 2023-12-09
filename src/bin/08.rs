use std::collections::HashMap;

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
    let starts = map
        .iter()
        .filter(|(&k, _)| k.ends_with("A"))
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();

    let mut steps = vec![];

    for start in starts {
        let mut step = 0;
        let mut k = start;

        for instr in instructions.clone() {
            step += 1;

            let next = &map[k];

            let next_key = match instr {
                'R' => next.right,
                'L' => next.left,
                _ => unreachable!(),
            };

            if next_key.ends_with('Z') {
                break;
            }

            k = next_key;
        }

        steps.push(step)
    }

    Some(lcm(&steps))
}

pub fn lcm(numbers: &[u32]) -> u32 {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let first = numbers[0];
    let rest = lcm(&numbers[1..]);

    first * rest / gcd(first, rest)
}

fn gcd(first: u32, second: u32) -> u32 {
    if second == 0 {
        return first;
    }

    gcd(second, first % second)
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
