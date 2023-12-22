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

fn parse(file: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut r = 0;
    let mut grid = Vec::new();
    let mut start = (usize::MAX, usize::MAX);
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let mut row = Vec::new();

        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                row.push(true);
                start = (r, i);
            } else {
                row.push(c == '.');
            }
        }
        grid.push(row);
        r += 1;
    }
    (grid, start)
}

fn part1(file: &str) -> u32 {
    let (grid, start) = parse(file);

    let num_steps = 64;
    let mut seen: HashSet<(usize, usize, u32)> = HashSet::new();
    let mut nodes = VecDeque::from([(start.0, start.1, 0)]);
    let mut final_cells: HashSet<(usize, usize)> = HashSet::new();
    while !nodes.is_empty() {
        let (r, c, current_steps) = nodes.pop_front().unwrap();
        if current_steps == num_steps {
            final_cells.insert((r, c));
            continue;
        }
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_r = r as i32 + dir.0;
            let next_c = c as i32 + dir.1;
            if next_r >= 0
                && next_r < grid.len() as i32
                && next_c >= 0
                && next_c < grid[0].len() as i32
                && grid[next_r as usize][next_c as usize]
            {
                let next_val = (next_r as usize, next_c as usize, current_steps + 1);
                if !seen.contains(&next_val) {
                    seen.insert(next_val);
                    nodes.push_back(next_val);
                }
            }
        }
    }
    final_cells.len() as u32
}

fn part2(file: &str) -> u32 {
    let (grid, start) = parse(file);

    // this isn't a real solution - I tweaked this var to find the # of solutions
    // for 1, 2, and 3 grids over and then dumped that in Wolfram Alpha to find
    // the quadratic formula and solved for 202301 grids
    let num_steps = 458;
    let mut seen: HashSet<((usize, usize), (i32, i32), u32)> = HashSet::new();
    let mut nodes = VecDeque::from([((start.0, start.1), (0, 0), 0)]);
    let mut final_cells: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
    while !nodes.is_empty() {
        let ((r, c), (grid_r, grid_c), current_steps) = nodes.pop_front().unwrap();
        if current_steps == num_steps {
            final_cells.insert(((r, c), (grid_r, grid_c)));
            continue;
        }
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let mut next_r = r as i32 + dir.0;
            let mut next_c = c as i32 + dir.1;
            let mut new_grid_r = grid_r;
            let mut new_grid_c = grid_c;
            if next_r == grid.len() as i32 {
                next_r = 0;
                new_grid_r += 1;
            } else if next_r == -1 {
                next_r = grid.len() as i32 - 1;
                new_grid_r -= 1;
            } else if next_c == grid[0].len() as i32 {
                next_c = 0;
                new_grid_c += 1;
            } else if next_c == -1 {
                next_c = grid[0].len() as i32 - 1;
                new_grid_c -= 1;
            }

            if grid[next_r as usize][next_c as usize] {
                let next_val = (
                    (next_r as usize, next_c as usize),
                    (new_grid_r, new_grid_c),
                    current_steps + 1,
                );
                if !seen.contains(&next_val) {
                    seen.insert(next_val);
                    nodes.push_back(next_val);
                }
            }
        }
    }
    final_cells.len() as u32
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
