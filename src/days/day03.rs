use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq)]
struct Number {
    line: usize,
    num: usize,
    position: (usize, usize),
}

#[derive(Debug, PartialEq)]
struct Gear {
    line: usize,
    position: (usize, usize),
}

fn is_symbol(c: char) -> bool {
    if !c.is_digit(10) && c != '.' {
        true
    } else {
        false
    }
}

fn check_slice_for_symbol(slice: &str) -> bool {
    for c in slice.chars() {
        if is_symbol(c) {
            return true;
        }
    }
    false
}

impl Gear {
    fn check_adjacent(&self, number: &Number) -> bool {
        if self.position.1 < number.position.0 || number.position.1 < self.position.0 {
            return false;
        }
        true
    }
}

fn find_all_gears(input: &str) -> Vec<Gear> {
    let mut result = Vec::new();
    let mut start;
    let mut end;
    let lines: Vec<&str> = input.lines().collect();
    for (current_index, line) in lines.iter().enumerate() {
        for (index, c) in line.char_indices() {
            if c == '*' {
                if index > 0 {
                    start = index - 1;
                } else {
                    start = index;
                }
                if index < input.len() {
                    end = index + 1;
                } else {
                    end = index;
                }
                result.push(Gear {
                    line: current_index,
                    position: (start, end),
                });
            }
        }
    }
    result
}

fn find_all_numbers(input: &str) -> Vec<Number> {
    let mut result = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for (current_index, line) in lines.iter().enumerate() {
        let mut start_index = None;
        for (index, c) in line.char_indices() {
            if c.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(index);
                }
            } else {
                if start_index.is_some() {
                    if let Some(start) = start_index.take() {
                        let end = index - 1;
                        let num_str: &str = &line[start..=end];
                        if let Ok(num) = num_str.parse::<usize>() {
                            result.push(Number {
                                line: current_index,
                                num,
                                position: (start, end),
                            });
                        }
                    }
                }
            }
        }

        // If the last character is a digit, record the end index
        if let Some(start) = start_index {
            let end = line.len() - 1;
            let num_str: &str = &line[start..=end];
            if let Ok(num) = num_str.parse::<usize>() {
                result.push(Number {
                    line: current_index,
                    num,
                    position: (start, end),
                });
            }
        }
    }
    result
}

fn find_number_indexes(input: &str, line_num: usize) -> Vec<Number> {
    let mut result = Vec::new();
    let mut start_index = None;
    for (index, c) in input.char_indices() {
        if c.is_digit(10) {
            if start_index.is_none() {
                start_index = Some(index);
            }
        } else {
            if start_index.is_some() {
                if let Some(start) = start_index.take() {
                    let end = index - 1;
                    let num_str: &str = &input[start..=end];
                    if let Ok(num) = num_str.parse::<usize>() {
                        result.push(Number {
                            line: line_num,
                            num,
                            position: (start, index),
                        });
                    }
                }
            }
        }
    }

    // If the last character is a digit, record the end index
    if let Some(start) = start_index {
        let end = input.len() - 1;
        let num_str: &str = &input[start..=end];
        if let Ok(num) = num_str.parse::<usize>() {
            result.push(Number {
                line: line_num,
                num,
                position: (start, end),
            });
        }
    }
    result
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("../../input/input03.txt");
    let sol1 = solve_part1(input);
    let sol2 = solve_part2(input);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part2(input: &str) -> usize {
    let mut sol: usize = 0;
    let gears = find_all_gears(input);
    let nums = find_all_numbers(input);
    for gear in gears {
        let mut adjacent: Vec<usize> = Vec::new();
        let result: Vec<&Number> = nums
            .iter()
            .filter(|&n| {
                n.line == gear.line
                    || n.line == gear.line.saturating_sub(1)
                    || (n.line == gear.line + 1)
            })
            .collect();
        for num in result {
            if gear.check_adjacent(num) {
                adjacent.push(num.num);
            }
        }

        if adjacent.len() == 2 {
            sol += adjacent.first().unwrap() * adjacent.last().unwrap();
        }
    }
    sol
}

fn solve_part1(input: &str) -> usize {
    let mut sol: usize = 0;
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    for (current_index, line) in lines.iter().enumerate() {
        let nums = find_number_indexes(line, current_index);
        for num in nums {
            let slice: &str = &line[num.position.0..=num.position.1];
            if check_slice_for_symbol(slice) {
                sol += num.num;
                continue;
            }

            // Check the previous line
            if current_index > 0 {
                let prev_line = lines[current_index - 1];
                let prev_slice: &str = &prev_line[num.position.0..=num.position.1];
                if check_slice_for_symbol(prev_slice) {
                    sol += num.num;
                    continue;
                }
            }

            // Check the next line
            if current_index < len - 1 {
                let next_line = lines[current_index + 1];
                let next_slice: &str = &next_line[num.position.0..=num.position.1];
                if check_slice_for_symbol(next_slice) {
                    sol += num.num;
                    continue;
                }
            }
        }
    }
    sol
}

#[test]
fn test_is_symbol() {
    assert_eq!(true, is_symbol('*'));
}

#[test]
fn test_period_is_not_symbol() {
    assert_eq!(false, is_symbol('.'));
}

#[test]
fn test_digit_is_not_symbol() {
    assert_eq!(false, is_symbol('5'));
}

#[test]
fn test_check_slice_for_symbol_true() {
    assert_eq!(true, check_slice_for_symbol(".......*"));
}

#[test]
fn test_check_slice_for_symbol_false() {
    assert_eq!(false, check_slice_for_symbol(".......1"));
}

#[test]
fn test_find_number_ranges() {
    let input1 = "467..114..";
    let result1 = find_number_indexes(input1, 1);
    assert_eq!(
        result1,
        vec![
            Number {
                line: 1,
                num: 467,
                position: (0, 3)
            },
            Number {
                line: 1,
                num: 114,
                position: (4, 8)
            }
        ]
    );
}

#[test]
fn test_sample_sol() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(4361, solve_part1(input));
}

#[test]
fn test_sample_sol2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(467835, solve_part2(input));
}
