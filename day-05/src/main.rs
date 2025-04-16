use regex::Regex;
use std::{clone, fs::read_to_string};

fn launch_safety_manual_1(input: &str) -> i32 {
    let re = Regex::new(r"\d+\|\d+|(\d+,?)+").unwrap();
    let mut rules = vec![];
    let mut page_number_update: Vec<Vec<i32>> = vec![];
    for caps in re.captures_iter(input) {
        if let Some(m) = caps.get(0) {
            let s = m.as_str().trim();
            if s.contains('|') {
                let parts: Vec<&str> = s.split('|').collect();
                let left = parts[0].parse::<i32>().unwrap();
                let right = parts[1].parse::<i32>().unwrap();
                rules.push((left, right));
            } else if s.contains(',') {
                page_number_update
                    .push(s.split(',').filter_map(|x| x.parse::<i32>().ok()).collect());
            }
        }
    }

    let mut reverse;
    let mut sum = 0;
    for page in page_number_update {
        //present = 0;
        reverse = 0;
        for pos in 1..page.len() {
            reverse += rules
                .iter()
                .filter(|(left, right)| page[pos] == *left && page[pos - 1] == *right)
                .count();
        }
        if reverse == 0 {
            sum += page[page.len() / 2]
        }
    }
    sum
}

fn get_reverse(rules: &[(i32, i32)], page: &[i32], pos: usize) -> usize {
    rules
        .iter()
        .filter(|(left, right)| page[pos] == *left && page[pos - 1] == *right)
        .count()
}

fn adjust_sequence(rules: &[(i32, i32)], page: &[i32]) -> Vec<i32> {
    let mut new_page = page.to_vec();
    let mut reverse = 0;
    for pos in 1..page.len() {
        reverse += get_reverse(rules, page, pos);
        if reverse != 0 {
            let temp = new_page[pos];
            new_page[pos] = page[pos - 1];
            new_page[pos - 1] = temp;
            break;
        }
    }

    // Use the recorsive function to check if the sequence is correct
    if reverse != 0 {
        let old_page = new_page.clone();
        new_page = adjust_sequence(rules, &old_page);
    }
    new_page
}

fn launch_safety_manual_2(input: &str) -> i32 {
    let re = Regex::new(r"\d+\|\d+|(\d+,?)+").unwrap();
    let mut rules = vec![];
    let mut page_number_update: Vec<Vec<i32>> = vec![];
    for caps in re.captures_iter(input) {
        if let Some(m) = caps.get(0) {
            let s = m.as_str().trim();
            if s.contains('|') {
                let parts: Vec<&str> = s.split('|').collect();
                let left = parts[0].parse::<i32>().unwrap();
                let right = parts[1].parse::<i32>().unwrap();
                rules.push((left, right));
            } else if s.contains(',') {
                page_number_update
                    .push(s.split(',').filter_map(|x| x.parse::<i32>().ok()).collect());
            }
        }
    }

    let mut sum = 0;
    for page in page_number_update {
        let mut reverse = 0;
        for pos in 1..page.len() {
            reverse += get_reverse(&rules, &page, pos);
        }
        if reverse != 0 {
            let new_page = adjust_sequence(&rules, &page);
            sum += new_page[new_page.len() / 2];
        }
    }
    sum
}

fn main() {
    let values =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-05/src/input.txt").unwrap();
    println!("Result 1: {:?}", launch_safety_manual_1(values.as_str()));
    println!("Result 2: {:?}", launch_safety_manual_2(values.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "47|53
                    97|13
                    97|61
                    97|47
                    75|29
                    61|13
                    75|53
                    29|13
                    97|29
                    53|29
                    61|53
                    97|53
                    61|29
                    47|13
                    75|47
                    97|75
                    47|61
                    75|61
                    47|29
                    75|13
                    53|13

                    75,47,61,53,29
                    97,61,53,29,13
                    75,29,13
                    75,97,47,61,53
                    61,13,29
                    97,13,75,29,47";
        assert_eq!(143, launch_safety_manual_1(input));
    }

    #[test]
    fn test2() {
        let input = "47|53
                    97|13
                    97|61
                    97|47
                    75|29
                    61|13
                    75|53
                    29|13
                    97|29
                    53|29
                    61|53
                    97|53
                    61|29
                    47|13
                    75|47
                    97|75
                    47|61
                    75|61
                    47|29
                    75|13
                    53|13

                    75,47,61,53,29
                    97,61,53,29,13
                    75,29,13
                    75,97,47,61,53
                    61,13,29
                    97,13,75,29,47";
        assert_eq!(123, launch_safety_manual_2(input));
    }
}
