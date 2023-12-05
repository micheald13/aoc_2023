use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
use rayon::prelude::*;

#[derive(Debug)]
struct Mapping {
    source: usize,
    dest: usize,
    len: usize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seeds_part2: Vec<Mapping>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    fn new() -> Almanac {
        Almanac {
            seeds: Vec::new(),
            seeds_part2: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }
}

fn parse_seeds_part2(seeds: &Vec<usize>) -> Vec<Mapping> {
    let mut array: Vec<Mapping> = Vec::new();
    for part in seeds.chunks_exact(2) {
        array.push(Mapping {
            source: part[0],
            dest: 0,
            len: part[1],
        })
    }
    array
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();
    let mut almanac = Almanac::new();

    if let Some(seeds_line) = lines.next() {
        almanac.seeds = seeds_line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    almanac.seeds_part2 = parse_seeds_part2(&almanac.seeds);

    lines.next();
    lines.next();
    almanac.seed_to_soil = parse_map(&mut lines);

    lines.next();
    almanac.soil_to_fertilizer = parse_map(&mut lines);

    lines.next();
    almanac.fertilizer_to_water = parse_map(&mut lines);

    lines.next();
    almanac.water_to_light = parse_map(&mut lines);

    lines.next();
    almanac.light_to_temperature = parse_map(&mut lines);

    lines.next();
    almanac.temperature_to_humidity = parse_map(&mut lines);

    lines.next();
    almanac.humidity_to_location = parse_map(&mut lines);

    almanac
}

fn parse_map(lines: &mut std::str::Lines) -> Vec<Mapping> {
    let mut list: Vec<Mapping> = Vec::new();

    while let Some(line) = lines.next() {
        let numbers: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 3 {
            list.push(Mapping {
                source: numbers[1],
                dest: numbers[0],
                len: numbers[2],
            });
        } else {
            break;
        }
    }
    list
}

fn transform_source_to_dest(source: usize, mapping: &Vec<Mapping>) -> usize {
    for map in mapping {
        if source >= map.source && source < map.source + map.len {
            return (source - map.source) + map.dest;
        }
    }
    source
}

fn map_seed_to_location(seed: usize, almanac: &Almanac) -> usize {
    let value = transform_source_to_dest(seed, &almanac.seed_to_soil);
    let value = transform_source_to_dest(value, &almanac.soil_to_fertilizer);
    let value = transform_source_to_dest(value, &almanac.fertilizer_to_water);
    let value = transform_source_to_dest(value, &almanac.water_to_light);
    let value = transform_source_to_dest(value, &almanac.light_to_temperature);
    let value = transform_source_to_dest(value, &almanac.temperature_to_humidity);
    transform_source_to_dest(value, &almanac.humidity_to_location)
}

fn map_seed2_range_to_min_location(seeds_part2: &Mapping, almanac: &Almanac) -> usize {
    let mut min_loc: usize = usize::MAX;
    for seed_val in seeds_part2.source..seeds_part2.source + seeds_part2.len {
        let val = map_seed_to_location(seed_val, almanac);
        min_loc = usize::min(min_loc, val);
    }
    println!(
        "Seed block: {} --> {}, Min Location is {}",
        seeds_part2.source,
        seeds_part2.source + seeds_part2.len,
        min_loc
    );
    min_loc
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("../../input/input05.txt");
    let almanac = parse_input(input);
    let sol1 = solve_part1(&almanac);
    let sol2 = solve_part2(&almanac);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, almanac))
        .min()
        .unwrap()
}

fn solve_part2(almanac: &Almanac) -> usize {
    almanac
        .seeds_part2
        .par_iter()
        .map(|seed| map_seed2_range_to_min_location(seed, almanac))
        .reduce(|| usize::MAX, |a, b| usize::min(a, b))
}

#[test]
fn test_solve_part1() {
    let input = include_str!("../../input/input05_test.txt");
    let almanac = parse_input(input);
    assert_eq!(35, solve_part1(&almanac));
}

#[test]
fn test_solve_part1_real() {
    let input = include_str!("../../input/input05.txt");
    let almanac = parse_input(input);
    assert_eq!(486613012, solve_part1(&almanac));
}

#[test]
fn test_solve_part2() {
    let input = include_str!("../../input/input05_test.txt");
    let almanac = parse_input(input);
    assert_eq!(46, solve_part2(&almanac));
}

#[test]
fn test_seed_to_soil() {
    let input = include_str!("../../input/input05_test.txt");
    let almanac = parse_input(input);
    assert_eq!(10, transform_source_to_dest(10, &almanac.seed_to_soil));

    assert_eq!(57, transform_source_to_dest(55, &almanac.seed_to_soil));
    assert_eq!(81, transform_source_to_dest(79, &almanac.seed_to_soil));
}

#[test]
fn test_seed_to_location() {
    let input = include_str!("../../input/input05_test.txt");
    let almanac = parse_input(input);
    assert_eq!(82, map_seed_to_location(79, &almanac));
    assert_eq!(43, map_seed_to_location(14, &almanac));
    assert_eq!(86, map_seed_to_location(55, &almanac));
    assert_eq!(35, map_seed_to_location(13, &almanac));
}
