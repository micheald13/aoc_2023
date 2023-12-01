use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
use std::str::FromStr;

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/input01.txt");
    let sol2: usize = input
        .lines()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .replace("zero", "z0o")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|digit| digit as usize)
                .collect::<Vec<usize>>()
        })
        .map(|vec| {
            let first = *vec.first().unwrap_or(&0);
            let last = *vec.last().unwrap_or(&0);
            first * 10 + last
        })
        .sum();

    let sol1: usize = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|digit| digit as usize)
                .collect::<Vec<usize>>()
        })
        .map(|vec| {
            let first = *vec.first().unwrap_or(&0);
            let last = *vec.last().unwrap_or(&0);
            first * 10 + last
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
