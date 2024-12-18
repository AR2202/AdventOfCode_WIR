use petgraph::algo::astar;
use std::collections::HashSet;
use std::fs;

use petgraph::graph::{Graph, Neighbors};
fn main() {
    println!("Welcome to Day 18!");
    let contents = fs::read_to_string("input/day18ex.txt").expect("file not found");
    let fallen_bytes = make_bytes_dict(&contents,12);
    let example_graph = make_graph(&fallen_bytes,7);
    
    let startpoint = 0;
    let endpoint = 48;
    println!("example: {:?} ",find_shortest_path(example_graph, startpoint, endpoint));
    let contents_inp = fs::read_to_string("input/day18.txt").expect("file not found");
    let fallen_bytes_inp = make_bytes_dict(&contents_inp,1024);
    let graph = make_graph(&fallen_bytes_inp,71);
   
    let startpoint_p1 = 0;
    let endpoint_p1 = 5040;
    println!("part1: {:?} ",find_shortest_path(graph, startpoint_p1, endpoint_p1));
}
fn make_bytes_dict(contents: &str, nbytes:usize) -> HashSet<(u32, u32)> {
 let linevec: Vec<&str> = contents.lines().collect();
 linevec[0..nbytes].iter().map(|line|line.split(",").map(|n| n.parse::<u32>().unwrap()).collect()).map(|v:Vec<u32>|(v[0],v[1])).collect()
}

fn make_graph(bytes_dict:&HashSet<(u32,u32)>, limits: u32) -> Graph<u32,u32>{
    let mut edgevec = Vec::new();
    for x in 0..limits{
        for y in 0..limits{
            if !bytes_dict.contains(&(x,y)){
                if x>0 && !bytes_dict.contains(&(x-1,y)){
                    edgevec.push((y*limits+x, y*limits+x-1,1));
                    edgevec.push((y*limits+x-1, y*limits+x,1));
                }
                if  x < limits - 1 && !bytes_dict.contains(&(x+1,y)){
                    edgevec.push((y*limits+x, y*limits+x+1,1));
                    edgevec.push((y*limits+x+1, y*limits+x,1));
                }
                if y>0 && !bytes_dict.contains(&(x,y-1)){
                    edgevec.push(((y-1)*limits+x, y*limits+x,1));
                    edgevec.push((y*limits+x,(y-1)*limits+x, 1));
                }
                if y < limits - 1  && !bytes_dict.contains(&(x,y+1)){
                    edgevec.push(((y+1)*limits+x, y*limits+x,1));
                    edgevec.push((y*limits+x,(y+1)*limits+x, 1));
                }
            }
        }
    }
    Graph::<u32, u32>::from_edges(&edgevec)
}
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