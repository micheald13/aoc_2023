use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
use itertools::Itertools;

fn parse_input(input: &str) -> (&str, Vec<usize>) {
    let (springs, list) = input.split_once(' ').unwrap();

    let list = list
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (springs, list)
}

fn solve_part1(springs: &str, nums: Vec<usize>) -> usize {
    let springs = format!(".{}", springs.trim_end_matches('.'));
    let springs = springs.chars().collect_vec();

    let mut dp = vec![0; springs.len() + 1];
    dp[0] = 1;

    for (i, _) in springs.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }
    println!("{:?}", dp);

    for num in nums {
        let mut n_dp = vec![0; springs.len() + 1];
        let mut chunk = 0;

        for (i, &c) in springs.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= num && springs[i - num] != '#' {
                n_dp[i + 1] += dp[i - num];
            }
        }

        println!("{:?}", n_dp);
        println!("");
        dp = n_dp;
    }

    *dp.last().unwrap()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol2: u64 = 0;

    let input = include_str!("../../input/input12.txt");
    let sol1 = input
        .lines()
        .map(|line| {
            let (springs, nums) = parse_input(line);
            solve_part1(springs, nums)
        })
        .sum::<usize>();

    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test_parse_line() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let num = input
        .lines()
        .map(|line| {
            let (springs, nums) = parse_input(line);
            solve_part1(springs, nums)
        })
        .sum::<usize>();
    assert_eq!(21, num);
}
