use std::{convert::TryInto, fs};
fn main() {
    println!("Welcome to Day 1!");
    let nums = read_input("input/Day01.txt");
    let sorted_lists = sep_lists(nums);
    let diffs = sum_differences(sorted_lists);
    println!("{:?}", diffs)
}
fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).unwrap();
    let numbers: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    numbers
}

fn sep_lists(numlist: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut first_list = numlist.iter().map(|l| l[0]).collect::<Vec<i32>>();
    first_list.sort();
    let mut second_list = numlist.iter().map(|l| l[1]).collect::<Vec<i32>>();
    second_list.sort();
    let zipped = first_list
        .into_iter()
        .zip(second_list.into_iter())
        .collect();
    zipped
}

fn sum_differences(numtuples: Vec<(i32, i32)>) -> i32 {
    numtuples.iter().map(|l| (l.0 - l.1).abs()).sum()
}
