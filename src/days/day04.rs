use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
use std::str::FromStr;

#[derive(Clone)]
struct LotteryCard {
    winning_nums: Vec<usize>,
    card_nums: Vec<usize>,
    points: usize,
    matches: usize,
    copies: usize,
}

impl FromStr for LotteryCard {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((_, rest)) = s.split_once(':') {
            let split_strings: Vec<&str> = rest.split('|').collect();
            let winning_nums = split_strings[0].trim();
            let card_nums = split_strings[1].trim();

            let winning_nums: Vec<usize> = winning_nums
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let card_nums: Vec<usize> = card_nums
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let matching_nums = card_nums
                .iter()
                .filter(|&num| winning_nums.contains(num))
                .count();

            let mut points = 0;
            for i in 0..matching_nums {
                points = 2usize.pow(i as u32);
            }
            Ok(LotteryCard {
                winning_nums,
                card_nums,
                points,
                matches: matching_nums,
                copies: 1,
            })
        } else {
            Err("No : found")
        }
    }
}

fn check_for_winners_and_add(mut cards: Vec<LotteryCard>) -> usize {
    let num_cards = cards.len();
    for index in 0..num_cards {
        if cards[index].matches > 0 {
            let mut last_index = index + 1 + cards[index].matches;
            if index + cards[index].matches > num_cards {
                last_index = num_cards;
            }
            for next_index in index + 1..last_index {
                cards[next_index].copies += 1 * cards[index].copies;
            }
        }
    }

    cards.iter().map(|card| card.copies).sum()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("../../input/input04.txt");
    let sol1: usize = solve_part1(input);
    let sol2: usize = solve_part2(input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<LotteryCard>())
        .map(|card| card.unwrap().points)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let mut lottery_cards: Vec<LotteryCard> = input
        .lines()
        .map(|line| line.parse::<LotteryCard>().unwrap())
        .collect();

    check_for_winners_and_add(lottery_cards)
}

#[test]
fn test_sol1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(13, solve_part1(input));
}

#[test]
fn test_sol2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(30, solve_part2(input));
}
