use std::collections::HashSet;
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

fn parse(file: &str) -> Vec<Vec<Vec<char>>> {
    let mut all_grids: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current_grid: Vec<Vec<char>> = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        if line == "" {
            all_grids.push(current_grid);
            current_grid = Vec::new();
        } else {
            current_grid.push(line.chars().collect());
        }
    }
    all_grids.push(current_grid);
    all_grids
}

fn reflect_indices(vec: &Vec<char>) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    // line is between elements i and i + 1
    for reflect_index in 0..vec.len() - 1 {
        let first_chunk_len = reflect_index + 1;
        let last_chunk_len = vec.len() - reflect_index - 1;
        let first_chunk: Vec<char>;
        let mut last_chunk: Vec<char>;
        if first_chunk_len > last_chunk_len {
            first_chunk = vec[reflect_index - last_chunk_len + 1..reflect_index + 1].to_vec();
            last_chunk = vec[reflect_index + 1..vec.len()].to_vec();
        } else if first_chunk_len < last_chunk_len {
            first_chunk = vec[0..reflect_index + 1].to_vec();
            last_chunk = vec[reflect_index + 1..reflect_index + 1 + first_chunk_len].to_vec();
        } else {
            first_chunk = vec[0..reflect_index + 1].to_vec();
            last_chunk = vec[reflect_index + 1..vec.len()].to_vec();
        }
        last_chunk.reverse();
        if first_chunk == last_chunk {
            indices.push(reflect_index);
        }
    }
    indices
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Reflection {
    row: Option<usize>,
    col: Option<usize>,
}

fn find_reflections(grid: &Vec<Vec<char>>) -> Vec<Reflection> {
    let mut reflections = Vec::new();
    let mut common_rows: HashSet<usize> = HashSet::from_iter(0..grid[0].len() - 1);
    for r in grid {
        let reflect_rows = HashSet::from_iter(reflect_indices(&r));
        common_rows = HashSet::from_iter(common_rows.intersection(&reflect_rows).map(|&r| r));
        if common_rows.is_empty() {
            break;
        }
    }
    for r in common_rows {
        reflections.push(Reflection {
            row: Some(r),
            col: None,
        })
    }

    // ok let's do columns then
    let mut common_cols: HashSet<usize> = HashSet::from_iter(0..grid.len() - 1);
    for c in 0..grid[0].len() {
        let column: Vec<char> = grid.iter().map(|r| r[c]).collect();
        let reflect_cols: HashSet<usize> = HashSet::from_iter(reflect_indices(&column));
        common_cols = HashSet::from_iter(common_cols.intersection(&reflect_cols).map(|&c| c));
    }
    for c in common_cols {
        reflections.push(Reflection {
            row: None,
            col: Some(c),
        })
    }
    reflections
}

fn part1(file: &str) -> u32 {
    let grids = parse(file);
    let mut sum: u32 = 0;
    for grid in grids {
        let reflections = find_reflections(&grid);
        let reflection = reflections.iter().next().unwrap();
        if reflection.row.is_some() {
            sum += reflection.row.unwrap() as u32 + 1;
        } else {
            sum += (reflection.col.unwrap() as u32 + 1) * 100;
        }
    }
    sum
}

fn part2(file: &str) -> u32 {
    let grids = parse(file);
    let mut sum = 0;
    'grid: for grid in grids {
        let original_reflection = *find_reflections(&grid).iter().next().unwrap();
        for smudge_r in 0..grid.len() {
            for smudge_c in 0..grid[0].len() {
                let mut new_grid = grid.clone();
                if new_grid[smudge_r][smudge_c] == '.' {
                    new_grid[smudge_r][smudge_c] = '#';
                } else {
                    new_grid[smudge_r][smudge_c] = '.';
                }
                let smudge_reflections = find_reflections(&new_grid);
                for sr in smudge_reflections {
                    if sr != original_reflection {
                        if sr.row.is_some() {
                            sum += sr.row.unwrap() as u32 + 1;
                        } else {
                            sum += (sr.col.unwrap() as u32 + 1) * 100;
                        }
                        continue 'grid;
                    }
                }
            }
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
