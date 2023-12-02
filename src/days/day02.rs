use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, str::FromStr};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
}

#[derive(Debug, Clone, Copy)]
struct NumBalls {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
}

impl NumBalls {
    fn get_power(self) -> usize {
        self.num_red * self.num_green * self.num_blue
    }
}

impl Round {
    fn valid(&self, num_balls: &NumBalls) -> bool {
        self.num_red <= num_balls.num_red
            && self.num_green <= num_balls.num_green
            && self.num_blue <= num_balls.num_blue
    }
}

impl Game {
    fn is_valid(&self, num_balls: NumBalls) -> bool {
        for round in &self.rounds {
            if !round.valid(&num_balls) {
                return false;
            }
        }
        true
    }

    fn get_power(&self) -> usize {
        let mut max_red = usize::MIN;
        let mut max_green = usize::MIN;
        let mut max_blue = usize::MIN;

        for round in &self.rounds {
            // Update maximum values if current round has higher values
            max_red = max_red.max(round.num_red);
            max_green = max_green.max(round.num_green);
            max_blue = max_blue.max(round.num_blue);
        }

        NumBalls {
            num_red: max_red,
            num_green: max_green,
            num_blue: max_blue,
        }
        .get_power()
    }
}

impl FromStr for Round {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;
        let colour_pairs: Vec<&str> = s.split(',').map(|s| s.trim()).collect();
        for pair in colour_pairs {
            let mut iter = pair.splitn(2, ' ');
            if let Some(quantity_str) = iter.next() {
                if let Some(colour) = iter.next() {
                    if let Ok(quanntity) = quantity_str.parse::<usize>() {
                        match colour {
                            "red" => num_red = quanntity,
                            "green" => num_green = quanntity,
                            "blue" => num_blue = quanntity,
                            _ => (),
                        }
                    }
                }
            }
        }
        Ok(Self {
            num_red,
            num_green,
            num_blue,
        })
    }
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let id = parts[0]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap_or_else(|_| {
                eprintln!("Error parsing data: {:?}", parts[0]);
                std::process::exit(1);
            });
        // .unwrap();
        let rounds: Vec<Round> = parts[1]
            .split(';')
            .map(|round| round.parse::<Round>().unwrap())
            .collect();
        Ok(Self { id, rounds })
    }
}
pub fn solve() -> SolutionPair {
    // Your solution here...

    let input = include_str!("../../input/input02.txt");
    let sol1 = get_part1(input);
    let sol2 = get_part2(input);
    (Solution::from(sol1), Solution::from(sol2))
}

fn get_part1(input: &str) -> usize {
    let num_balls = NumBalls {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .filter(|game| game.as_ref().unwrap().is_valid(num_balls))
        .map(|game| game.unwrap().id)
        .sum()
}

fn get_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .map(|game| game.unwrap().get_power())
        .sum()
}

#[test]
fn test_id_parse() {
    let line = "Game 73: 8 blue, 2 green, 9 red; 2 green, 10 red, 6 blue; 3 blue, 6 green, 2 red";
    let game = line.parse::<Game>();
    assert_eq!(73, game.unwrap().id);
}

#[test]
fn test_color_parse() {
    let line = "Game 73: 8 blue, 2 green, 9 red; 2 green, 10 red, 6 blue; 3 blue, 6 green, 2 red";
    let game = line.parse::<Game>();
    assert_eq!(8, game.unwrap().rounds[0].num_blue);
}

#[test]
fn test_valid_round() {
    let num_balls = NumBalls {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };
    let round = "8 blue, 2 green, 9 red";
    let round = round.parse::<Round>();
    assert_eq!(true, round.unwrap().valid(&num_balls));
}

#[test]
fn test_valid_game() {
    let num_balls = NumBalls {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };
    let line = "Game 73: 8 blue, 2 green, 9 red; 2 green, 10 red, 6 blue; 3 blue, 6 green, 2 red";
    let game = line.parse::<Game>();
    assert_eq!(true, game.unwrap().is_valid(num_balls));
}

#[test]
fn test_invalid_game() {
    let num_balls = NumBalls {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };
    let line = "Game 73: 8 blue, 2 green, 19 red; 2 green, 10 red, 6 blue; 3 blue, 6 green, 2 red";
    let game = line.parse::<Game>();
    assert_eq!(false, game.unwrap().is_valid(num_balls));
}

#[test]
fn test_test_data() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let sol = get_part1(input);
    assert_eq!(8, sol);
}
