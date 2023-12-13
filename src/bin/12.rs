advent_of_code::solution!(12);

use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Tile {
    Damaged,
    Operational,
    Unknown,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let input = split.next().unwrap();

        let other = split
            .next()
            .unwrap()
            .split(',')
            .map(|it| it.parse::<u32>().unwrap())
            .collect_vec();

        let tiles = input
            .chars()
            .map(|char| match char {
                '#' => Tile::Damaged,
                '.' => Tile::Operational,
                '?' => Tile::Unknown,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        // println!("{tiles:?}");

        let q_count = tiles.iter().filter(|it| **it == Tile::Unknown).count();

        for perm in repeat_n(vec![Tile::Damaged, Tile::Operational].into_iter(), q_count)
            .multi_cartesian_product()
        // .combinations_with_replacement(q_count)
        {
            let mut idx = 0;
            let mut replacement = vec![];

            for tile in &tiles {
                if *tile == Tile::Unknown {
                    replacement.push(perm[idx]);
                    idx += 1;
                } else {
                    replacement.push(*tile);
                }
            }

            let is_valid = {
                let replacement = replacement
                    .into_iter()
                    .dedup_by(|a, b| *a == Tile::Operational && *b == Tile::Operational)
                    .collect_vec();

                let y = replacement
                    .split(|x| *x == Tile::Operational)
                    .map(|it| it.len() as u32)
                    .filter(|it| *it != 0);

                y.eq(other.iter().cloned())
            };

            if is_valid {
                sum += 1
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
