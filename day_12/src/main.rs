use petgraph::algo::{connected_components, min_spanning_tree};
use petgraph::data::Element;
use petgraph::data::FromElements;
use petgraph::graph::NodeIndex;
use petgraph::graph::{Graph, Neighbors, UnGraph};
use petgraph::visit::Bfs;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
fn main() {
    println!("Welcome to Day 12!");
    let contents = fs::read_to_string("input/day12.txt").expect("File not found");
    let coordmap = coorinates(&contents);
    println!(
        "{:?}",
        get_connected_subgraphs(&make_whole_graph(&coordmap, &contents.lines().count()))
    );
}

fn coorinates(contents: &str) -> HashMap<(usize, usize), char> {
    let coordvec: Vec<((usize, usize), char)> = contents
        .lines()
        .enumerate()
        .map(|(i, l)| l.chars().enumerate().map(|(j, c)| ((j, i), c)).collect())
        .collect::<Vec<Vec<((usize, usize), char)>>>()
        .concat();
    coordvec.into_iter().collect()
}

fn make_edgevec(
    coorddict: &HashMap<(usize, usize), char>,
    coord: &(usize, usize),
    gridsize: &usize,
) -> Vec<(u32, u32)> {
    let neighbors_coord = neighbors(&coord);
    let char_coord = coorddict.get(&coord);
    let edgevec: Vec<(u32, u32)> = neighbors_coord
        .iter()
        .filter(|ncoord| coorddict.get(ncoord) == char_coord)
        .map(|neighb| {
            (
                linearize_coord(&coord, gridsize),
                linearize_coord(neighb, gridsize),
            )
        })
        .collect();
    edgevec
}
fn whole_grid_edgevec(
    coorddict: &HashMap<(usize, usize), char>,
    gridsize: &usize,
) -> Vec<(u32, u32)> {
    let coords: Vec<(usize, usize)> = coorddict.clone().into_keys().collect();

    coords
        .iter()
        .map(|coord| make_edgevec(&coorddict, coord, &gridsize))
        .collect::<Vec<Vec<(u32, u32)>>>()
        .concat()
}
fn make_graph(
    coorddict: &HashMap<(usize, usize), char>,
    coord: &(usize, usize),
    gridsize: &usize,
) -> Graph<u32, ()> {
    let edgevec: Vec<(u32, u32)> = make_edgevec(coorddict, coord, gridsize);
    Graph::<u32, ()>::from_edges(&edgevec)
}

fn make_whole_graph(coorddict: &HashMap<(usize, usize), char>, gridsize: &usize) -> Graph<u32, ()> {
    let edgevec: Vec<(u32, u32)> = whole_grid_edgevec(coorddict, gridsize);
    Graph::<u32, ()>::from_edges(&edgevec)
}
fn linearize_coord(coord: &(usize, usize), gridsize: &usize) -> u32 {
    let &(x, y) = coord;
    (y * gridsize + x) as u32
}

fn neighbors(coord: &(usize, usize)) -> Vec<(usize, usize)> {
    let &(x, y) = coord;
    if (x == 0) {
        if (y == 0) {
            vec![(x + 1, y), (x, y + 1)]
        } else {
            vec![(x + 1, y), (x, y - 1), (x, y + 1)]
        }
    } else if (y == 0) {
        vec![(x - 1, y), (x + 1, y), (x, y + 1)]
    } else {
        //careful with 0 coordinates, as this will lead to type errors with usize
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
    }
}

fn get_connected_subgraphs<N, E>(graph: &Graph<N, E>) -> usize
where
    N: Clone,
    E: Clone,
{
    let mut visited = HashSet::new();
    let mut subgraphs = Vec::new();
    let mut component_sizes = Vec::new();
    let mut neighbours_counts: Vec<usize> = Vec::new();
    let mut fence_cost: Vec<usize> = Vec::new();

    for start in graph.node_indices() {
        if visited.contains(&start) {
            continue;
        }

        let mut component_nodes = Vec::new();
        let mut bfs = Bfs::new(graph, start);
        let mut neighbors_count = Vec::new();

        while let Some(node) = bfs.next(graph) {
            if visited.insert(node) {
                component_nodes.push(node);
                neighbors_count
                    .push(4 - graph.neighbors_directed(node, Direction::Outgoing).count());
            }
        }

        subgraphs.push(component_nodes.clone());
        component_sizes.push(component_nodes.len());
        neighbours_counts.push(neighbors_count.iter().sum());
        fence_cost.push(component_nodes.len() * neighbors_count.iter().sum::<usize>());
    }

    fence_cost.iter().sum()
}
