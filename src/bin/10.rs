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
            to_idx(row - 1, col - 1, len),
            list.get(to_idx(row - 1, col - 1, len)),
        ),
        (
            to_idx(row - 1, col, len),
            list.get(to_idx(row - 1, col, len)),
        ),
        (
            to_idx(row - 1, col + 1, len),
            list.get(to_idx(row - 1, col + 1, len)),
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
                }
            }
        }
    }

    (nums[0], nums[1])
}

pub fn part_one(input: &str) -> Option<u32> {
    let len = input.lines().count();

    let mut tiles = Vec::with_capacity(len * len);

    for line in input.lines() {
        let tiles_i = line.chars().map(|c| match c {
            '|' => Tile::NorthAndSouth,
            '-' => Tile::EastAndWest,
            'L' => Tile::NorthAndEast,
            'J' => Tile::NorthAndWest,
            '7' => Tile::SouthAndWest,
            'F' => Tile::SouthAndEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        });

        tiles.extend(tiles_i);
    }

    let start = tiles.iter().position(|t| matches!(t, Tile::Start)).unwrap();

    let (row, col) = from_idx(start, len);

    let (_, second) = get_start_connectors(&tiles[..], row, col, len);

    let mut prev = start;
    let mut n = second;

    let mut count = 1;

    while n != start {
        let (row, col) = from_idx(n, len);

        let (first, second) = tile_connects_to(tiles[n], len, row, col);

        let next = if first == prev { second } else { first };

        prev = n;
        n = next;

        count += 1;
    }

    Some(count / 2)
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
