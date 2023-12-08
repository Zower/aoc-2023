use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut tokens = line.split_whitespace();

        // Card
        let _ = tokens.next();
        let _ = tokens.next();

        let mut winning = Vec::new();
        let mut have = Vec::new();

        let mut processing_winning = true;

        loop {
            match tokens.next() {
                Some("|") => {
                    processing_winning = false;
                }
                Some(num) => {
                    let num = num.parse::<u32>();

                    if processing_winning {
                        winning.push(num);
                    } else {
                        have.push(num);
                    }
                }
                None => {
                    let mut count = 0;

                    for num in &have {
                        if winning.contains(num) {
                            count += 1;
                        }
                    }

                    if count != 0 {
                        sum += 2_u32.pow(count - 1)
                    }

                    break;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut copies = HashMap::new();

    for line in input.lines() {
        let mut tokens = line.split_whitespace();

        // Card
        let _ = tokens.next();

        let game_nr = tokens
            .next()
            .unwrap()
            .replace(":", "")
            .parse::<u32>()
            .unwrap();

        let mut winning = Vec::new();
        let mut have = Vec::new();

        let mut processing_winning = true;

        loop {
            match tokens.next() {
                Some("|") => {
                    processing_winning = false;
                }
                Some(num) => {
                    let num = num.parse::<u32>();

                    if processing_winning {
                        winning.push(num);
                    } else {
                        have.push(num);
                    }
                }
                None => {
                    let mut count = 0;

                    for num in &have {
                        if winning.contains(num) {
                            count += 1;
                        }
                    }

                    for _ in 0..*copies.get(&game_nr).unwrap_or(&0) + 1 {
                        sum += 1;

                        for i in game_nr + 1..=game_nr + count {
                            let entry = copies.entry(i).or_insert(0);

                            *entry = *entry + 1;
                        }
                    }

                    break;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
