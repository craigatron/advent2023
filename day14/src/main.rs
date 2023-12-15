use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let start1 = Instant::now();
    let p1 = part1(file);
    let duration1 = start1.elapsed();
    println!("part1: {}, time {:?}", p1, duration1);

    let start2 = Instant::now();
    let p2 = part2(file);
    let duration2 = start2.elapsed();
    println!("part2: {}, time {:?}", p2, duration2);
}

struct Grid {
    grid: Vec<Vec<char>>,
    rocks: Vec<(usize, usize)>,
}

#[derive(PartialEq, Eq)]
struct Direction {
    dr: i32,
    dc: i32,
}

const NORTH: Direction = Direction { dr: -1, dc: 0 };
const SOUTH: Direction = Direction { dr: 1, dc: 0 };
const EAST: Direction = Direction { dr: 0, dc: 1 };
const WEST: Direction = Direction { dr: 0, dc: -1 };

fn parse(file: &str) -> Grid {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut rocks: Vec<(usize, usize)> = Vec::new();
    let mut r: usize = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let mut row: Vec<char> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'O' {
                rocks.push((r, i));
            }
        }
        grid.push(row);
        r += 1;
    }
    Grid {
        grid: grid,
        rocks: rocks,
    }
}

fn roll(grid: &mut Grid, direction: &Direction) {
    let mut blocked: Vec<bool> = vec![false; grid.rocks.len()];
    loop {
        let mut changes_made = false;
        for i in 0..grid.rocks.len() {
            let (rock_r, rock_c) = grid.rocks[i];
            if blocked[i] {
                continue;
            }
            // is rock at edge?
            if (direction == &NORTH && rock_r == 0)
                || (direction == &SOUTH && rock_r == grid.grid.len() - 1)
                || (direction == &EAST && rock_c == grid.grid[0].len() - 1)
                || (direction == &WEST && rock_c == 0)
            {
                blocked[i] = true;
                continue;
            }
            let next_coords = (
                (rock_r as i32 + direction.dr) as usize,
                (rock_c as i32 + direction.dc) as usize,
            );
            let next_char = grid.grid[next_coords.0][next_coords.1];
            if next_char == '#' {
                blocked[i] = true;
                continue;
            }

            if next_char == '.' {
                grid.grid[rock_r][rock_c] = '.';
                grid.grid[next_coords.0][next_coords.1] = 'O';
                grid.rocks[i] = (next_coords.0, next_coords.1);
                changes_made = true;
            }
        }
        if !changes_made {
            break;
        }
    }
}

fn part1(file: &str) -> u32 {
    let mut grid = parse(file);
    roll(&mut grid, &NORTH);

    grid.rocks
        .iter()
        .map(|(r, _)| (grid.grid.len() - r) as u32)
        .sum()
}

fn part2(file: &str) -> u32 {
    let mut grid = parse(file);
    let dir_cycle = [NORTH, WEST, SOUTH, EAST];

    let mut hash: HashMap<Vec<(usize, usize)>, u32> = HashMap::new();
    let mut loads: Vec<u32> = Vec::new();

    let mut i = 1;
    let ret_load: u32;
    loop {
        for dir in dir_cycle.iter() {
            roll(&mut grid, dir);
        }
        let mut new_rocks = grid.rocks.clone();
        new_rocks.sort();
        if let Some(start_i) = hash.get(&new_rocks) {
            let load = ((1000000000 - i) % (i - start_i)) + start_i - 1;
            ret_load = loads[load as usize];
            break;
        } else {
            let load: u32 = new_rocks
                .iter()
                .map(|(r, _)| (grid.grid.len() - r) as u32)
                .sum();
            hash.insert(new_rocks, i);
            loads.push(load);
        }
        i += 1;
    }
    ret_load
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
