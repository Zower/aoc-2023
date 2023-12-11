use std::collections::HashSet;

advent_of_code::solution!(9);

pub fn extract_data(input: &str, mut f: impl FnMut(&[Vec<i32>])) {
    for line in input.lines() {
        let mut tree_values = vec![];

        let mut values: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        tree_values.push(values.clone());

        loop {
            let mut tree_value = vec![];

            let mut iter = values.windows(2);

            while let Some([left, right]) = iter.next() {
                let diff = right - left;

                tree_value.push(diff);
            }

            tree_values.push(tree_value.clone());

            if HashSet::<i32>::from_iter(tree_value.iter().cloned()).len() == 1 {
                f(&tree_values[..]);

                break;
            }

            values = tree_value;
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;

    extract_data(input, |tree_values| {
        let mut last_value = 0;

        for line in tree_values.iter().rev() {
            let last_idx_value = line.last().unwrap() + last_value;

            last_value = last_idx_value;
        }

        sum += last_value;
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;

    extract_data(input, |tree_values| {
        let mut last_value = 0;

        for line in tree_values.iter().rev() {
            let last_idx_value = line.first().unwrap() - last_value;

            last_value = last_idx_value;
        }

        sum += last_value;
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
