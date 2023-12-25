use crate::{Solution, SolutionPair};
use std::str::FromStr;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Nope,
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    entries: Vec<MapEntry>,
}

#[derive(Debug)]
struct MapEntry {
    location: String,
    left: String,
    right: String,
}

impl Map {
    fn new() -> Map {
        Map {
            directions: Vec::new(),
            entries: Vec::new(),
        }
    }
}

impl FromStr for MapEntry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('=').collect();

        if parts.len() != 2 {
            return Err("Invalid format");
        }

        let location = parts[0].trim();
        let values: Vec<&str> = parts[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();

        if values.len() != 2 {
            return Err("Invalid format");
        }

        let left = values[0][2..].trim();
        let right = values[1].trim();

        Ok(MapEntry {
            location: location.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

fn parse_direction(c: char) -> Direction {
    match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => Direction::Nope,
    }
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines();
    let mut map = Map::new();

    map.directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| parse_direction(c))
        .collect();

    map.entries = input.lines().filter_map(|line| line.parse().ok()).collect();

    map
}

fn traverse_map(map: Map) -> usize {
    let mut location = "AAA";
    let end = "ZZZ";
    let mut steps = 0;
    let mut finished = false;

    //Find the start
    while !finished {
        for direction in &map.directions {
            if let Some(entry) = map.entries.iter().find(|&e| e.location == location) {
                location = match direction {
                    Direction::Left => &entry.left,
                    Direction::Right => &entry.right,
                    Direction::Nope => &entry.right,
                };
                steps += 1;
                if location == end {
                    finished = true;
                    break;
                }
            } else {
                println!("Entry with location '{}' not found", location);
            }
        }
    }
    steps
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("../../input/input08.txt");
    let map = parse_input(input);
    let sol1: usize = solve_part1(map);

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(map: Map) -> usize {
    traverse_map(map)
}

#[test]
fn test_sample_sol1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let map = parse_input(input);
    assert_eq!(2, traverse_map(map));

    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let map = parse_input(input);
    assert_eq!(6, traverse_map(map));
}
