use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    catch: bool,
    pos: usize,
    x: usize,
    y: usize,
    children: Vec<Number>,
}

impl Number {
    fn count(&mut self, num: &mut Vec<usize>) {
        // println!("value:{} - {}x{}", self.value, self.y, self.x);
        if self.children.is_empty() {
            // println!("pos: {} - value: {}", self.pos, self.value);
            if self.value == 9 && !self.catch {
                self.catch = true;
                num.push(self.pos);
            }
        } else {
            for child in self.children.iter_mut() {
                child.count(num);
            }
        }
    }
}

fn are_near(num1: &Number, num2: &Number, witdh: usize, length: usize) -> bool {
    if num1.y == num2.y {
        // destra
        if ((num1.x + 1) < witdh) && ((num1.x + 1) == num2.x) {
            return true;
        }
        // sinistra
        if num1.x > 0 && num1.x - 1 == num2.x {
            return true;
        }
    }

    if num1.x == num2.x {
        // sotto
        if ((num1.y + 1) < length) && ((num1.y + 1) == num2.y) {
            return true;
        }
        // sopra
        if num1.y > 0 && num1.y - 1 == num2.y {
            return true;
        }
    }

    false
}

fn find_trailhead_1(input: &Vec<u32>, witdh: usize, length: usize, all: bool) -> usize {
    let mut result = 0;
    let mut numbers = Vec::new();
    let max_number: usize = 9;

    for number in 0..=max_number {
        numbers.push(
            input
                .iter()
                .enumerate()
                .filter(|(_, f)| **f as usize == number)
                .map(|(i, _)| Number {
                    value: number,
                    catch: false,
                    pos: i,
                    x: i % witdh,
                    y: i / witdh,
                    children: Vec::new(),
                })
                .collect::<Vec<_>>(),
        );
    }

    //println!("numbers: {:?}", numbers);

    // Per ogni zero devo controllare se c'è un uno vicino e così via fino al 9
    for i in 1..=max_number {
        let (num_before, num_after) = numbers.split_at_mut(i);
        for num in &mut num_before[i - 1] {
            let close_correct_numbers = num_after[0]
                .iter()
                .filter(|n| are_near(num, n, witdh, length))
                .cloned()
                .collect::<Vec<Number>>();
            num.children = close_correct_numbers;
        }
    }

    //println!("numbers: {:?}", numbers);

    for i in (1..max_number).rev() {
        let (num_before, num_after) = numbers.split_at_mut(i);
        // println!("\n\n");
        // println!("num_before: {:?}", num_before[num_before.len() - 1]);
        // println!("\nnum_after: {:?}", num_after[0]);

        // devo inserire i children nel num_after dentro i children del num_before
        for num in num_before[num_before.len() - 1].iter_mut() {
            for child in num.children.iter_mut() {
                // println!("\nchild prima: {:?}", child);
                // Ricerco in num_after questo child, prendo i suoi children e li copio nel
                // in questo child
                if let Some(num_after_filtered) = num_after[0].iter().find(|n| n.pos == child.pos) {
                    child.children = num_after_filtered.children.clone();
                }
                // println!("child dopo: {:?}", child);
            }
        }
    }

    // println!("numbers: {:?}", numbers);

    for number in numbers[0].iter_mut() {
        let mut num = Vec::new();
        number.count(&mut num);
        if all {
            result += num.len();
        } else {
            use std::collections::HashSet;
            let unique_nums: HashSet<_> = num.iter().cloned().collect();
            result += unique_nums.len();
        }
        //println!("num: {:?}\n\n", num);
    }

    result
}

fn main() {
    let input =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-10/src/input.txt").unwrap();
    let mut width = 0;
    if let Some(line) = input.lines().next() {
        width = line.len() as i32;
    }
    let lines = input.lines().count();
    println!("width: {} - length: {}", width, lines);
    let input = input
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(10))
        .collect::<Vec<u32>>();
    // println!("content: {}", content_str);
    println!(
        "Result 1: {}",
        find_trailhead_1(&input, width as usize, lines, false)
    );

    println!(
        "Result 2: {}",
        find_trailhead_1(&input, width as usize, lines, true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let input = input
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10))
            .collect::<Vec<u32>>();
        assert_eq!(36, find_trailhead_1(&input, 8, 8, false));
    }

    #[test]
    fn test2() {
        let input = "0......
1......
2......
3......
4......
5......
6789...";
        let input = input
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10))
            .collect::<Vec<u32>>();
        assert_eq!(1, find_trailhead_1(&input, 7, 7, false));
    }

    #[test]
    fn test3() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let input = input
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10))
            .collect::<Vec<u32>>();
        assert_eq!(2, find_trailhead_1(&input, 7, 7, false));
    }

    #[test]
    fn test4() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let input = input
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10))
            .collect::<Vec<u32>>();
        assert_eq!(4, find_trailhead_1(&input, 7, 7, false));
    }

    #[test]
    fn test5() {
        let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        let input = input
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10))
            .collect::<Vec<u32>>();
        assert_eq!(3, find_trailhead_1(&input, 7, 7, false));
    }
}
