use std::collections::HashMap;
use std::fs;
fn main() {
    println!("Welcome to Day 5!");
    println!("Part1: {:?}", solve_day5_part1("input/day05.txt"));
    println!("Part2: {:?}", solve_day5_part2("input/day05.txt"));
}
/// reads the input and splits it on newline character
fn read_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let lists: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();
    lists
}
/// making vector of vector of page numbers
fn to_pagelists(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|line| line.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect()
}
/// making dictionary, mapping each
/// page to all pages it should be printed before
fn contruct_priority_dict(s: &str) -> HashMap<i32, Vec<i32>> {
    let rules: Vec<Vec<i32>> = s
        .lines()
        .map(|line| line.split("|").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    rules.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v[0]).or_insert(Vec::new()).push(v[1]);
        acc
    })
}
/// reversing each vector of numbers in a vector
fn rev_lists(pagelists: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let reversed: Vec<Vec<i32>> = pagelists
        .into_iter()
        .map(|list| list.into_iter().rev().collect())
        .collect();
    reversed
}

/// check if a list is correctly orderd
fn is_correctly_ordered(pagelist: &Vec<i32>, rules: HashMap<i32, Vec<i32>>) -> bool {
    let mut page_iter = pagelist.iter();
    while let Some(i) = page_iter.next() {
        if let Some(priorities) = rules.get(i) {
            let mut lookahead_iter = page_iter.clone();
            if lookahead_iter.any(|n| priorities.contains(n)) {
                return false;
            }
        }
    }
    true
}

/// filter out all correctly ordered lists
fn correctly_ordered(pagelists: Vec<Vec<i32>>, rules: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    pagelists
        .into_iter()
        .filter(|list| is_correctly_ordered(&list, rules.clone()))
        .collect()
}

/// filter out all incorrectly ordered lists
fn incorrectly_ordered(pagelists: Vec<Vec<i32>>, rules: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    pagelists
        .into_iter()
        .filter(|list| !is_correctly_ordered(&list, rules.clone()))
        .collect()
}

/// find the middle element of a vector
fn middle_elem(lists: Vec<i32>) -> i32 {
    lists[lists.len() / 2]
}

/// find all middle elements of all vectors
fn middles(lists: Vec<Vec<i32>>) -> Vec<i32> {
    lists.into_iter().map(|list| middle_elem(list)).collect()
}

/// sort the lists according to rules
fn sort_with_rules(pagelist: &Vec<i32>, rules: HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut remaining_pages: Vec<i32> = pagelist.clone(); // Clone to create a mutable copy
    let mut sorted = Vec::new();

    while let Some(i) = remaining_pages.first().cloned() {
        remaining_pages.remove(0);

        if let Some(priorities) = rules.get(&i) {
            if remaining_pages.iter().any(|n| priorities.contains(n)) {
                remaining_pages.push(i);
            } else {
                sorted.push(i);
            }
        } else {
            sorted.push(i);
        }
    }

    sorted
}
fn sort_all_with_rules(pagelists: &Vec<Vec<i32>>, rules: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    pagelists
        .iter()
        .map(|l| sort_with_rules(l, rules.clone()))
        .collect()
}
fn prepare_day5(filename: &str) -> (Vec<Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let inp = read_input(filename);
    let pagelists = to_pagelists(&inp[1]);
    let rev_pagelists = rev_lists(pagelists);
    let dict = contruct_priority_dict(&inp[0]);

    (rev_pagelists, dict)
}
fn solve_day5_part1(filename: &str) -> i32 {
    let (pagelists, dict) = prepare_day5(filename);

    let sum_middle = middles(correctly_ordered(pagelists, dict));

    sum_middle.iter().sum()
}
fn solve_day5_part2(filename: &str) -> i32 {
    let (pagelists, dict) = prepare_day5(filename);

    let incorrect = incorrectly_ordered(pagelists, dict.clone());
    let resorted_middles = middles(sort_all_with_rules(&incorrect, dict));

    resorted_middles.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5a_test() {
        assert_eq!(solve_day5_part1("input/day05ex.txt"), 143);
    }
}
