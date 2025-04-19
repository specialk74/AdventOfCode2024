use std::fs::read_to_string;

fn make_calculus(result: usize, addendum: &[usize], part2: bool) -> usize {
    let debug = 0;
    let mut operations = vec!['+'; addendum.len() - 1];

    loop {
        if debug >= 2 {
            println!("operations: {:?}", operations);
        }
        let mut total = 0;
        for (i, operation) in operations.iter().enumerate() {
            if i == 0 {
                total = addendum[0];
            }
            match *operation {
                '+' => total += addendum[i + 1],
                '*' => total *= addendum[i + 1],
                '|' => {
                    if addendum[i + 1] < 10 {
                        total = total * 10 + addendum[i + 1];
                    } else if addendum[i + 1] < 100 {
                        total = total * 100 + addendum[i + 1];
                    } else if addendum[i + 1] < 1000 {
                        total = total * 1000 + addendum[i + 1];
                    } else {
                        panic!("Error: number too big");
                    }
                }
                _ => unreachable!(),
            }
        }

        if debug >= 2 {
            println!(
                "total: {:?} while result: {:?} for operations: {:?} in addendum: {:?}",
                total, result, operations, addendum
            );
        }

        if total == result {
            if debug >= 1 {
                println!(
                    "TROVATO per operations: {:?} result: {} addendum: {:?}",
                    operations, result, addendum
                );
            }
            return result;
        }

        let mut pos = operations.len() - 1;
        loop {
            if operations[pos] == '+' {
                operations[pos] = '*';
                break;
            } else if operations[pos] == '*' && part2 {
                operations[pos] = '|';
                break;
            } else if pos == 0 {
                return 0;
            }
            operations[pos] = '+';
            pos -= 1;
        }
    }

    0
}

fn operation_procedures_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        //println!("\n\n{}", line);
        let numbers: Vec<&str> = line.split(':').collect();
        let result = numbers[0].parse::<usize>().unwrap();
        let addendum = numbers[1]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        sum += make_calculus(result, &addendum, false);
    }
    sum
}

fn operation_procedures_2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        //println!("\n\n{}", line);
        let numbers: Vec<&str> = line.split(':').collect();
        let result = numbers[0].parse::<usize>().unwrap();
        let addendum = numbers[1]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        sum += make_calculus(result, &addendum, true);
    }
    sum
}

fn main() {
    let input =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-07/src/input.txt").unwrap();

    println!("Result 1: {}", operation_procedures_1(&input));

    println!("Result 2: {}", operation_procedures_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, operation_procedures_1(input));
    }

    #[test]
    fn test2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387, operation_procedures_2(input));
    }
}
