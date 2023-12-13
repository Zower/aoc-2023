use std::ops::Range;

advent_of_code::solution!(3);

#[derive(Debug, Eq, PartialEq)]
enum Kind {
    Num(char),
    Period,
    Symbol,
    Star,
}

#[derive(Debug)]
struct CachedNum {
    range: Range<usize>,
    num: i32,
}

const ROW_LEN: i32 = 140;

fn is_line_end(index: usize) -> bool {
    (index + 1) % ROW_LEN as usize == 0
}

fn any_adjacent_is_symbol(list: &[Kind], index: usize) -> bool {
    let (row, col) = (index as i32 / ROW_LEN, index as i32 % ROW_LEN);

    let vec = vec![
        list.get(to_idx(row, col - 1)),
        list.get(to_idx(row - 1, col - 1)),
        list.get(to_idx(row - 1, col)),
        list.get(to_idx(row - 1, col + 1)),
        list.get(to_idx(row, col + 1)),
        list.get(to_idx(row + 1, col + 1)),
        list.get(to_idx(row + 1, col)),
        list.get(to_idx(row + 1, col - 1)),
    ];

    vec.iter()
        .any(|it| it.is_some_and(|kind| matches!(kind, Kind::Symbol)))
}

fn to_idx(row: i32, col: i32) -> usize {
    if row < 0 || col < 0 {
        return usize::MAX;
    }

    if col >= ROW_LEN {
        return usize::MAX;
    }

    if row > 139 {
        return usize::MAX;
    }

    (row * ROW_LEN + col) as usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|it| {
            it.chars().map(|char| match char {
                char if char.is_ascii_digit() => Kind::Num(char),
                '.' => Kind::Period,
                _ => Kind::Symbol,
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut sum = 0;

    let mut line_sum = 0;

    let mut line_i = 0;

    for (i, kind) in lines.iter().enumerate() {
        match kind {
            Kind::Num(_) => {
                if i != 0 {
                    let prev = lines.get(i - 1);

                    if prev.is_some_and(|prev| matches!(prev, Kind::Num(_))) {
                        if !is_line_end(i - 1) {
                            if is_line_end(i) {
                                line_i += 1;

                                sum += line_sum;

                                println!("{line_sum}");

                                line_sum = 0;
                            }
                            continue;
                        }
                    }
                }

                let mut final_num = String::new();
                let mut y = i;
                let mut any_valid = false;

                loop {
                    let kind = lines.get(y);

                    let Some(kind) = kind else {
                        break;
                    };

                    match kind {
                        Kind::Num(char) => final_num.push(*char),
                        _ => break,
                    };

                    any_valid = any_valid || any_adjacent_is_symbol(&lines, y);

                    if is_line_end(y) {
                        break;
                    }

                    y += 1;
                }

                if any_valid {
                    line_sum += final_num.parse::<u32>().unwrap();
                }
            }
            _ => (),
        }

        if is_line_end(i) {
            line_i += 1;

            sum += line_sum;

            println!("{line_sum}");

            line_sum = 0;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|it| {
            it.chars().map(|char| match char {
                char if char.is_ascii_digit() => Kind::Num(char),
                '.' => Kind::Period,
                '*' => Kind::Star,
                _ => Kind::Symbol,
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut num_cache = Vec::new();

    for (i, kind) in lines.iter().enumerate() {
        match kind {
            Kind::Num(_) => {
                if i != 0 {
                    let prev = lines.get(i - 1);

                    if prev.is_some_and(|prev| matches!(prev, Kind::Num(_))) {
                        if !is_line_end(i - 1) {
                            continue;
                        }
                    }
                }

                let mut final_num = String::new();
                let mut y = i;

                loop {
                    let kind = lines.get(y);

                    let Some(kind) = kind else {
                        break;
                    };

                    match kind {
                        Kind::Num(char) => final_num.push(*char),
                        _ => break,
                    };

                    if is_line_end(y) {
                        break;
                    }

                    y += 1;
                }

                num_cache.push(CachedNum {
                    range: i..y,
                    num: final_num.parse::<i32>().unwrap(),
                });
            }
            _ => (),
        }
    }

    let mut sum = 0;

    for (i, kind) in lines.iter().enumerate() {
        match kind {
            Kind::Star => {
                if let Some((first, second)) = two_adjacent_is_number(&lines, i) {
                    let num1 = num_cache
                        .iter()
                        .find(|it| it.range.start <= first && it.range.end >= first)
                        .unwrap();

                    let num2 = num_cache
                        .iter()
                        .find(|it| it.range.start <= second && it.range.end >= second)
                        .unwrap();

                    sum += num1.num * num2.num;
                }
            }
            _ => {}
        }
    }

    Some(sum as u32)
}

fn two_adjacent_is_number(list: &[Kind], index: usize) -> Option<(usize, usize)> {
    let (row, col) = (index as i32 / ROW_LEN, index as i32 % ROW_LEN);

    let vec = vec![
        (to_idx(row, col - 1), list.get(to_idx(row, col - 1))),
        (to_idx(row - 1, col - 1), list.get(to_idx(row - 1, col - 1))),
        (to_idx(row - 1, col), list.get(to_idx(row - 1, col))),
        (to_idx(row - 1, col + 1), list.get(to_idx(row - 1, col + 1))),
        (to_idx(row, col + 1), list.get(to_idx(row, col + 1))),
        (to_idx(row + 1, col + 1), list.get(to_idx(row + 1, col + 1))),
        (to_idx(row + 1, col), list.get(to_idx(row + 1, col))),
        (to_idx(row + 1, col - 1), list.get(to_idx(row + 1, col - 1))),
    ];

    let mut num_indices = vec![];

    let mut all_nums_processed = vec![];

    for (idx, it) in vec.iter() {
        if let Some(Kind::Num(_)) = it {
            if !(all_nums_processed.contains(&(idx - 1)) || all_nums_processed.contains(&(idx + 1)))
            {
                num_indices.push(*idx);
            }
            all_nums_processed.push(*idx);
        };
    }

    if num_indices.len() == 2 {
        return Some((num_indices[0], num_indices[1]));
    }

    None
}
