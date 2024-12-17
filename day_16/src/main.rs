use petgraph::algo::astar;
use std::collections::HashSet;
use std::fs;

use petgraph::graph::{Graph, Neighbors};

fn main() {
    println!("Welcome to Day 16!");

    let contents = fs::read_to_string("input/day16.txt").expect("file not found");
    let obstacles = make_obstacle_dict(&contents);
    let (graph, endpoint) = make_graph(&contents, obstacles);
    let startpoint = start_pos(&contents);
    let part1 = find_shortest_path(graph, startpoint, endpoint);
    println!("{:?}", part1);
}
/// this is just a test to make sure I can construct a weighted graph
fn test_graph() -> Graph<u32, u32> {
    let edgevec: Vec<(u32, u32, u32)> = vec![(1, 2, 1), (2, 3, 6)];
    Graph::<u32, u32>::from_edges(&edgevec)
}
/// makes the actual graph out of the grid
/// each coordinate has 4 nodes, one for each facing direction
/// that are connected among themselves with a weight of 1000
/// they are connected to their neighbors with an edge weight of 1
fn make_graph(s: &str, obstacles: HashSet<(usize, usize)>) -> (Graph<u32, u32>, u32) {
    let mut edgevec: Vec<(u32, u32, u32)> = Vec::new();
    let mut gridwidth = 0;
    let mut u = 0;
    let mut r = 1;
    let mut d = 2;
    let mut l = 3;
    let mut endpoint = 0;
    for (i, line) in s.lines().enumerate() {
        gridwidth = line.len();
        for (j, c) in line.chars().enumerate() {
            if c == 'E' {
                // all this horrible remapping
                //and type casting is necessary
                // because the petgraph library doesn't
                //accept a tuple (nor usize) as a node label
                //or I don't understand how to use it
                u = i * gridwidth * 4 + j * 4;
                endpoint = u as u32;
                if (!obstacles.contains(&(j, i - 1))) {
                    edgevec.push((((i - 1) * gridwidth * 4 + j * 4 + 2) as u32, u as u32, 1));
                }
                if (!obstacles.contains(&(j, i + 1))) {
                    edgevec.push((((i + 1) * gridwidth * 4 + j * 4) as u32, u as u32, 1));
                }
                if (!obstacles.contains(&(j - 1, i))) {
                    edgevec.push(((i * gridwidth * 4 + (j - 1) * 4 + 1) as u32, u as u32, 1));
                }
                if (!obstacles.contains(&(j + 1, i))) {
                    edgevec.push(((i * gridwidth * 4 + (j + 1) * 4 + 3) as u32, u as u32, 1));
                }
            } else if c != '#' {
                u = i * gridwidth * 4 + j * 4;
                r = u + 1;
                d = u + 2;
                l = u + 3;
                edgevec.push((u as u32, r as u32, 1000));
                edgevec.push((u as u32, l as u32, 1000));
                edgevec.push((r as u32, d as u32, 1000));
                edgevec.push((r as u32, u as u32, 1000));
                edgevec.push((d as u32, l as u32, 1000));
                edgevec.push((d as u32, r as u32, 1000));
                edgevec.push((l as u32, u as u32, 1000));
                edgevec.push((l as u32, d as u32, 1000));

                if (!obstacles.contains(&(j, i - 1))) {
                    edgevec.push((u as u32, ((i - 1) * gridwidth * 4 + j * 4) as u32, 1));
                }
                if (!obstacles.contains(&(j, i + 1))) {
                    edgevec.push((d as u32, ((i + 1) * gridwidth * 4 + j * 4 + 2) as u32, 1));
                }
                if (!obstacles.contains(&(j - 1, i))) {
                    edgevec.push((l as u32, (i * gridwidth * 4 + (j - 1) * 4 + 3) as u32, 1));
                }
                if (!obstacles.contains(&(j + 1, i))) {
                    edgevec.push((r as u32, (i * gridwidth * 4 + (j + 1) * 4 + 1) as u32, 1));
                }
            }
        }
    }
    (Graph::<u32, u32>::from_edges(&edgevec), endpoint)
}
/// make a HashSet of all the Coordinates that are walls
fn make_obstacle_dict(contents: &str) -> HashSet<(usize, usize)> {
    let mut y = 0;
    let mut obstacles = HashSet::new();
    for line in contents.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                obstacles.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    obstacles
}
/// find the start position
fn start_pos(s: &str) -> u32 {
    for (i, line) in s.lines().enumerate() {
        let gridwidth = line.len();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                return (i * gridwidth * 4 + 4 * j + 1) as u32;
            }
        }
    }
    // should hopefully not happen
    // this is only if S is not found
    return 0;
}
/// use A* algorithm to find the weight of the shortest path
/// returns 0 if there is no path between start and end coordinate
fn find_shortest_path(graph: Graph<u32, u32>, startpoint: u32, endpoint: u32) -> u32 {
    let path = astar(
        &graph,
        startpoint.into(),
        |finish| finish == endpoint.into(),
        |e| *e.weight(),
        |_| 0,
    );

    match path {
        Some((w, p)) => w,
        _ => 0,
    }
}
