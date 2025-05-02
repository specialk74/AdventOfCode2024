use std::{collections::HashMap, fs::read_to_string, str::FromStr, vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Stone {
    value: usize,
}

impl Stone {
    fn split(&self) -> Vec<Self> {
        if self.value == 0 {
            return vec![Self { value: 1 }];
        }

        let value_str = self.value.to_string();
        if value_str.len() % 2 == 0 {
            let middle = value_str.len() / 2;
            let splitted = value_str.split_at(middle);
            return vec![
                splitted.0.parse::<Stone>().unwrap(),
                splitted.1.parse::<Stone>().unwrap(),
            ];
        }

        vec![Self {
            value: self.value * 2024,
        }]
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<usize>().map_err(|_| ())?;
        Ok(Stone { value })
    }
}

fn stoned_change_1(input: &str) -> usize {
    let stones = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<Stone>().unwrap())
        .collect::<Vec<_>>();

    let mut current = stones.clone();
    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in &current {
            new_stones.extend(stone.split());
        }
        current = new_stones;
    }

    current.len()
}

fn stoned_change_2(input: &str) -> usize {
    let stones = input
        .split_ascii_whitespace()
        .map(|s| (s.parse::<Stone>().unwrap(), 1))
        .collect::<HashMap<Stone, usize>>();

    let mut current = stones.clone();

    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, count) in &current {
            for new_stone in stone.split() {
                let entry = new_stones.entry(new_stone).or_default();
                *entry += count;
            }
        }

        current = new_stones;
    }

    current.iter().map(|s| s.1).sum()
}

fn main() {
    let values =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-11/src/input.txt").unwrap();
    println!("Result 1: {:?}", stoned_change_1(values.as_str()));
    println!("Result 2: {:?}", stoned_change_2(values.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "125 17";
        assert_eq!(55312, stoned_change_1(input));
    }
}
