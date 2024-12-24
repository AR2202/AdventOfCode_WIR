use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
fn main() {
    println!("Welcome to Day 20!");
    let contents = fs::read_to_string("input/day20.txt").expect("file not found");
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap_or("").chars().count();
    let obstacles = make_obstacle_dict(&contents);
    let graph = make_graph(&obstacles, width, height);
    let startpoint = find_pos(&contents, 'S');
    let endpoint = find_pos(&contents, 'E');
    let paths_from_start = find_shortest_paths(&graph, startpoint);
    let paths_from_end = find_shortest_paths(&graph, endpoint);
    let remaining = find_remaining_time(paths_from_start.clone(), endpoint, 100);

    //println!("{:?}", remaining);
   // println!("{:?}", &paths_from_end);
    println!("{:?}", find_low_cost_paths(paths_from_start, paths_from_end, endpoint, 100, width.try_into().unwrap_or(0)));
}
/// makes a graph with connections between coordinates that don't have a fallen byte
fn make_graph(
    walls_dict: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> Graph<u32, u32> {
    let mut edgevec: Vec<(usize, usize, usize)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if !walls_dict.contains(&(x, y)) {
                if x > 0 && !walls_dict.contains(&(x - 1, y)) {
                    edgevec.push((y * width + x, y * width + x - 1, 1));
                    edgevec.push((y * width + x - 1, y * width + x, 1));
                }
                if x < width - 1 && !walls_dict.contains(&(x + 1, y)) {
                    edgevec.push((y * width + x, y * width + x + 1, 1));
                    edgevec.push((y * width + x + 1, y * width + x, 1));
                }
                if y > 0 && !walls_dict.contains(&(x, y - 1)) {
                    edgevec.push(((y - 1) * width + x, y * width + x, 1));
                    edgevec.push((y * width + x, (y - 1) * width + x, 1));
                }
                if y < height - 1 && !walls_dict.contains(&(x, y + 1)) {
                    edgevec.push(((y + 1) * width + x, y * width + x, 1));
                    edgevec.push((y * width + x, (y + 1) * width + x, 1));
                }
            }
        }
    }
    let edgevec_u32: Vec<(u32, u32, u32)> = edgevec
        .iter()
        .map(|(source, target, weight)| (*source as u32, *target as u32, *weight as u32))
        .collect();
    Graph::<u32, u32>::from_edges(&edgevec_u32)
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
fn find_pos(s: &str, cr: char) -> u32 {
    for (i, line) in s.lines().enumerate() {
        let gridwidth = line.len();
        for (j, c) in line.chars().enumerate() {
            if c == cr {
                return (i * gridwidth + j) as u32;
            }
        }
    }
    // should hopefully not happen
    // this is only if S is not found
    return 0;
}
/// used Dijkstra's algorithm to find shortest paths to all
/// other nodes, if a path exists
fn find_shortest_paths(graph: &Graph<u32, u32>, startpoint: u32) -> HashMap<NodeIndex, u32> {
    dijkstra(graph, startpoint.into(), None, |_| 1)
}
/// finds the remaining time to get to endpoint
/// from each node that has a path from startpoint
/// at least "time_save" lower than the path to the endpoint
/// subtract 2 as the connection will incur at least cost 2
fn find_remaining_time(
    costs: HashMap<NodeIndex, u32>,
    endpoint: u32,
    time_save: u32,
) -> HashMap<NodeIndex, u32> {
    let cost_without_cheat = *&costs
        .iter()
        .filter(|(node, &cost)| **node == <u32 as Into<NodeIndex>>::into(endpoint))
        .map(|x| *x.1)
        .next()
        .unwrap();
    println!("cost without cheat: {:?}", &cost_without_cheat);
    costs
        .into_iter()
        .filter(|(node, cost)| *cost < cost_without_cheat - time_save - 2)
        .map(|(node, cost)| (node, cost_without_cheat - time_save - cost))
        .collect()
}
/// finds all neighbors of a coordinate
fn find_neighbors(coord: u32, gridwidth: u32) -> Vec<u32> {
    let mut neighbors = Vec::new();
    //check to prevent overflow of unsigned int
    if coord >= 1 {
        neighbors.push(coord - 1);
    }
    if coord >= gridwidth {
        neighbors.push(coord - gridwidth);
    }
    neighbors.push(coord + 1);
    neighbors.push(coord + gridwidth);
    neighbors
}
/// creates a hashmap of the number of low_cost paths
fn find_low_cost_paths(
    costs: HashMap<NodeIndex, u32>,
    paths_from_end: HashMap<NodeIndex, u32>,
    endpoint: u32,
    time_save: u32,
    gridwidth: u32,
) ->  u32 {
    let mut low_cost_paths = HashMap::new();
    let costmap = find_remaining_time(costs.clone(), endpoint, time_save);
    let mut cost_vec: Vec<(&NodeIndex, &u32)> = costmap.iter().collect();
    cost_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (node, cost) in cost_vec.iter().rev() {
        let n = (**node).index().try_into().unwrap_or(0);
        for neighbor in find_neighbors(n, gridwidth).iter() {
            let mut n_paths = 0;
            if !low_cost_paths.contains_key(&(n,*neighbor)) {
                
                for neighbors_neighbor in find_neighbors(*neighbor, gridwidth).iter() {
                    
                    let maybe_path = paths_from_end.get(&(*neighbors_neighbor).into());
                   
                    match maybe_path {
                        None => {},
                        Some(c) => {
                          
                            if *neighbors_neighbor != n && **cost>=2 && *c <= **cost - 2 {
                                n_paths += 1;
                                
                            }
                        }
                    }
                    
                }
                low_cost_paths.insert((n,*neighbor), n_paths);
            }
        }
       
    }

    low_cost_paths.values().sum()
}
