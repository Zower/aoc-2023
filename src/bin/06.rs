advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let records = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut sum = 1;

    for (time, record) in times.iter().zip(records.iter()) {
        let mut count = 0;
        for i in 1..*time {
            let dist = i * (time - i);

            if dist > *record {
                count += 1;
            }
        }

        sum *= count;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let record = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let mut sum = 1;

    let mut count = 0;

    for i in 1..time {
        let dist = i * (time - i);

        if dist > record {
            count += 1;
        }
    }

    sum *= count;

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
