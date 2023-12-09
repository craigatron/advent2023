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

fn parse_histories(file: &str) -> Vec<Vec<Vec<i32>>> {
    let mut all_histories: Vec<Vec<Vec<i32>>> = Vec::new();
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let vals: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut histories = vec![vals];
        while histories[histories.len() - 1].iter().any(|&v| v != 0) {
            let curr_last = &histories[histories.len() - 1];
            let mut new_last: Vec<i32> = Vec::new();
            for i in 1..curr_last.len() {
                new_last.push(curr_last[i] - curr_last[i - 1]);
            }
            histories.push(new_last);
        }
        all_histories.push(histories);
    }
    all_histories
}

fn part1(file: &str) -> i32 {
    let mut sum: i32 = 0;
    for mut histories in parse_histories(file) {
        let rows = histories.len();
        histories[rows - 1].push(0);
        for i in (0..rows - 1).rev() {
            let val = { histories[i + 1].last().unwrap() + histories[i].last().unwrap() };
            histories[i].push(val);
        }
        sum += histories[0].last().unwrap();
    }
    sum
}

fn part2(file: &str) -> i32 {
    let mut sum: i32 = 0;
    for mut histories in parse_histories(file) {
        let rows = histories.len();
        histories[rows - 1].insert(0, 0);
        for i in (0..rows - 1).rev() {
            let val = { histories[i][0] - histories[i + 1][0] };
            histories[i].insert(0, val);
        }
        sum += histories[0][0];
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
