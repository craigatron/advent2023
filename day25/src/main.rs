use std::cmp;
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

#[derive(Debug)]
struct Graph {
    vertices: HashSet<String>,
    edges: Vec<Edge>,
    edges_by_vertex: HashMap<String, Vec<usize>>,
}

#[derive(Clone, Debug)]
struct Edge {
    vertices: HashSet<String>,
}

fn parse(file: &str) -> Graph {
    let mut vertices: HashSet<String> = HashSet::new();
    let mut edges: Vec<Edge> = vec![];
    let mut edges_by_vertex: HashMap<String, Vec<usize>> = HashMap::new();

    for l in read_lines(file).unwrap() {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(": ").collect();

        let v = parts[0].to_owned();
        if !edges_by_vertex.contains_key(&v) {
            edges_by_vertex.insert(v.clone(), vec![]);
        }
        vertices.insert(v.clone());
        for next in parts[1].split_whitespace() {
            let n = next.to_owned();
            vertices.insert(n.clone());
            if !edges_by_vertex.contains_key(&n) {
                edges_by_vertex.insert(n.clone(), vec![]);
            }
            let edge = Edge {
                vertices: HashSet::from([v.clone(), next.to_owned()]),
            };
            let edge_i = edges.len();
            edges_by_vertex.get_mut(&v).unwrap().push(edge_i);
            edges_by_vertex.get_mut(&n).unwrap().push(edge_i);
            edges.push(edge);
        }
    }

    Graph {
        vertices: vertices,
        edges: edges,
        edges_by_vertex: edges_by_vertex,
    }
}

fn bfs(graph: &Graph, start_v: String, ignore_edges: HashSet<usize>) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::from([start_v]);
    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();
        visited.insert(n.clone());
        let edges = graph.edges_by_vertex.get(&n).unwrap();
        for e in edges {
            if !ignore_edges.contains(e) {
                for v in graph.edges[*e].vertices.iter() {
                    if !visited.contains(v) {
                        queue.push_back(v.clone());
                    }
                }
            }
        }
    }
    visited
}

fn dfs(
    graph: &Graph,
    u: String,
    v: String,
    low: &mut HashMap<String, i32>,
    pre: &mut HashMap<String, i32>,
    cnt: &mut i32,
    ignore_edges: &HashSet<usize>,
) -> Option<usize> {
    *cnt += 1;
    pre.insert(v.clone(), *cnt);
    low.insert(v.clone(), *pre.get(&v).unwrap());

    for w in graph.edges_by_vertex.get(&v).unwrap() {
        if ignore_edges.contains(w) {
            continue;
        }
        let node = graph.edges[*w]
            .vertices
            .iter()
            .filter(|&x| *x != v)
            .next()
            .unwrap();
        if *pre.get(node).unwrap() == -1 {
            let maybe_bridge = dfs(graph, v.clone(), node.clone(), low, pre, cnt, ignore_edges);
            if maybe_bridge.is_some() {
                return maybe_bridge;
            }
            low.insert(
                v.clone(),
                cmp::min(*low.get(&v).unwrap(), *low.get(node).unwrap()),
            );
            if low.get(node).unwrap() == pre.get(node).unwrap() {
                return Some(*w);
            }
        } else if *node != u {
            low.insert(
                v.clone(),
                cmp::min(*low.get(&v).unwrap(), *pre.get(node).unwrap()),
            );
        }
    }

    None
}

fn cut_edges(graph: &Graph) -> HashSet<usize> {
    for i in 0..graph.edges.len() {
        println!("i: {}", i);
        for j in i + 1..graph.edges.len() {
            let mut low: HashMap<String, i32> =
                graph.vertices.iter().map(|v| (v.clone(), -1)).collect();
            let mut pre = low.clone();
            let mut cnt = 0;

            let ignore_edges = HashSet::from([i, j]);
            for v in graph.vertices.iter() {
                if let Some(edge) = dfs(
                    &graph,
                    v.clone(),
                    v.clone(),
                    &mut low,
                    &mut pre,
                    &mut cnt,
                    &ignore_edges,
                ) {
                    println!("edges: {:?} + {}", ignore_edges, edge);
                    let mut cut_edges = ignore_edges.clone();
                    cut_edges.insert(edge);
                    return cut_edges;
                }
            }
        }
    }

    HashSet::new()
}

// spent hours implementing Stoer-Wagner just for it to have bugs I couldn't
// figure out, so YOLO let's just brute force this bad boy and let it run while I watch
// a movie or something
// borrowed algorithm from https://stackoverflow.com/a/28917697
fn part1(file: &str) -> u32 {
    let graph = parse(file);
    println!("num edges: {}", graph.edges.len());

    let cut_edges = cut_edges(&graph);
    println!("cuts: {:?}", cut_edges);

    let p1 = bfs(
        &graph,
        graph.vertices.iter().next().unwrap().clone(),
        cut_edges,
    );

    p1.len() as u32 * (graph.vertices.len() - p1.len()) as u32
}

fn part2(file: &str) -> u32 {
    0
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
