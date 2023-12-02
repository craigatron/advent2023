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

fn part1(file: &str) -> u16 {
    const RED: u8 = 12;
    const GREEN: u8 = 13;
    const BLUE: u8 = 14;

    let mut sum: u16 = 0;
    'game: for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts = line.split(": ").collect::<Vec<&str>>();
        for draw in parts[1].split("; ") {
            for cube in draw.split(", ") {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let cube_count = cube_parts[0].parse::<u8>().unwrap();
                let cube_color = cube_parts[1];
                if (cube_color == "red" && cube_count > RED)
                    || (cube_color == "blue" && cube_count > BLUE)
                    || (cube_color == "green" && cube_count > GREEN)
                {
                    continue 'game;
                }
            }
        }
        sum += parts[0].split(" ").last().unwrap().parse::<u16>().unwrap();
    }
    sum
}

fn part2(file: &str) -> u32 {
    let mut sum: u32 = 0;
    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts = line.split(": ").collect::<Vec<&str>>();
        let mut max_red: u32 = 0;
        let mut max_blue: u32 = 0;
        let mut max_green: u32 = 0;
        for draw in parts[1].split("; ") {
            for cube in draw.split(", ") {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let cube_count = cube_parts[0].parse::<u32>().unwrap();
                let cube_color = cube_parts[1];
                if cube_color == "red" && cube_count > max_red {
                    max_red = cube_count;
                } else if cube_color == "green" && cube_count > max_green {
                    max_green = cube_count;
                } else if cube_color == "blue" && cube_count > max_blue {
                    max_blue = cube_count;
                }
            }
        }
        sum += max_blue * max_green * max_red;
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
