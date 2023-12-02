use regex::Regex;
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

fn first_last<'a>(mut i: impl Iterator<Item = &'a str>) -> (String, String) {
    let first = i.next().unwrap();
    let last: &str;
    if let Some(l) = i.last() {
        last = l;
    } else {
        last = first;
    }
    (first.to_string(), last.to_string())
}

fn part1(file: &str) -> u32 {
    let re = Regex::new(r"\d").unwrap();
    let mut sum: u32 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let matches = re.find_iter(&line).map(|m| m.as_str());
        let (first, last) = first_last(matches);

        sum += format!("{}{}", first, last).parse::<u32>().unwrap();
    }
    sum
}

fn str_to_num(n: &str, first: bool) -> u32 {
    match n {
        "oneight" => {
            if first {
                1
            } else {
                8
            }
        }
        "twone" => {
            if first {
                2
            } else {
                1
            }
        }
        "threeight" => {
            if first {
                3
            } else {
                8
            }
        }
        "fiveight" => {
            if first {
                5
            } else {
                8
            }
        }
        "sevenine" => {
            if first {
                7
            } else {
                9
            }
        }
        "eightwo" => {
            if first {
                8
            } else {
                2
            }
        }
        "eighthree" => {
            if first {
                8
            } else {
                3
            }
        }
        "nineight" => {
            if first {
                9
            } else {
                8
            }
        }
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("invalid number: {}", n),
    }
}

fn part2(file: &str) -> u32 {
    // lol there's gotta be a better way to capture overlapping regex matches than this
    let re = Regex::new(r"(\d|oneight|twone|threeight|fiveight|sevenine|eightwo|eighthree|nineight|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut sum: u32 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let matches = re.find_iter(&line).map(|m| m.as_str());
        let (first, last) = first_last(matches);
        let num = format!(
            "{}{}",
            str_to_num(first.as_str(), true),
            str_to_num(last.as_str(), false)
        )
        .parse::<u32>()
        .unwrap();
        sum += num
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
