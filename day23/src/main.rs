use std::collections::{HashMap, HashSet, VecDeque};
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

fn parse(file: &str) -> Vec<Vec<char>> {
    read_lines(file)
        .unwrap()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect()
}

fn find_splits(grid: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize), u32)> {
    let mut v = vec![];

    // first pair is the node to start at, second pair is the node we came from
    let mut nodes: VecDeque<((usize, usize), (usize, usize))> = VecDeque::from([((1, 1), (0, 1))]);
    let exit = (grid.len() - 1, grid[0].len() - 2);
    let mut seen_splits: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    while !nodes.is_empty() {
        let node = nodes.pop_front().unwrap();
        // walk the grid until we find a split in the path
        let mut last_coord = node.1;
        let mut coord = node.0;
        let mut steps = 0;
        loop {
            let (r, c) = coord;
            if coord == exit {
                v.push((node.1, coord, steps + 1));
                break;
            }
            let next_steps: Vec<(usize, usize)> = [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .iter()
                .filter(|&&p| p != last_coord && grid[p.0][p.1] != '#')
                .map(|&c| c)
                .collect();
            if next_steps.len() > 1 {
                // found a split, stop traversing this path
                if !seen_splits.contains(&(node.1, coord))
                    && !seen_splits.contains(&(coord, node.1))
                {
                    v.push((node.1, coord, steps + 1));
                    for n in next_steps {
                        nodes.push_back((n, coord));
                    }
                    seen_splits.insert((node.1, coord));
                    seen_splits.insert((coord, node.1));
                }
                break;
            } else {
                last_coord = coord;
                coord = next_steps[0];
                steps += 1;
            }
        }
    }

    v
}

fn longest_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    exit: (usize, usize),
    ignore_slopes: bool,
) -> u32 {
    let mut max = 0;

    let mut nodes: VecDeque<(usize, usize, HashSet<(usize, usize)>)> =
        VecDeque::from([(start.0, start.1, HashSet::from([start]))]);
    while !nodes.is_empty() {
        let (start_r, start_c, visited) = nodes.pop_front().unwrap();
        if (start_r, start_c) == exit {
            if visited.len() > max {
                max = visited.len();
            }
            continue;
        }
        let next: Vec<(usize, usize)>;
        if start_r == 0 {
            next = vec![(start_r + 1, start_c)];
        } else if ignore_slopes {
            next = vec![
                (start_r + 1, start_c),
                (start_r - 1, start_c),
                (start_r, start_c + 1),
                (start_r, start_c - 1),
            ];
        } else {
            next = match grid[start_r][start_c] {
                '>' => vec![(start_r, start_c + 1)],
                '<' => vec![(start_r, start_c - 1)],
                '^' => vec![(start_r - 1, start_c)],
                'v' => vec![(start_r + 1, start_c)],
                '.' => vec![
                    (start_r + 1, start_c),
                    (start_r - 1, start_c),
                    (start_r, start_c + 1),
                    (start_r, start_c - 1),
                ],
                _ => panic!("unexpected char"),
            };
        }

        for n in next {
            if !visited.contains(&n) && grid[n.0][n.1] != '#' {
                let mut new_visited = visited.clone();
                new_visited.insert(n);
                nodes.push_back((n.0, n.1, new_visited));
            }
        }
    }

    max as u32 - 1
}

fn part1(file: &str) -> u32 {
    let grid = parse(file);
    let start_col = grid[0].iter().position(|&c| c == '.').unwrap();
    let end_col = grid[grid.len() - 1].iter().position(|&c| c == '.').unwrap();
    longest_path(&grid, (0, start_col), (grid.len() - 1, end_col), false)
}

fn longest_path_splits(
    grid: &Vec<Vec<char>>,
    splits: &Vec<((usize, usize), (usize, usize), u32)>,
) -> u32 {
    let exit = (grid.len() - 1, grid[0].len() - 2);
    let mut split_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for i in 0..splits.len() {
        if !split_map.contains_key(&splits[i].0) {
            split_map.insert(splits[i].0, vec![]);
        }
        if !split_map.contains_key(&splits[i].1) {
            split_map.insert(splits[i].1, vec![]);
        }
        split_map.get_mut(&splits[i].0).unwrap().push(i);
        split_map.get_mut(&splits[i].1).unwrap().push(i);
    }
    let mut nodes: VecDeque<(usize, usize, HashSet<usize>, HashSet<(usize, usize)>)> =
        VecDeque::from([(0, 1, HashSet::new(), HashSet::from([(0, 1)]))]);
    let mut max = 0;
    while !nodes.is_empty() {
        let (r, c, visited, visited_points) = nodes.pop_front().unwrap();
        for split_index in split_map.get(&(r, c)).unwrap() {
            if visited.contains(split_index) {
                continue;
            }
            let (split_start, split_end, _) = splits[*split_index];
            let mut new_visited = visited.clone();
            new_visited.insert(*split_index);
            if split_end == exit {
                // found the exit, count up the lengths
                let sum: u32 = new_visited.iter().map(|i| splits[*i].2).sum();
                if sum > max {
                    max = sum;
                }
            } else {
                let next_index = if r == split_start.0 && c == split_start.1 {
                    split_end
                } else {
                    split_start
                };
                if visited_points.contains(&next_index) {
                    continue;
                }
                let mut new_visited_points = visited_points.clone();
                new_visited_points.insert(next_index);
                nodes.push_back((next_index.0, next_index.1, new_visited, new_visited_points));
            }
        }
    }
    max
}

fn part2(file: &str) -> u32 {
    let grid = parse(file);
    let splits = find_splits(&grid);
    longest_path_splits(&grid, &splits)
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
