use std::cmp;
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

#[derive(Clone, Copy, Debug)]
struct Brick {
    start_x: u32,
    start_y: u32,
    start_z: u32,
    end_x: u32,
    end_y: u32,
    end_z: u32,
}

impl Brick {
    fn bottom_points(&self) -> Vec<(u32, u32, u32)> {
        let mut v = vec![];
        let min_z = cmp::min(self.start_z, self.end_z);
        for x in self.start_x..self.end_x + 1 {
            for y in self.start_y..self.end_y + 1 {
                v.push((x, y, min_z));
            }
        }
        v
    }

    fn top_points(&self) -> Vec<(u32, u32, u32)> {
        let mut v = vec![];
        let max_z = cmp::max(self.start_z, self.end_z);
        for x in self.start_x..self.end_x + 1 {
            for y in self.start_y..self.end_y + 1 {
                v.push((x, y, max_z));
            }
        }
        v
    }
}

fn parse(file: &str) -> Vec<Brick> {
    let mut v = vec![];

    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let mut points = line.split("~");
        let start_points: Vec<u32> = points
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let end_points: Vec<u32> = points
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        v.push(Brick {
            start_x: start_points[0],
            start_y: start_points[1],
            start_z: start_points[2],
            end_x: end_points[0],
            end_y: end_points[1],
            end_z: end_points[2],
        });
    }

    v
}

fn drop_bricks(mut bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
    let mut stuck_bricks: Vec<Brick> = vec![];

    let mut stuck_points: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for x in 0..10 {
        for y in 0..10 {
            stuck_points.insert((x, y), vec![0]);
        }
    }

    // find any already stuck bricks
    loop {
        let mut indices_to_remove: Vec<usize> = vec![];
        'brick: for i in 0..bricks.len() {
            let brick = bricks[i];
            let bottom_points = brick.bottom_points();
            for (x, y, z) in bottom_points.iter() {
                let current_stuck_points = stuck_points.get(&(*x, *y)).unwrap();

                if current_stuck_points.contains(&(z - 1)) {
                    // brick or ground below, this brick is stuck yo
                    for (x2, y2, z2) in brick.top_points() {
                        if let Some(stuck_zs) = stuck_points.get_mut(&(x2, y2)) {
                            stuck_zs.push(z2);
                            stuck_zs.sort();
                        } else {
                            stuck_points.insert((x2, y2), vec![0, z2]);
                        }
                    }
                    stuck_bricks.push(brick);
                    indices_to_remove.insert(0, i);
                    continue 'brick;
                }
            }
        }
        if indices_to_remove.is_empty() {
            break;
        } else {
            for i in indices_to_remove {
                bricks.swap_remove(i);
            }
        }
    }

    let unstuck = bricks.len();

    while !bricks.is_empty() {
        // sort so the lowest block is at the top of the vector
        bricks.sort_by(|a, b| cmp::min(a.start_z, a.end_z).cmp(cmp::min(&b.start_z, &b.end_z)));
        let mut brick = bricks.swap_remove(0);
        // brick isn't stuck, find the max highest point value underneath it
        let mut max_z = 0;
        for (x, y, z) in brick.bottom_points().iter() {
            let stuck_zs = stuck_points.get(&(*x, *y)).unwrap();
            let mut high_z = 0;
            for stuck_z in stuck_zs {
                if stuck_z > z {
                    break;
                }
                high_z = *stuck_z;
            }

            if high_z > max_z {
                max_z = high_z;
            }
        }

        let drop_space = brick.start_z - max_z - 1;

        brick.start_z -= drop_space;
        brick.end_z -= drop_space;
        for (x, y, z) in brick.top_points() {
            let stuck_zs = stuck_points.get_mut(&(x, y)).unwrap();
            stuck_zs.push(z);
            stuck_zs.sort();
        }
        stuck_bricks.push(brick);
    }
    (stuck_bricks, unstuck)
}

// will bricks drop if you dissolve the brick at index?
fn will_drop(bricks: &Vec<Brick>, index: usize) -> bool {
    let potential_points: Vec<(u32, u32, u32)> = bricks[index]
        .top_points()
        .iter()
        .map(|&(x, y, z)| (x, y, z + 1))
        .collect();
    let potential_drops: Vec<&Brick> = bricks
        .iter()
        .enumerate()
        .filter(|&(j, b)| {
            j != index
                && b.bottom_points()
                    .iter()
                    .any(|&p| potential_points.contains(&p))
        })
        .map(|(_, b)| b)
        .collect();

    let mut will_drop = false;
    for b in potential_drops {
        let supports: Vec<(u32, u32, u32)> = b
            .bottom_points()
            .iter()
            .map(|&(x, y, z)| (x, y, z - 1))
            .collect();
        if !bricks
            .iter()
            .enumerate()
            .any(|(j, b)| index != j && b.top_points().iter().any(|&p| supports.contains(&p)))
        {
            will_drop = true;
            break;
        }
    }

    will_drop
}

fn part1(file: &str) -> u32 {
    let bricks = parse(file);

    let (stuck_bricks, _) = drop_bricks(bricks);

    let mut count = 0;

    for i in 0..stuck_bricks.len() {
        if !will_drop(&stuck_bricks, i) {
            count += 1;
        }
    }

    count
}

fn part2(file: &str) -> u32 {
    let bricks = parse(file);
    let (stuck_bricks, _) = drop_bricks(bricks);

    // I am too tired to optimize this so whatever
    let mut sum: u32 = 0;
    for i in 0..stuck_bricks.len() {
        if will_drop(&stuck_bricks, i) {
            let mut new_bricks = stuck_bricks.clone();
            new_bricks.swap_remove(i);
            let (_, dropped) = drop_bricks(new_bricks);
            sum += dropped as u32;
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
