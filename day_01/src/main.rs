use std::{convert::TryInto, fs};
fn main() {
    println!("Welcome to Day 1!");
    let nums = read_input("input/Day01.txt");
    let first_list = extract_first_list(&nums);
    let second_list = extract_second_list(&nums);
    let zipped_lists = zip_lists(&first_list, &second_list);
    let diffs = sum_differences(&zipped_lists);
    let product_with_counts = mult_by_count(&first_list, &second_list);
    println!("Day 1 Part 1: {:?}", diffs);
    println!("Day 1 Part 2: {:?}", product_with_counts);
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

fn extract_first_list(numlist: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut first_list = numlist.iter().map(|l| l[0]).collect::<Vec<i32>>();
    first_list.sort();
    first_list
    
}
fn extract_second_list(numlist: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut second_list = numlist.iter().map(|l| l[1]).collect::<Vec<i32>>();
    second_list.sort();
    second_list
    
}
fn zip_lists<'a>(first_list: &'a Vec<i32>, second_list:&'a Vec<i32>) -> Vec<(&'a i32, &'a i32)> {

    let zipped = first_list
        .into_iter()
        .zip(second_list.into_iter())
        .collect();
    zipped
}
fn sum_differences(numtuples: &Vec<(&i32, &i32)>) -> i32 {
    numtuples.iter().map(|l| (l.0 - l.1).abs()).sum()
}

fn count_occurrances(elem: i32, list:&Vec<i32>) -> i32{
    list.iter().filter(|&n| *n == elem).count().try_into().unwrap()
}

fn mult_by_count(list1: &Vec<i32>, list2: &Vec<i32>) -> i32{
    list1.iter().map(|elem| count_occurrances(*elem, list2) * elem).sum::<i32>().try_into().unwrap()
}