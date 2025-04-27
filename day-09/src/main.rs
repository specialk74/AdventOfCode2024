use std::fs::read_to_string;

fn created_disk_map(input: &str) -> Vec<i64> {
    let input = input.trim().replace("\n", "");
    let mut disk_map = Vec::new();

    let mut iter = input.chars().peekable();

    let mut count = 0_i64;
    // Take the first character and used it like len for count
    while let Some(len) = iter.next() {
        let len = len as u8 - b'0';
        disk_map.extend(vec![count; len as usize]);
        count += 1;

        // Take the second character and used it like len for gap
        // If the next character is None, no gap is created and the next while loop breaks
        if let Some(gap) = iter.next() {
            let gap = gap as u8 - b'0';
            disk_map.extend(vec![-1; gap as usize]);
        }
    }

    disk_map
}

fn disk_map_1(input: &str) -> usize {
    let mut disk_map = created_disk_map(input);

    // Search for the first -1 until None from left to right
    while let Some(free_space) = disk_map.iter().position(|&x| x == -1) {
        // Take last value from disk_map from right to left
        while let Some(val) = disk_map.pop() {
            // If val is -1, continue to next iteration
            if val == -1 {
                continue;
            }

            // If the position of the free space was outside the disk_map,
            // add new value at the botton of the disk_map
            if free_space >= disk_map.len() {
                // If free_space is greater than disk_map, break
                disk_map.push(val);
                break;
            }

            // Take the last value and repositioning instead of -1
            disk_map[free_space] = val;
            break;
        }
    }

    disk_map
        .iter()
        .enumerate()
        .map(|(i, &x)| i as i64 * x)
        .sum::<i64>() as usize
}

#[derive(Debug)]
struct Number {
    fileno: i64,
    pos: usize,
    len: usize,
}
impl Number {
    fn move_number(number: &Number, new_pos: usize) -> Self {
        Self {
            fileno: number.fileno,
            pos: new_pos,
            len: number.len,
        }
    }

    fn add_gap(num: &Number) -> Self {
        Self {
            fileno: -1,
            pos: num.pos,
            len: num.len,
        }
    }
}

fn disk_map_2(input: &str) -> usize {
    let mut numbers: Vec<Number> = Vec::new();
    let mut gaps: Vec<Number> = Vec::new();
    let mut iter = input.chars().peekable();
    let mut pos = 0;
    let mut fileno = 0;

    // Create double vectors with numbers and gaps
    while let Some(len) = iter.next() {
        let len = len as u8 - b'0';
        numbers.push(Number {
            fileno,
            pos,
            len: len as usize,
        });
        fileno += 1;
        pos += len as usize;

        // Take the second character and used it like len for gap
        // If the next character is None, no gap is created and the next while loop breaks
        if let Some(len) = iter.next() {
            let len = len as u8 - b'0';
            if len > 0 {
                gaps.push(Number {
                    fileno: -1,
                    pos,
                    len: len as usize,
                });
                pos += len as usize;
            }
        }
    }

    // Create two vectors with new_number moved and
    // the index of the number to be removed
    let mut new_numbers: Vec<Number> = Vec::new();
    let mut remover = vec![];
    for (idx_num, number) in numbers.iter().enumerate().rev() {
        if let Some((idx, gap)) = gaps
            .iter()
            .enumerate()
            .find(|(_, gap)| gap.len >= number.len && gap.pos <= number.pos)
        {
            remover.push(idx_num);
            new_numbers.push(Number::move_number(number, gap.pos));

            // Checks if the len of the gap is equal to the len of the number
            if number.len == gap.len {
                // Yes: remove the gap
                gaps.remove(idx);
            } else {
                // No: reduce the len of the gap
                // and increase the pos of the gap
                // to the right
                gaps[idx].len -= number.len;
                gaps[idx].pos += number.len;
            }
            // Add a new gap from the number moved
            gaps.push(Number::add_gap(number));

            // Merge the gaps if they are adjacent
            // This is done by checking if the pos of the previous gap
            // plus the len of the previous gap is equal to the pos of the current gap
            // If they are equal, merge them
            // by adding the len of the current gap to the previous gap
            // and removing the current gap
            // This is done in a loop until no more gaps can be merged
            loop {
                let mut merged = false;
                for idx in 1..gaps.len() {
                    if gaps[idx - 1].pos + gaps[idx - 1].len == gaps[idx].pos {
                        gaps[idx - 1].len += gaps[idx].len;
                        gaps.remove(idx);
                        merged = true;
                        break;
                    }
                }
                if !merged {
                    break;
                }
            }

            gaps.sort_by_key(|f| f.pos);
        }
    }

    // Remove the numbers that were moved
    for idx in remover {
        numbers.remove(idx);
    }

    // Extend the numbers with new_numbers and gaps
    numbers.extend(new_numbers);
    numbers.extend(gaps);
    numbers.sort_by_key(|f| f.pos);

    // Create the disk_map with the numbers
    let mut disk_map: Vec<i64> = vec![];
    for number in &numbers {
        if number.fileno == -1 {
            disk_map.extend(vec![-1; number.len]);
        } else {
            disk_map.extend(vec![number.fileno; number.len]);
        }
    }

    disk_map
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != -1)
        .map(|(i, &x)| i as i64 * x)
        .sum::<i64>() as usize
}

fn main() {
    let content =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-09/src/input.txt").unwrap();

    println!("Result 1: {}", disk_map_1(&content));

    println!("Result 2: {}", disk_map_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "2333133121414131402";
        assert_eq!(1928, disk_map_1(input));
    }

    #[test]
    fn test2() {
        let input = "2333133121414131402";
        assert_eq!(2858, disk_map_2(input));
    }
}
