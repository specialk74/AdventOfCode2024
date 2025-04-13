use std::fs::read_to_string;

fn total_distance2(nums: &[(i32, i32)]) -> u32 {
    // The vector in input needs to be sorted
    nums.iter()
        .map(|(l, r)| (*l as u32).abs_diff(*r as u32))
        .sum()
}

fn read_lines(filename: &str) -> Vec<(i32, i32)> {
    // Read from line unsorted values
    println!("Reading lines from file: {}", filename);
    let content = read_to_string(filename).unwrap();
    let lines = content.lines();
    let mut nums = vec![];
    for line in lines {
        let (left, right) = line.split_once("   ").unwrap();
        let left = left.parse::<i32>().unwrap();
        let right = right.parse::<i32>().unwrap();
        nums.push((left, right));
    }
    nums
}

fn sort_tuple_vec(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // Sort the vector of tuples
    // Sort the left and right values separately
    // Then combine them into a new vector
    let mut left = vec![];
    let mut right = vec![];
    for (l, r) in input {
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();
    let mut nums = vec![];
    for i in 0..left.len() {
        nums.push((left[i], right[i]));
    }
    nums
}

fn similarity_score(nums: &[(i32, i32)]) -> u32 {
    let left = nums.iter().map(|(l, _)| *l as u32).collect::<Vec<u32>>();
    let right = nums.iter().map(|(_, r)| *r as u32).collect::<Vec<u32>>();

    let mut score = 0;
    for item in left {
        score += right.iter().filter(|r| *r == &item).count() as u32 * item;
    }
    score
}

fn main() {
    let test = [(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
    let nums = sort_tuple_vec(test.to_vec());
    println!("Sum of absolute differences: {}", total_distance2(&nums));

    let lines = read_lines("/Users/specialk74/Rust/AdventOfCode2024/day-01/src/input.txt");
    let nums = sort_tuple_vec(lines.to_vec());
    println!("Sum of absolute differences: {}", total_distance2(&nums));

    println!("Similarity score for test: {}", similarity_score(&test));
    println!("Similarity score for file: {}", similarity_score(&lines));
}
