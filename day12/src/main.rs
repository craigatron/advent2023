use std::collections::HashMap;
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

fn count_possible_solutions(
    chars: &Vec<char>,
    counts: &Vec<usize>,
    memo: &mut HashMap<(String, Vec<usize>), u64>,
) -> u64 {
    let key: (String, Vec<usize>) = (chars.iter().collect(), counts.clone());
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }

    let count;
    if counts.is_empty() && !chars.iter().any(|&c| c == '#') {
        count = 1;
    } else if counts.is_empty() && chars.iter().any(|&c| c == '#') {
        count = 0;
    } else if chars.is_empty() && !counts.is_empty() {
        count = 0;
    } else {
        if chars[0] == '.' {
            count = count_possible_solutions(&chars[1..].to_vec(), counts, memo);
        } else if chars[0] == '#' {
            let target_count = counts[0];
            if chars.len() >= target_count
                && chars[0..target_count].iter().all(|&c| c == '#' || c == '?')
                && (chars.len() == target_count || chars[target_count] != '#')
            {
                let next_chars = if chars.len() == target_count {
                    vec![]
                } else {
                    chars[target_count + 1..].to_vec()
                };
                count = count_possible_solutions(&next_chars, &counts[1..].to_vec(), memo)
            } else {
                count = 0;
            }
        } else {
            // chars[0] == '?'
            let mut v1 = chars.clone();
            v1[0] = '#';
            let mut v2 = chars.clone();
            v2[0] = '.';
            count = count_possible_solutions(&v1, counts, memo)
                + count_possible_solutions(&v2, counts, memo);
        }
    }
    memo.insert(key, count);
    count
}

fn part1(file: &str) -> u64 {
    let mut sum: u64 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = parts[0];
        let counts: Vec<usize> = parts[1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut memo: HashMap<(String, Vec<usize>), u64> = HashMap::new();

        sum += count_possible_solutions(&springs.chars().collect(), &counts, &mut memo);
    }
    sum
}

fn part2(file: &str) -> u64 {
    let mut sum: u64 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = vec![parts[0]; 5].join("?");
        let counts: Vec<usize> = vec![parts[1]; 5]
            .join(",")
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut memo: HashMap<(String, Vec<usize>), u64> = HashMap::new();

        sum += count_possible_solutions(&springs.chars().collect(), &counts, &mut memo) as u64;
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
