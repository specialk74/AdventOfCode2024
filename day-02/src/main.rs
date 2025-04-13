use std::{cmp::Ordering, fs::read_to_string};

fn check_safe_1_report(report: &Vec<i32>) -> i32 {
    let mut versus = 0;
    let debug = false;
    for i in 1..report.len() {
        match report[i].cmp(&report[i - 1]) {
            Ordering::Equal => {
                if debug {
                    println!("{:?} is usafe because two are equal", report);
                }
                return 0;
            }
            Ordering::Greater => {
                if versus == -1 {
                    if debug {
                        println!("{:?} is usafe because versus changed from -1 to 1", report);
                    }
                    return 0;
                }
                versus = 1;
            }
            Ordering::Less => {
                if versus == 1 {
                    if debug {
                        println!("{:?} is usafe because versus changed from 1 to -1", report);
                    }
                    return 0;
                }
                versus = -1;
            }
        }
        if (report[i] - report[i - 1]).abs() > 3 {
            if debug {
                println!("{:?} is usafe because diff is to high", report);
            }
            return 0;
        }
    }
    if debug {
        println!("{:?} is safe", report);
    }
    1
}

fn check_safe_2_report(report: &Vec<i32>) -> i32 {
    let mut versus = 0;
    let debug = true;
    for i in 1..report.len() {
        match report[i].cmp(&report[i - 1]) {
            Ordering::Equal => {
                if debug {
                    println!("{:?} is usafe because two are equal", report);
                }
                return 0;
            }
            Ordering::Greater => {
                if versus == -1 {
                    if debug {
                        println!("{:?} is usafe because versus changed from -1 to 1", report);
                    }
                    return 0;
                }
                versus = 1;
            }
            Ordering::Less => {
                if versus == 1 {
                    if debug {
                        println!("{:?} is usafe because versus changed from 1 to -1", report);
                    }
                    return 0;
                }
                versus = -1;
            }
        }
        if (report[i] - report[i - 1]).abs() > 3 {
            if debug {
                println!("{:?} is usafe because diff is to high", report);
            }
            return 0;
        }
    }
    if debug {
        println!("{:?} is safe", report);
    }
    1
}

fn check_safe_1_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for report in reports {
        safe += check_safe_1_report(report);
    }
    safe
}

fn check_safe_2_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for report in reports {
        let value = check_safe_1_report(&report);
        if value != 0 {
            safe += value;
            continue;
        }
        for i in 0..report.len() {
            let mut report_cloned = report.clone();
            report_cloned.remove(i);
            let value = check_safe_1_report(&report_cloned);
            if value == 0 {
                continue;
            }
            safe += value;
            break;
        }
    }
    safe
}

fn read_lines(filename: &str) -> Vec<Vec<i32>> {
    // Read from line unsorted values
    println!("Reading lines from file: {}", filename);
    let content = read_to_string(filename).unwrap();
    let lines = content.lines();
    let mut nums = vec![];
    for line in lines {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
        nums.push(values);
    }
    nums
}

fn main() {
    // let input_file = include_str!("../input.txt");
    let test = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    assert_eq!(2, check_safe_1_reports(&test));

    let values = read_lines("/Users/specialk74/Rust/AdventOfCode2024/day-02/src/input.txt");
    println!("Safe 1 reports: {}", check_safe_1_reports(&values));

    assert_eq!(4, check_safe_2_reports(&test));
    println!("Safe 2 reports: {}", check_safe_2_reports(&values));
}
