use std::collections::HashSet;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input/day06.txt").expect("File not found");
    let obstacles = make_obstacle_dict(&contents);
    let startpos = initital_pos(&contents);
    let limits = grid_limits(&contents);
    let res = move_to_limits(startpos[0].clone(), obstacles, limits);
    println!("Welcome to Day 6!");
    println!("Part1:{:?}", res.len())
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
impl Facing {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Facing::Up),
            '<' => Some(Facing::Left),
            '>' => Some(Facing::Right),
            'v' => Some(Facing::Down),
            _ => None,
        }
    }
    fn turn_right(self: Facing) -> Self {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
}
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

fn initital_pos(contents: &str) -> Vec<(Coord, Facing)> {
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
        .filter(|(coor, c)| *c == '^')
        .map(|(Coord(x, y), c)| (Coord(y + 1, x + 1), Facing::from_char(c).unwrap()))
        .collect()
}

fn grid_limits(grid: &str) -> Coord {
    let ylim = grid.lines().collect::<Vec<&str>>().len();
    let xlim = grid
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>().len())
        .next()
        .unwrap();
    return Coord(xlim, ylim);
}

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

fn outside_limits(curr_pos: (Coord, Facing), limits: Coord) -> bool {
    let (Coord(x, y), _) = curr_pos;
    let Coord(xlim, ylim) = limits;
    x > xlim || x < 1 || y > ylim || y < 1
}
fn move_to_limits(
    start_pos: (Coord, Facing),
    obstacles: HashSet<Coord>,
    limits: Coord,
) -> HashSet<Coord> {
    let mut visited = HashSet::new();
    let mut curr_pos = start_pos.clone();
    visited.insert(start_pos.0);
    let (mut v, mut p) = move_one_step(&obstacles, &mut visited, &curr_pos);
    while !outside_limits(p, limits.clone()) {
        (visited, curr_pos) = move_one_step(&obstacles, &mut visited, &curr_pos);
        (v, p) = move_one_step(&obstacles, &mut v, &curr_pos);
    }
    return visited;
}
