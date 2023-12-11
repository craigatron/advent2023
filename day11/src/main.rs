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

fn parse_grid(file: &str, expand_count: u32) -> Vec<(u32, u32)> {
    let mut expand_cols: Vec<bool> = vec![];
    let mut galaxies: Vec<(u32, u32)> = vec![];
    let mut row_index: u32 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        if expand_cols.is_empty() {
            for _ in 0..line.len() {
                expand_cols.push(true);
            }
        }
        let row_galaxies: Vec<usize> = line.match_indices('#').map(|m| m.0).collect();
        if row_galaxies.is_empty() {
            row_index += expand_count;
        } else {
            for galaxy_col in row_galaxies {
                expand_cols[galaxy_col] = false;
                galaxies.push((row_index, galaxy_col as u32));
            }
        }
        row_index += 1;
    }
    let expand_col_i: Vec<usize> = expand_cols
        .iter()
        .enumerate()
        .filter(|&(_, expand)| *expand)
        .map(|(i, _)| i)
        .collect();
    for i in 0..galaxies.len() {
        let (gr, gc) = galaxies[i];
        let new_gc =
            gc + (expand_count * expand_col_i.iter().filter(|&i| *i < gc as usize).count() as u32);
        galaxies[i] = (gr, new_gc);
    }
    galaxies
}

fn manhattan_distance(galaxies: Vec<(u32, u32)>) -> u64 {
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        let gal1 = galaxies[i];
        for j in i..galaxies.len() {
            let gal2 = galaxies[j];
            // that's manhattan distance baby
            sum += (gal1.0 as i32 - gal2.0 as i32).abs() as u64
                + (gal1.1 as i32 - gal2.1 as i32).abs() as u64;
        }
    }
    sum
}

fn part1(file: &str) -> u64 {
    let galaxies = parse_grid(file, 1);
    manhattan_distance(galaxies)
}

fn part2(file: &str) -> u64 {
    let galaxies = parse_grid(file, 999999);
    manhattan_distance(galaxies)
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
