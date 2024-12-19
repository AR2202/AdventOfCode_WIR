use petgraph::adj::NodeIndex;
use petgraph::algo::{all_simple_paths, astar};
use petgraph::graph::{Graph, Neighbors};
use std::collections::HashSet;
use std::fs;
fn main() {
    println!("Welcome to Day 18!");
    let contents = fs::read_to_string("input/day18ex.txt").expect("file not found");
    let (fallen_bytes, rest_ex) = make_bytes_dict(&contents, 12);
    let example_graph = make_graph(&fallen_bytes, 7);

    let startpoint = 0;
    let endpoint = 48;
    println!(
        "example: {:?} ",
        find_shortest_path(example_graph.clone(), startpoint, endpoint)
    );
    println!(
        "part2 example: {} ",
        try_all_bytes(&contents, startpoint, endpoint, 12, 7)
    );
    let contents_inp = fs::read_to_string("input/day18.txt").expect("file not found");
    let (fallen_bytes_inp, rest) = make_bytes_dict(&contents_inp, 1024);
    let graph = make_graph(&fallen_bytes_inp, 71);
    let startpoint_p1 = 0;
    let endpoint_p1 = 5040;
    println!(
        "part1: {:?} ",
        find_shortest_path(graph.clone(), startpoint_p1, endpoint_p1)
    );
    println!(
        "part2: {} ",
        try_all_bytes(&contents_inp, startpoint_p1, endpoint_p1, 1024, 71)
    );
}
/// makes a HashSet of all the already fallen bytes
fn make_bytes_dict(contents: &str, nbytes: usize) -> (HashSet<(u32, u32)>, Vec<(u32, u32)>) {
    let linevec: Vec<&str> = contents.lines().collect();
    (
        linevec[0..nbytes]
            .iter()
            .map(|line| line.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
            .map(|v: Vec<u32>| (v[0], v[1]))
            .collect(),
        linevec[nbytes..]
            .iter()
            .map(|line| line.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
            .map(|v: Vec<u32>| (v[0], v[1]))
            .collect(),
    )
}
/// makes a graph with connections between coordinates that don't have a fallen byte
fn make_graph(bytes_dict: &HashSet<(u32, u32)>, limits: u32) -> Graph<u32, u32> {
    let mut edgevec = Vec::new();
    for x in 0..limits {
        for y in 0..limits {
            if !bytes_dict.contains(&(x, y)) {
                if x > 0 && !bytes_dict.contains(&(x - 1, y)) {
                    edgevec.push((y * limits + x, y * limits + x - 1, 1));
                    edgevec.push((y * limits + x - 1, y * limits + x, 1));
                }
                if x < limits - 1 && !bytes_dict.contains(&(x + 1, y)) {
                    edgevec.push((y * limits + x, y * limits + x + 1, 1));
                    edgevec.push((y * limits + x + 1, y * limits + x, 1));
                }
                if y > 0 && !bytes_dict.contains(&(x, y - 1)) {
                    edgevec.push(((y - 1) * limits + x, y * limits + x, 1));
                    edgevec.push((y * limits + x, (y - 1) * limits + x, 1));
                }
                if y < limits - 1 && !bytes_dict.contains(&(x, y + 1)) {
                    edgevec.push(((y + 1) * limits + x, y * limits + x, 1));
                    edgevec.push((y * limits + x, (y + 1) * limits + x, 1));
                }
            }
        }
    }
    Graph::<u32, u32>::from_edges(&edgevec)
}
/// finds the shortes path
fn find_shortest_path(graph: Graph<u32, u32>, startpoint: u32, endpoint: u32) -> Option<u32> {
    let path = astar(
        &graph,
        startpoint.into(),
        |finish| finish == endpoint.into(),
        |e| *e.weight(),
        |_| 0,
    );

    match path {
        Some((w, p)) => Some(w),
        _ => None,
    }
}
/// this finds the first one that blocks the path
/// for every simple path
/// too inefficient
fn find_blocking_all_paths(
    g: &Graph<u32, u32>,
    startpoint: u32,
    endpoint: u32,
    bytes: Vec<(u32, u32)>,
) -> (usize, (u32, u32)) {
    let mut first_blocking =
        all_simple_paths::<Vec<_>, _>(g, startpoint.into(), endpoint.into(), 0, None).map(|path| {
            let path_as_u32: Vec<u32> = path.into_iter().map(|node| node.index() as u32).collect();
            find_first_byte(path_as_u32, &bytes)
        });

    first_blocking.fold(
        (0, (0, 0)),
        |(acci, accb), (i, b)| {
            if i > acci {
                (i, b)
            } else {
                (acci, accb)
            }
        },
    )
}
/// this finds the first byte that blocks a path
fn find_first_byte(path: Vec<u32>, bytes: &Vec<(u32, u32)>) -> (usize, (u32, u32)) {
    for (i, b) in bytes.iter().enumerate() {
        if path.contains(&(b.0 + 71 * b.1)) {
            return (i, *b);
        }
    }
    return (1000000, (0, 0));
}
/// this tries all bytes after the initial ones and returns the first one that has no
/// result for path
fn try_all_bytes(
    contents_inp: &str,
    startpoint_p1: u32,
    endpoint_p1: u32,
    fallen: usize,
    limits: u32,
) -> &str {
    let ls = contents_inp.lines().collect::<Vec<&str>>();
    for num_bytes in fallen..ls.len() {
        let (fallen_bytes_inp, rest) = make_bytes_dict(contents_inp, num_bytes);
        let graph = make_graph(&fallen_bytes_inp, limits);
        match find_shortest_path(graph, startpoint_p1, endpoint_p1) {
            None => {
                return ls[num_bytes - 1];
            }
            _ => {}
        }
    }

    "not found"
}
