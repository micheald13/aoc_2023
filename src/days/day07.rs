use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<CardType>,
    // cards_part2: Vec<CardTypePart2>,
    result: HandType,
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    FiveofAKind,
    FourofAKind,
    FullHouse,
    ThreeofAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum CardTypePart2 {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Jack,
}

fn parse_card(c: char) -> CardType {
    match c.to_ascii_uppercase() {
        'A' => CardType::Ace,
        'K' => CardType::King,
        'Q' => CardType::Queen,
        'J' => CardType::Jack,
        'T' => CardType::Ten,
        '9' => CardType::Nine,
        '8' => CardType::Eight,
        '7' => CardType::Seven,
        '6' => CardType::Six,
        '5' => CardType::Five,
        '4' => CardType::Four,
        '3' => CardType::Three,
        '2' => CardType::Two,
        _ => CardType::Two,
    }
}

fn calc_hand(cards: &Vec<CardType>) -> HandType {
    let mut card_counts: HashMap<&CardType, usize> = HashMap::new();

    for card in cards {
        let count = card_counts.entry(card).or_insert(0);
        *count += 1;
    }

    let mut pair_count = 0;
    let mut triple_count = 0;
    let mut four_count = 0;
    let mut five_count = 0;

    for count in card_counts.values() {
        match count {
            2 => pair_count += 1,
            3 => triple_count += 1,
            4 => four_count += 1,
            5 => five_count += 1,
            _ => (),
        }
    }

    if five_count == 1 {
        HandType::FiveofAKind
    } else if four_count == 1 {
        HandType::FourofAKind
    } else if triple_count == 1 && pair_count == 1 {
        HandType::FullHouse
    } else if triple_count == 1 {
        HandType::ThreeofAKind
    } else if pair_count == 2 {
        HandType::TwoPair
    } else if pair_count == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        // First, compare based on HandType
        let hand_type_comparison = a.result.cmp(&b.result);

        // If HandType is the same, compare based on CardType values
        if hand_type_comparison == std::cmp::Ordering::Equal {
            // println!("{a:?} -- {b:?}");
            for (a_card, b_card) in a.cards.iter().zip(&b.cards) {
                // println!("{a_card:?} -- {b_card:?}");
                let card_comparison = a_card.cmp(b_card);
                if card_comparison != std::cmp::Ordering::Equal {
                    return card_comparison;
                }
            }
        }
        hand_type_comparison
    });
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        if let Some((first, second)) = line.split_once(" ") {
            let cards: Vec<CardType> = first.chars().map(|c| parse_card(c)).collect();
            // let cards_part2: Vec<CardTypePart2> = first.chars().map(|c| parse_card(c)).collect();
            let bid = second.parse::<usize>().unwrap();
            let result = calc_hand(&cards);
            // let result_part2 = calc_hand_part(&cards);
            hands.push(Hand {
                cards,
                // cards_part2,
                result,
                bid,
            })
        }
    }
    hands
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("../../input/input07.txt");
    let sol1: usize = solve_part1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(input: &str) -> usize {
    let mut hands = parse_input(input);
    sort_hands(&mut hands);
    // println!("{hands:?}");
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

#[test]
fn test_parse() {
    let input = "32T3K 765";
    println!("{:?}", parse_input(input));
}

#[test]
fn test_sample_sol1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(6440, solve_part1(input));
}

#[test]
fn test_reddit_sample() {
    let input = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43";
    assert_eq!(1343, solve_part1(input));
}

#[test]
fn test_reddit_sample_2() {
    let input = "2345A 2
2345J 5
J345A 3
32T3K 7
T55J5 17
KK677 11
KTJJT 23
QQQJA 19
JJJJJ 29
JAAAA 37
AAAAJ 43
AAAAA 53
2AAAA 13
2JJJJ 41
JJJJ2 31";
    assert_eq!(3542, solve_part1(input));
}
