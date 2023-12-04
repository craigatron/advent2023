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

fn scan_grid(file: &str) -> (HashMap<(i16, i16), Vec<u32>>, Vec<(i16, i16, u32)>) {
    let mut parts: HashMap<(i16, i16), Vec<u32>> = HashMap::new();
    let mut part_numbers: Vec<(i16, i16, u32)> = Vec::new();
    let mut row: i16 = 0;
    for l in read_lines(file).unwrap() {
        let mut num_builder = String::new();
        let mut num_col_start: i16 = i16::MIN;
        let line = l.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if num_builder == "" {
                    num_col_start = col as i16;
                }
                num_builder.push(c);
                if col == line.len() - 1 {
                    part_numbers.push((row, num_col_start, num_builder.parse::<u32>().unwrap()));
                }
            } else {
                if num_builder != "" {
                    part_numbers.push((row, num_col_start, num_builder.parse::<u32>().unwrap()));
                    num_col_start = i16::MIN;
                    num_builder = String::new();
                }

                if c != '.' {
                    parts.insert((row, col as i16), Vec::new());
                }
            }
        }
        row += 1;
    }

    (parts, part_numbers)
}

fn possible_adj(r: i16, c: i16, n: u32) -> Vec<(i16, i16)> {
    let len = n.to_string().len() as i16;
    let mut checks: Vec<(i16, i16)> = Vec::new();
    for col in c - 1..c + len + 1 {
        checks.push((r - 1, col));
        checks.push((r + 1, col));
    }
    checks.push((r, c - 1));
    checks.push((r, c + len));

    checks
}

fn part1(file: &str) -> u32 {
    let (parts, part_numbers) = scan_grid(file);

    let mut sum: u32 = 0;
    'part: for (r, c, n) in part_numbers {
        for check in possible_adj(r, c, n) {
            if parts.contains_key(&check) {
                sum += n;
                continue 'part;
            }
        }
    }
    sum
}

fn part2(file: &str) -> u32 {
    let (mut parts, part_numbers) = scan_grid(file);

    for (r, c, n) in part_numbers {
        for check in possible_adj(r, c, n) {
            if parts.contains_key(&check) {
                parts.get_mut(&check).unwrap().push(n);
            }
        }
    }

    let mut sum: u32 = 0;
    for v in parts.values() {
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }

    sum
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
