advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_char(char: char) -> Option<Card> {
        Some(match char {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return None,
        })
    }
}

#[derive(Debug)]
struct Hand {
    hand: [Card; 5],
    kind: HandKind,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Unknown,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let mut hand = parts.next().unwrap().chars();

            let hand: [Card; 5] = [(); 5].map(|_| Card::from_char(hand.next().unwrap()).unwrap());

            let kind = calculate_kind(hand);

            let bid: u32 = parts.next().unwrap().parse().unwrap();

            Hand { hand, kind, bid }
        })
        .collect();

    hands.sort_by_key(|hand| hand.hand);
    hands.sort_by_key(|hand| hand.kind);

    let mut sum = 0;

    for (rank, hand) in hands.iter().rev().enumerate() {
        sum += (rank as u32 + 1) * hand.bid;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let mut hand = parts.next().unwrap().chars();

            let hand: [Card; 5] = [(); 5].map(|_| Card::from_char(hand.next().unwrap()).unwrap());

            let mut map: HashMap<Card, usize> = HashMap::new();

            for card in hand {
                if !matches!(card, Card::J) {
                    let val = map.get(&card).unwrap_or(&0);
                    map.insert(card, val + 1);
                }
            }

            let most_common = map
                .into_iter()
                .max_by_key(|it| it.1)
                .unwrap_or((Card::J, 5))
                .0;

            let new_cards = hand.map(|card| match card {
                Card::J => most_common,
                _ => card,
            });

            let kind = calculate_kind(new_cards);

            let bid: u32 = parts.next().unwrap().parse().unwrap();

            Hand { hand, kind, bid }
        })
        .collect();

    hands.sort_by_key(|hand| hand.hand);
    hands.sort_by_key(|hand| hand.kind);

    let mut sum = 0;

    for (rank, hand) in hands.iter().rev().enumerate() {
        sum += (rank as u32 + 1) * hand.bid;
    }

    Some(sum)
}

fn calculate_kind(cards: [Card; 5]) -> HandKind {
    let set = HashSet::from(cards);

    match set.len() {
        1 => HandKind::FiveOfAKind,
        2 => {
            let first = cards[0];
            match cards.iter().filter(|&card| *card == first).count() {
                1 | 4 => HandKind::FourOfAKind,
                2 | 3 => HandKind::FullHouse,
                _ => unreachable!(),
            }
        }
        3 => {
            if cards.iter().any(|card| {
                cards
                    .iter()
                    .filter(|&inner_card| inner_card == card)
                    .count()
                    == 3
            }) {
                HandKind::ThreeOfAKind
            } else {
                HandKind::TwoPair
            }
        }
        4 => HandKind::OnePair,
        5 => HandKind::HighCard,
        _ => HandKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
