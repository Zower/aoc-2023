advent_of_code::solution!(11);

use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Galaxy,
    Space,
}

fn get_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Space,
                    '#' => Tile::Galaxy,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn get_expanded_space(space: &mut Vec<Vec<Tile>>) -> (Vec<usize>, Vec<usize>) {
    let mut indices = Vec::new();

    for (i, line) in space.into_iter().enumerate() {
        if !line.iter().any(|it| *it == Tile::Galaxy) {
            indices.push(i);
        }
    }

    let mut other_indices = Vec::new();

    for i in 0..space[0].len() {
        let mut all_empty = true;

        for line in space.iter() {
            if line[i] == Tile::Galaxy {
                all_empty = false;
            }
        }

        if all_empty {
            other_indices.push(i);
        }
    }

    (indices, other_indices)
}

fn calculate_length(space: Vec<Vec<Tile>>, rows: Vec<usize>, cols: Vec<usize>) -> usize {
    let mut galaxies = vec![];

    for x in 0..space.len() {
        for y in 0..space[x].len() {
            if space[x][y] == Tile::Galaxy {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    let mut sum = 0;

    for pair in galaxies.into_iter().combinations(2) {
        let (first, second) = (pair[0], pair[1]);

        let extra_adds = rows
            .iter()
            .filter(|&&extra_row| first.1 < extra_row as i32 && second.1 > extra_row as i32)
            .count()
            + cols
                .iter()
                .filter(|&&extra_col| first.0 < extra_col as i32 && second.0 < extra_col as i32)
                .count();

        let len = (first.0 - second.0).abs() + (first.1 - second.1).abs() + extra_adds as i32;

        sum += len;
    }

    sum as usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut space = get_input(input);

    let (r, c) = get_expanded_space(&mut space);

    Some(calculate_length(space, r, c) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut space = get_input(input);
    println!("before");

    let (r, c) = get_expanded_space(&mut space);

    Some(calculate_length(space, r, c) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
