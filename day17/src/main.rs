use std::collections::{HashMap, VecDeque};
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

fn parse(file: &str) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    for l in read_lines(file).unwrap() {
        v.push(l.unwrap().chars().map(|c| c as u8 - '0' as u8).collect());
    }
    v
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    col: usize,
    next_direction: Direction,
}

fn min_loss(grid: &Vec<Vec<u8>>, min_consecutive: u8, max_consecutive: u8) -> u32 {
    let mut min_loss = u32::MAX;

    let mut min_values: HashMap<Node, u32> = HashMap::new();

    let max_r = grid.len() - 1;
    let max_c = grid[0].len() - 1;

    let mut queue: VecDeque<(Node, u32)> = VecDeque::from([
        (
            Node {
                row: 0,
                col: 0,
                next_direction: Direction::Down,
            },
            0,
        ),
        (
            Node {
                row: 0,
                col: 0,
                next_direction: Direction::Right,
            },
            0,
        ),
    ]);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if current.0.row == max_r && current.0.col == max_c {
            if current.1 < min_loss {
                min_loss = current.1;
            }
            continue;
        }

        let mut new_heat_loss = current.1;
        let dir = current.0.next_direction;

        let mut new_r = current.0.row;
        let mut new_c = current.0.col;

        for i in 0..max_consecutive {
            if (dir == Direction::Up && new_r == 0)
                || (dir == Direction::Down && new_r == max_r)
                || (dir == Direction::Left && new_c == 0)
                || (dir == Direction::Right && new_c == max_c)
            {
                // hit a wall, can't go that way
                break;
            }
            match dir {
                Direction::Down => new_r += 1,
                Direction::Right => new_c += 1,
                Direction::Up => new_r -= 1,
                Direction::Left => new_c -= 1,
            }
            new_heat_loss += grid[new_r][new_c] as u32;
            if new_heat_loss > min_loss {
                break;
            }

            if i + 1 < min_consecutive {
                continue;
            }

            let next_directions = match dir {
                Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
                Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            };
            for next_d in next_directions {
                let n = Node {
                    row: new_r,
                    col: new_c,
                    next_direction: next_d,
                };
                let min_seen_value = min_values.get(&n).unwrap_or(&u32::MAX);
                if new_heat_loss < *min_seen_value {
                    min_values.insert(n, new_heat_loss);
                    queue.push_back((n, new_heat_loss));
                }
            }
        }
    }

    min_loss
}

fn part1(file: &str) -> u32 {
    let grid = parse(file);

    min_loss(&grid, 1, 3)
}

fn part2(file: &str) -> u32 {
    let grid = parse(file);

    min_loss(&grid, 4, 10)
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
