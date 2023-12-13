use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug)]
enum Tile {
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Start,
}

fn to_idx(row: usize, col: usize, len: usize) -> usize {
    row * len + col
}

fn get_start_connectors(list: &[Tile], row: usize, col: usize, len: usize) -> (usize, usize) {
    let vec = vec![
        (
            to_idx(row, col - 1, len),
            list.get(to_idx(row, col - 1, len)),
        ),
        (
            to_idx(row.checked_sub(1).unwrap_or(usize::MAX / 100), col - 1, len),
            list.get(to_idx(
                row.checked_sub(1).unwrap_or(usize::MAX / 100),
                col - 1,
                len,
            )),
        ),
        (
            to_idx(row.checked_sub(1).unwrap_or(usize::MAX / 100), col, len),
            list.get(to_idx(
                row.checked_sub(1).unwrap_or(usize::MAX / 100),
                col,
                len,
            )),
        ),
        (
            to_idx(row.checked_sub(1).unwrap_or(usize::MAX / 100), col + 1, len),
            list.get(to_idx(
                row.checked_sub(1).unwrap_or(usize::MAX / 100),
                col + 1,
                len,
            )),
        ),
        (
            to_idx(row, col + 1, len),
            list.get(to_idx(row, col + 1, len)),
        ),
        (
            to_idx(row + 1, col + 1, len),
            list.get(to_idx(row + 1, col + 1, len)),
        ),
        (
            to_idx(row + 1, col, len),
            list.get(to_idx(row + 1, col, len)),
        ),
        (
            to_idx(row + 1, col - 1, len),
            list.get(to_idx(row + 1, col - 1, len)),
        ),
    ];

    let mut nums = Vec::new();

    for (i, tile) in vec {
        if let Some(tile) = tile {
            if !matches!(tile, Tile::Ground) {
                let (o_row, o_col) = (i / len, i % len);
                let (f, s) = tile_connects_to(*tile, len, o_row, o_col);

                let start = to_idx(row, col, len);

                if f == start || s == start {
                    nums.push(i);
                } else {
                }
            }
        }
    }

    (nums[0], nums[1])
}

fn from_idx(start: usize, len: usize) -> (usize, usize) {
    let (row, col) = (start / len, start % len);

    (row, col)
}

fn tile_connects_to(tile: Tile, len: usize, row: usize, col: usize) -> (usize, usize) {
    let toi = |row, col| to_idx(row, col, len);

    let (first, second) = match tile {
        Tile::NorthAndSouth => (toi(row - 1, col), toi(row + 1, col)),
        Tile::EastAndWest => (toi(row, col - 1), toi(row, col + 1)),
        Tile::NorthAndEast => (toi(row - 1, col), toi(row, col + 1)),
        Tile::NorthAndWest => (toi(row - 1, col), toi(row, col - 1)),
        Tile::SouthAndWest => (toi(row + 1, col), toi(row, col - 1)),
        Tile::SouthAndEast => (toi(row + 1, col), toi(row, col + 1)),
        Tile::Ground => unreachable!(),
        Tile::Start => unreachable!(),
    };

    (first, second)
}

fn get_loop_path(input: &str) -> (Vec<Tile>, Vec<usize>) {
    let len = input.lines().count();

    let mut tiles = Vec::with_capacity(len * len);

    for line in input.lines() {
        tiles.extend(line.chars().map(|c| match c {
            '|' => Tile::NorthAndSouth,
            '-' => Tile::EastAndWest,
            'L' => Tile::NorthAndEast,
            'J' => Tile::NorthAndWest,
            '7' => Tile::SouthAndWest,
            'F' => Tile::SouthAndEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        }));
    }

    let start = tiles.iter().position(|t| matches!(t, Tile::Start)).unwrap();

    let (row, col) = from_idx(start, len);

    let (_, second) = get_start_connectors(&tiles[..], row, col, len);

    let mut prev = start;
    let mut n = second;

    let mut vec = vec![];

    vec.push(n);

    while n != start {
        let (row, col) = from_idx(n, len);

        let (first, second) = tile_connects_to(tiles[n], len, row, col);

        let next = if first == prev { second } else { first };

        prev = n;
        n = next;

        vec.push(next);
    }

    (tiles, vec)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, path) = get_loop_path(input);

    Some(path.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let len = input.lines().count();
    let (tiles, path) = get_loop_path(input);

    let path_map: HashSet<usize> = HashSet::from_iter(path.iter().cloned());

    let count = tiles
        .iter()
        .enumerate()
        .filter(|(idx, _)| {
            if path_map.contains(idx) {
                return false;
            }

            let (mut r, mut c) = from_idx(*idx, len);

            let mut hits = 0;

            loop {
                if c == len - 1 || r == 0 {
                    break;
                }

                r -= 1;
                c += 1;

                let tile_there = to_idx(r, c, len);

                let Some(tile) = tiles.get(tile_there) else {
                    break;
                };

                if path_map.contains(&tile_there) {
                    match tile {
                        Tile::NorthAndSouth => hits += 1,
                        Tile::EastAndWest => hits += 1,
                        Tile::NorthAndEast => hits += 1,
                        Tile::NorthAndWest => hits += 2,
                        Tile::SouthAndWest => hits += 1,
                        Tile::SouthAndEast => hits += 2,
                        Tile::Start => hits += 2,
                        Tile::Ground => {}
                    }
                }
            }

            hits % 2 != 0
        })
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));

        assert_eq!(result, Some(10));
    }
}
