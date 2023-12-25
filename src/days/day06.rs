use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let time_regex = Regex::new(r"\d+").unwrap();
    let distance_regex = Regex::new(r"\d+").unwrap();

    let time_values: Vec<usize> = time_regex
        .captures_iter(input.lines().nth(0).unwrap())
        .map(|capture| capture[0].parse().unwrap())
        .collect();

    let distance_values: Vec<usize> = distance_regex
        .captures_iter(input.lines().nth(1).unwrap())
        .map(|capture| capture[0].parse().unwrap())
        .collect();

    for (time, distance) in time_values.into_iter().zip(distance_values) {
        let race = Race { time, distance };
        races.push(race);
    }

    races
}

fn parse_input_2(input: &str) -> Race {
    let line_regex = Regex::new(r"\d+").unwrap();
    let time_line = input.lines().nth(0).unwrap();
    let distance_line = input.lines().nth(1).unwrap();

    let time: usize = time_line
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: usize = distance_line
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();

    Race { time, distance }
}

fn calc_num_possible_wins(race: &Race) -> usize {
    let mut counter = 0;
    for hold_time in 0..race.time {
        if hold_time * (race.time - hold_time) > race.distance {
            counter += 1;
        }
    }
    counter
}

fn solve_part1(races: Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| calc_num_possible_wins(race))
        .product()
}

fn solve_part2(race: Race) -> usize {
    calc_num_possible_wins(&race)
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("../../input/input06.txt");
    let races = parse_input(input);
    let race = parse_input_2(input);

    let sol1 = solve_part1(races);
    let sol2 = solve_part2(race);

    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test_parsing() {
    let input = r"Time:        49     97     94     94
        Distance:   263   1532   1378   1851
    ";
    let expected_result = vec![
        Race {
            time: 49,
            distance: 263,
        },
        Race {
            time: 97,
            distance: 1532,
        },
        Race {
            time: 94,
            distance: 1378,
        },
        Race {
            time: 94,
            distance: 1851,
        },
    ];

    assert_eq!(parse_input(input), expected_result);
}

#[test]
fn test_calc_num_possible_wins() {
    let race = Race {
        time: 7,
        distance: 9,
    };
    assert_eq!(4, calc_num_possible_wins(&race));
}

#[test]
fn test_sol_part1() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    let races = parse_input(input);
    assert_eq!(288, solve_part1(races));
}

#[test]
fn test_parse_input_2() {
    let input = "Time:      7  15   30
Distance:  9  40  200";

    let expected_result = Race {
        time: 71530,
        distance: 940200,
    };

    assert_eq!(parse_input_2(input), expected_result);
}
