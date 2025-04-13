use regex::Regex;
use regex::RegexSet;
use std::fs::read_to_string;

fn take_1_mul(input: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    }
    sum
}

fn take_2_mul(input: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut do_it = true;
    for caps in re.captures_iter(input) {
        if let Some(m) = caps.get(0) {
            if m.as_str() == "do()" {
                do_it = true;
            } else if m.as_str() == "don't()" {
                do_it = false;
            }
        }
        if do_it {
            if let (Some(left), Some(right)) = (caps.get(1), caps.get(2)) {
                sum +=
                    left.as_str().parse::<i32>().unwrap() * right.as_str().parse::<i32>().unwrap();
            }
        }
    }
    sum
}

fn main() {
    let input =
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

    assert_eq!(161, take_1_mul(&input));

    let values =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-03/src/input.txt").unwrap();
    println!("Result 1: {:?}", take_1_mul(values.as_str()));

    let input =
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    assert_eq!(48, take_2_mul(&input));
    println!("Result 2: {:?}", take_2_mul(values.as_str()));
}
