use once_cell::sync::Lazy;
use regex::Regex;
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

static SEPARATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(:|\|)").expect("invalid regex"));

fn num_matched_numbers(line: &str) -> u32 {
    let parts: Vec<&str> = SEPARATOR.split(&line).into_iter().collect();
    let winning_numbers: HashSet<&str> = HashSet::from_iter(parts[2].split_whitespace());
    parts[1]
        .split_whitespace()
        .filter(|s| winning_numbers.contains(s))
        .count() as u32
}

fn part1(file: &str) -> u32 {
    let mut sum: u32 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let matched = num_matched_numbers(&line);
        if matched > 0 {
            sum += 2_u32.pow(matched - 1);
        }
    }
    sum
}

fn part2(file: &str) -> u32 {
    let mut num_cards: Vec<u32> = Vec::new();
    let mut card_num: u32 = 0;
    for l in read_lines(file).unwrap() {
        // add the initial card
        if card_num >= num_cards.len() as u32 {
            num_cards.push(1);
        } else {
            num_cards[card_num as usize] += 1;
        }
        let line = l.unwrap();
        let matched = num_matched_numbers(&line) as u32;
        for card_to_add in card_num + 1..card_num + matched + 1 {
            if card_to_add >= num_cards.len() as u32 {
                num_cards.push(num_cards[card_num as usize]);
            } else {
                num_cards[card_to_add as usize] += num_cards[card_num as usize];
            }
        }
        card_num += 1;
    }
    let slice = &num_cards[..card_num as usize];
    slice.into_iter().sum()
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
