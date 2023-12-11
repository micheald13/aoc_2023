use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
struct GalaxyPos {
    x: usize,
    y: usize,
}

fn diff(num1: usize, num2: usize) -> usize {
    if num1 > num2 {
        num1 - num2
    } else {
        num2 - num1
    }
}

impl GalaxyPos {
    fn distance(self, gal: &GalaxyPos) -> usize {
        diff(self.x, gal.x) + diff(self.y, gal.y)
    }
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn get_galaxies(input: Vec<Vec<bool>>) -> Vec<GalaxyPos> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, vec)| {
            vec.iter()
                .enumerate()
                .filter(|(_, b)| **b)
                .map(move |(x, _)| GalaxyPos { x, y })
        })
        .collect()
}

fn expand_galaxy(
    galaxy: &GalaxyPos,
    empty_rows: &HashSet<usize>,
    empty_columns: &HashSet<usize>,
    size: usize,
) -> GalaxyPos {
    GalaxyPos {
        x: galaxy.x + size * empty_rows.iter().filter(|p| p < &&galaxy.x).count(),
        y: galaxy.y + size * empty_columns.iter().filter(|p| p < &&galaxy.y).count(),
    }
}

fn calc_all_distances(galaxies: Vec<GalaxyPos>) -> usize {
    let mut total = 0;
    for (i, gal) in galaxies.iter().enumerate() {
        for j in 0..i {
            total += gal.distance(&galaxies[j]);
        }
    }
    total
}

pub fn solve() -> SolutionPair {
    // Your solution here...

    let input = include_str!("../../input/input11.txt");
    let parsed_input = parse_input(&input);
    let galaxies = get_galaxies(parsed_input);

    let columns: HashSet<_> = galaxies.iter().map(|p| p.y).collect();
    let empty_columns: HashSet<_> = (0..*columns.iter().max().unwrap())
        .filter(|c| !columns.contains(c))
        .collect();

    let rows: HashSet<_> = galaxies.iter().map(|p| p.x).collect();
    let empty_rows: HashSet<_> = (0..*rows.iter().max().unwrap())
        .filter(|c| !rows.contains(c))
        .collect();

    let sol1 = calc_all_distances(
        galaxies
            .iter()
            .map(|galaxy| expand_galaxy(galaxy, &empty_rows, &empty_columns, 1))
            .collect(),
    );

    let sol2 = calc_all_distances(
        galaxies
            .iter()
            .map(|galaxy| expand_galaxy(galaxy, &empty_rows, &empty_columns, 999999))
            .collect(),
    );

    (Solution::from(sol1), Solution::from(sol2))
}

#[test]
fn test_parse_example_1() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let data = parse_input(input);
    let galaxies = get_galaxies(data);
    let columns: HashSet<_> = galaxies.iter().map(|p| p.y).collect();
    let empty_columns: HashSet<_> = (0..*columns.iter().max().unwrap())
        .filter(|c| !columns.contains(c))
        .collect();

    let rows: HashSet<_> = galaxies.iter().map(|p| p.x).collect();
    let empty_rows: HashSet<_> = (0..*rows.iter().max().unwrap())
        .filter(|c| !rows.contains(c))
        .collect();
    println!("{empty_columns:?}");
    println!("{empty_rows:?}");
}
