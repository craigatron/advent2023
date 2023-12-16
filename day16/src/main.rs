use std::collections::{HashSet, VecDeque};
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

fn parse(file: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in read_lines(file).unwrap() {
        grid.push(l.unwrap().chars().collect());
    }
    grid
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn next_tile(coord: (usize, usize), d: Direction, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let next_tile: (isize, isize) = match d {
        Direction::Right => (coord.0 as isize, coord.1 as isize + 1),
        Direction::Down => (coord.0 as isize + 1, coord.1 as isize),
        Direction::Left => (coord.0 as isize, coord.1 as isize - 1),
        Direction::Up => (coord.0 as isize - 1, coord.1 as isize),
    };
    if next_tile.0 < 0
        || next_tile.0 >= grid.len() as isize
        || next_tile.1 < 0
        || next_tile.1 >= grid[0].len() as isize
    {
        // hit a wall, nowhere to go
        None
    } else {
        Some((next_tile.0 as usize, next_tile.1 as usize))
    }
}

fn next_tiles(
    coord: (usize, usize),
    d: Direction,
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize, Direction)> {
    let mut tiles: Vec<(usize, usize, Direction)> = Vec::new();

    let char = grid[coord.0][coord.1];
    if char == '.' {
        if let Some(t) = next_tile(coord, d, grid) {
            tiles.push((t.0, t.1, d));
        }
    } else if char == '/' || char == '\\' {
        let next_direction = match (d, char) {
            (Direction::Right, '/') => Direction::Up,
            (Direction::Down, '/') => Direction::Left,
            (Direction::Left, '/') => Direction::Down,
            (Direction::Up, '/') => Direction::Right,
            (Direction::Right, '\\') => Direction::Down,
            (Direction::Down, '\\') => Direction::Right,
            (Direction::Left, '\\') => Direction::Up,
            (Direction::Up, '\\') => Direction::Left,
            _ => panic!("unexpected char"),
        };
        if let Some(t) = next_tile(coord, next_direction, grid) {
            tiles.push((t.0, t.1, next_direction));
        }
    } else if char == '|' || char == '-' {
        let next_directions = match (d, char) {
            (Direction::Right, '|') => vec![Direction::Up, Direction::Down],
            (Direction::Down, '|') => vec![Direction::Down],
            (Direction::Left, '|') => vec![Direction::Up, Direction::Down],
            (Direction::Up, '|') => vec![Direction::Up],
            (Direction::Right, '-') => vec![Direction::Right],
            (Direction::Down, '-') => vec![Direction::Left, Direction::Right],
            (Direction::Left, '-') => vec![Direction::Left],
            (Direction::Up, '-') => vec![Direction::Left, Direction::Right],
            _ => panic!("unexpected char"),
        };
        for d in next_directions {
            if let Some(t) = next_tile(coord, d, grid) {
                tiles.push((t.0, t.1, d));
            }
        }
    }

    tiles
}

fn count_energized_tiles(grid: &Vec<Vec<char>>, start: (usize, usize), d: Direction) -> u32 {
    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut seen_tiles: HashSet<(usize, usize, Direction)> = HashSet::new();

    let mut beams: VecDeque<(usize, usize, Direction)> = VecDeque::from([(start.0, start.1, d)]);

    while !beams.is_empty() {
        let beam = beams.pop_front().unwrap();
        if seen_tiles.contains(&beam) {
            continue;
        }
        seen_tiles.insert(beam);
        energized_tiles.insert((beam.0, beam.1));

        let next_tiles = next_tiles((beam.0, beam.1), beam.2, &grid);
        for n in next_tiles {
            beams.push_back(n);
        }
    }

    energized_tiles.len() as u32
}

fn part1(file: &str) -> u32 {
    let grid = parse(file);
    count_energized_tiles(&grid, (0, 0), Direction::Right)
}

fn part2(file: &str) -> u32 {
    let grid = parse(file);

    // yeah I could do something smarter here with memoizing results across runs
    // but I've gotta leave in 20 minutes so I'm happy with this
    let mut starts: Vec<(usize, usize, Direction)> = Vec::new();
    for r in 0..grid.len() {
        starts.push((r, 0, Direction::Right));
        starts.push((r, grid[0].len() - 1, Direction::Left));
    }
    for c in 0..grid[0].len() {
        starts.push((0, c, Direction::Down));
        starts.push((grid.len() - 1, c, Direction::Up));
    }

    let mut max = 0;
    for start in starts {
        let count = count_energized_tiles(&grid, (start.0, start.1), start.2);
        if count > max {
            max = count;
        }
    }

    max
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
