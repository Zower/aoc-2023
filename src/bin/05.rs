use std::{error::Error, ops::Range};

use rangemap::RangeMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    fn do_it(input: &str) -> Result<u64, Box<dyn Error>> {
        let mut seeds: Vec<u64> = vec![];
        let mut maps: Vec<RangeMap<u64, u64>> = vec![];

        for line in input.lines() {
            if line.contains("seeds") {
                seeds.extend(
                    line.replace("seeds:", "")
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap()),
                );

                continue;
            }

            if line == "" {
                continue;
            }

            if line.contains("map") {
                maps.push(RangeMap::new());

                continue;
            }

            let mut tokens = line.split_whitespace();

            let destination_range_start: u64 = tokens.next().unwrap().parse()?;
            let source_range_start: u64 = tokens.next().unwrap().parse()?;
            let length: u64 = tokens.next().unwrap().parse()?;

            maps.last_mut().unwrap().insert(
                source_range_start..source_range_start + length,
                destination_range_start,
            );
        }

        let mut fin = u64::MAX;

        for seed in seeds {
            let mut calc = seed;

            for map in &maps {
                let result = map.get_key_value(&calc);
                calc = get_value(calc, result);
            }

            fin = fin.min(calc);
        }

        Ok(fin)
    }

    Some(do_it(input).unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    fn do_it(input: &str) -> Result<u64, Box<dyn Error>> {
        let mut seeds: Vec<Range<u64>> = vec![];
        let mut maps: Vec<RangeMap<u64, u64>> = vec![];

        for line in input.lines() {
            if line.contains("seeds") {
                let line = line.replace("seeds:", "");
                let mut tokens = line.split_whitespace();

                loop {
                    let seed_start: u64 = if let Some(token) = tokens.next() {
                        token.parse()?
                    } else {
                        break;
                    };

                    let len: u64 = tokens.next().unwrap().parse()?;

                    seeds.push(seed_start..seed_start + len);
                }

                continue;
            }

            if line == "" {
                continue;
            }

            if line.contains("map") {
                maps.push(RangeMap::new());
                continue;
            }

            let mut tokens = line.split_whitespace();

            let destination_range_start: u64 = tokens.next().unwrap().parse()?;
            let source_range_start: u64 = tokens.next().unwrap().parse()?;
            let length: u64 = tokens.next().unwrap().parse()?;

            maps.last_mut().unwrap().insert(
                source_range_start..source_range_start + length,
                destination_range_start,
            );
        }

        let mut fin = u64::MAX;

        for seed_range in seeds {
            for seed in seed_range {
                let mut calc = seed;

                for map in &maps {
                    let result = map.get_key_value(&calc);
                    calc = get_value(calc, result);
                }

                fin = fin.min(calc);
            }
        }

        Ok(fin)
    }

    Some(do_it(input).unwrap())
}

fn get_value(calc: u64, result: Option<(&Range<u64>, &u64)>) -> u64 {
    if let Some((source_range, dest)) = result {
        dest + (calc - source_range.start)
    } else {
        calc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
