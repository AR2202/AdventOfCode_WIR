use std::collections::HashSet;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input/day06.txt").expect("File not found");
    let obstacles = make_obstacle_dict(&contents);
    let startpos = initital_pos(&contents);
    let limits = grid_limits(&contents);
    let visited = move_to_limits(&startpos, &obstacles, &limits);
    // too inefficient
    //let non_obstacles = make_non_obstacle_dict(&contents);
    //let all_insert_pos = insert_obstacle(&obstacles, &non_obstacles);
    let all_insert_pos = insert_obstacle(&obstacles, &visited);
    println!("Welcome to Day 6!");
    println!("Part1:{:?}", visited.len());
    let num_loops = all_insert_pos
        .iter()
        .map(|obs| move_to_outcome(obs, &mut HashSet::new(), &startpos, &limits))
        .filter(|outcome| outcome == &Outcome::Loop)
        .count();
    println!("Part2:{:?}", num_loops);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord(usize, usize);
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Outcome {
    Loop,
    Escape,
}
impl Facing {
    /// convert from char to Facing
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Facing::Up),
            '<' => Some(Facing::Left),
            '>' => Some(Facing::Right),
            'v' => Some(Facing::Down),
            _ => None,
        }
    }
    /// turn right 90 degrees and return new Facing
    fn turn_right(self: Facing) -> Self {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
}
/// save coordinates of all obstacles to HashSet
fn make_obstacle_dict(contents: &str) -> HashSet<Coord> {
    let mut y = 1;
    let mut obstacles = HashSet::new();
    for line in contents.lines() {
        let mut x = 1;
        for c in line.chars() {
            if c == '#' {
                obstacles.insert(Coord(x, y));
            }
            x += 1;
        }
        y += 1;
    }
    obstacles
}

/// pull out initial position from String
fn initital_pos(contents: &str) -> (Coord, Facing) {
    contents
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| (Coord(i, j), c))
                .collect()
        })
        .collect::<Vec<Vec<(Coord, char)>>>()
        .concat()
        .into_iter()
        .filter(|(coor, c)| vec!['<', '>', '^', 'v'].contains(c))
        .map(|(Coord(x, y), c)| (Coord(y + 1, x + 1), Facing::from_char(c).unwrap()))
        .collect::<Vec<(Coord, Facing)>>()[0]
        .clone()
}

/// get the limits of the grid
fn grid_limits(grid: &str) -> Coord {
    let ylim = grid.lines().collect::<Vec<&str>>().len();
    let xlim = grid
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>().len())
        .next()
        .unwrap();
    return Coord(xlim, ylim);
}

///move one step forward if possible or turn right
fn move_one_step(
    obstacles: &HashSet<Coord>,
    visited: &mut HashSet<Coord>,
    curr_pos: &(Coord, Facing),
) -> (HashSet<Coord>, (Coord, Facing)) {
    let (Coord(x, y), f) = curr_pos;
    let next_coord = match f {
        Facing::Up => Coord(*x, y - 1),
        Facing::Right => Coord(x + 1, *y),
        Facing::Down => Coord(*x, y + 1),
        Facing::Left => Coord(x - 1, *y),
    };
    if obstacles.contains(&next_coord) {
        return (visited.clone(), (Coord(*x, *y), f.clone().turn_right()));
    } else {
        visited.insert(next_coord.clone());
        return (visited.clone(), (next_coord, f.clone()));
    }
}

/// check if position is outside grid
fn outside_limits(curr_pos: (Coord, Facing), limits: Coord) -> bool {
    let (Coord(x, y), _) = curr_pos;
    let Coord(xlim, ylim) = limits;
    x > xlim || x < 1 || y > ylim || y < 1
}
/// move until outside limits of grid
fn move_to_limits(
    start_pos: &(Coord, Facing),
    obstacles: &HashSet<Coord>,
    limits: &Coord,
) -> HashSet<Coord> {
    let mut visited = HashSet::new();
    let mut curr_pos = start_pos.clone();
    visited.insert(start_pos.0.clone());
    let (mut v, mut p) = move_one_step(&obstacles, &mut visited, &curr_pos);
    while !outside_limits(p, limits.clone()) {
        (visited, curr_pos) = move_one_step(&obstacles, &mut visited, &curr_pos);
        (v, p) = move_one_step(&obstacles, &mut v, &curr_pos);
    }
    return visited;
}
// Part 2
/// save coordinates of all obstacles to HashSet
fn make_non_obstacle_dict(contents: &str) -> HashSet<Coord> {
    let mut y = 1;
    let mut obstacles = HashSet::new();
    for line in contents.lines() {
        let mut x = 1;
        for c in line.chars() {
            if c == '.' {
                obstacles.insert(Coord(x, y));
            }
            x += 1;
        }
        y += 1;
    }
    obstacles
}
/// insert one obstacle
fn insert_obstacle(
    obstacles: &HashSet<Coord>,
    no_obstacles: &HashSet<Coord>,
) -> Vec<HashSet<Coord>> {
    no_obstacles
        .iter()
        .map(|pos| {
            let mut obstacles_new = obstacles.clone();
            obstacles_new.insert(pos.clone());
            obstacles_new.clone()
        })
        .collect()
}
/// recursively move until either a loop occurs of agent escapes
///
fn move_to_outcome(
    obstacles: &HashSet<Coord>,
    visited: &mut HashSet<(Coord, Facing)>,
    curr_pos: &(Coord, Facing),
    limits: &Coord,
) -> Outcome {
    // this is a hack to get rid of the obstacle in the start positions
    if obstacles.contains(&curr_pos.0) {
        return Outcome::Escape;
    }
    // check if we have visited the current position before
    if visited.contains(&curr_pos) {
        return Outcome::Loop;
    }
    if outside_limits(curr_pos.clone(), limits.clone()) {
        return Outcome::Escape;
    }
    // cehck if current position is outside grid
    let (Coord(x, y), f) = curr_pos;
    visited.insert(curr_pos.clone());
    let next_coord = match f {
        Facing::Up => (Coord(*x, y - 1)),
        Facing::Right => Coord(x + 1, *y),
        Facing::Down => Coord(*x, y + 1),
        Facing::Left => Coord(x - 1, *y),
    };

    let next_facing;
    if obstacles.contains(&next_coord) {
        next_facing = (Coord(*x, *y), f.clone().turn_right());
    } else {
        next_facing = (next_coord.clone(), f.clone());
    }
    move_to_outcome(obstacles, visited, &next_facing.clone(), &limits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6a_test() {
        let contents = fs::read_to_string("input/day06ex.txt").expect("File not found");
        let obstacles = make_obstacle_dict(&contents);
        let startpos = initital_pos(&contents);
        let limits = grid_limits(&contents);
        let visited = move_to_limits(&startpos, &obstacles, &limits);
        assert_eq!(visited.len(), 41);
    }
}
