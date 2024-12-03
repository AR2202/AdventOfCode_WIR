use std::{convert::TryInto, fs};
///solving both parts of day 1 and printing
fn main() {
    println!("Welcome to Day 1!");
    let (part1, part2) = sol_day1("input/Day01.txt");
    println!("Day 1 Part 1: {:?}", part1);
    println!("Day 1 Part 2: {:?}", part2);
}

/// solving  day 1 from file input
fn sol_day1(filename: &str) -> (i32, i32) {
    let nums = read_input(filename);
    let (first_list, second_list) = extract_sep_lists(&nums);
    let zipped_lists = zip_lists(&first_list, &second_list);
    (
        sum_differences(&zipped_lists),
        mult_by_count(&first_list, &second_list),
    )
}
/// reading input from file and returning list of lists
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
/// separating the two lists and sorting them
fn extract_sep_lists(numlist: &Vec<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = numlist.iter().map(|l| l[0]).collect::<Vec<i32>>();
    first_list.sort();
    let mut second_list = numlist.iter().map(|l| l[1]).collect::<Vec<i32>>();
    second_list.sort();
    (first_list, second_list)
}
/// zipping lists to list of tuples
fn zip_lists<'a>(first_list: &'a Vec<i32>, second_list: &'a Vec<i32>) -> Vec<(&'a i32, &'a i32)> {
    let zipped = first_list
        .into_iter()
        .zip(second_list.into_iter())
        .collect();
    zipped
}
/// sum of differences of the tuple values
fn sum_differences(numtuples: &Vec<(&i32, &i32)>) -> i32 {
    numtuples.iter().map(|l| (l.0 - l.1).abs()).sum()
}
/// count occurrances of element in vector
fn count_occurrances(elem: i32, list: &Vec<i32>) -> i32 {
    list.iter()
        .filter(|&n| *n == elem)
        .count()
        .try_into()
        .unwrap()
}
/// multiply each element by its number of occurrances
fn mult_by_count(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1
        .iter()
        .map(|elem| count_occurrances(*elem, list2) * elem)
        .sum::<i32>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1a_test() {
        assert_eq!(sol_day1("input/Day01example.txt").0, 11);
    }
    #[test]
    fn day1b_test() {
        assert_eq!(sol_day1("input/Day01example.txt").1, 31);
    }
}
