extern crate nalgebra as na;
use na::{ArrayStorage, Dynamic, Matrix2, VecStorage, Vector2, U2, U3};
use std::fs;
use std::num;
fn main() {
    println!("Welcome to Day 13!");

    let res_ex = win_all_prizes(parse_input("input/day13ex.txt"));
    let res = win_all_prizes(parse_input("input/day13.txt"));
    println!("example: {:?}", res_ex);
    println!("part1: {:?}", res);
}

fn win_prize(x_a: i32, x_b: i32, y_a: i32, y_b: i32, x: i32, y: i32) -> (bool, i32, i32) {
    // construct the matrix of the equations
    let a = Matrix2::new(x_a as f32, x_b as f32, y_a as f32, y_b as f32);
    // construct the vector of the solutions
    let mut b = Vector2::new(x as f32, y as f32);
    let decomp = a.lu();


    let button_presses = decomp.solve(&b).expect("no decomposition possible");

    // check if there is an integer solution
    let button_presses_rounded_a = button_presses[0].round();
    let button_presses_rounded_b = button_presses[1].round();
    let a_is_int = (button_presses_rounded_a - button_presses[0].abs()).abs() < 0.01;
    let b_is_int = (button_presses_rounded_b - button_presses[1].abs()).abs() < 0.01;
   
    return (
        (a_is_int && b_is_int),
        button_presses_rounded_a as i32,
        button_presses_rounded_b as i32,
    );
}

/// calculates the tokens necessary to win all winable games
fn calc_tokens(all_button_presses: Vec<(bool, i32, i32)>) -> i32 {
    all_button_presses
        .iter()
        .filter(|res| res.0)
        .map(|res| res.1 * 3 + res.2)
        .sum()
}
///parsing the equations coefficients
fn parse_input(fname: &str) -> Vec<(i32, i32, i32, i32, i32, i32)> {
    let contents = fs::read_to_string(fname).expect("File not found");
    let mut x_a = 0;
    let mut x_b = 0;
    let mut y_a = 0;
    let mut y_b = 0;
    let mut x_s = 0;
    let mut y_s = 0;
    let mut equations = Vec::new();
    for (i, e) in contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .enumerate()
    {
        match i % 4 {
            0 => {
                x_a = e[2][2..e[2].len() - 1].parse::<i32>().unwrap();
                y_a = e[3][2..].parse::<i32>().unwrap();
            }
            1 => {
                x_b = e[2][2..e[2].len() - 1].parse::<i32>().unwrap();
                y_b = e[3][2..].parse::<i32>().unwrap();
            }
            2 => {
                x_s = e[1][2..e[1].len() - 1].parse::<i32>().unwrap();
                y_s = e[2][2..].parse::<i32>().unwrap();
                equations.push((x_a, x_b, y_a, y_b, x_s, y_s));
            }
            _ => {}
        }
    }
    equations
}
/// passing in the tuple that gets output by parsing step to win_prizes
fn win_from_tuple(tup: &(i32, i32, i32, i32, i32, i32)) -> (bool, i32, i32) {
    let (x_a, x_b, y_a, y_b, x_s, y_s) = *tup;
    win_prize(x_a, x_b, y_a, y_b, x_s, y_s)
}
fn win_all_prizes(tups: Vec<(i32, i32, i32, i32, i32, i32)>) -> i32 {
    calc_tokens(tups.iter().map(|tup| win_from_tuple(tup)).collect())
}
