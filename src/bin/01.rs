advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input.lines()))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input.lines().map(|line| {
        line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9e")
            .replace("zero", "z0o")
    })))
}

fn solve<T: AsRef<str>>(iter: impl Iterator<Item = T>) -> u32 {
    iter.into_iter()
        .map(|line| {
            let line = line.as_ref();

            let mut digits = line.chars().filter(char::is_ascii_digit);

            let first = digits.next().unwrap_or('0');
            let text_number = format!("{}{}", first, digits.last().unwrap_or(first));

            text_number.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
