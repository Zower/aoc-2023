advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let (game_nr, inputs) = get_inputs(line);

        let possible = inputs.iter().all(|it| match it.color.as_str() {
            "red" => it.count <= MAX_RED,
            "green" => it.count <= MAX_GREEN,
            "blue" => it.count <= MAX_BLUE,
            _ => unreachable!(),
        });

        if possible {
            sum += game_nr;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let (_, runs) = get_inputs(line);
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;

        for it in runs {
            match it.color.as_str() {
                "red" => {
                    if it.count > max_red {
                        max_red = it.count;
                    }
                }
                "green" => {
                    if it.count > max_green {
                        max_green = it.count;
                    }
                }
                "blue" => {
                    if it.count > max_blue {
                        max_blue = it.count;
                    }
                }
                _ => unreachable!(),
            }
        }

        sum += max_red * max_green * max_blue;
    }

    Some(sum)
}

fn get_inputs(line: &str) -> (u32, Vec<OneCount>) {
    let data = line
        .replace("Game ", "")
        .replace(":", ",")
        .replace(";", ",");

    let mut split = data.split(",");

    let game_nr = split.next().unwrap().parse::<u32>().unwrap();

    let mut inputs = vec![];

    for combo in split {
        let mut split = combo.split_whitespace();

        let nr = split.next().unwrap();
        let count = nr.parse::<u32>().unwrap();

        let color = split.next().unwrap();

        inputs.push(OneCount {
            color: color.into(),
            count,
        })
    }

    (game_nr, inputs)
}

#[derive(Debug)]
struct OneCount {
    color: String,
    count: u32,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
