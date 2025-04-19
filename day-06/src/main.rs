use std::{collections::HashMap, fs::read_to_string};
use strum_macros::EnumIter; // Required for deriving EnumIter

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Copy, Clone)] // Ensure EnumIter, PartialEq, Eq, Hash, Copy, and Clone are derived
enum Verso {
    Up,
    Down,
    Left,
    Right,
}

fn guard_1(input: &str, witdh: usize, length: usize) -> usize {
    let mut input = input.replace('\n', "");
    let cursor = input.find('^').unwrap();
    let mut x = (cursor % witdh) as i32;
    let mut y = (cursor / witdh) as i32;
    let mut steps = 0;
    let mut verso = Verso::Up;
    loop {
        if x >= witdh as i32 || x < 0 {
            return steps;
        }
        if y >= length as i32 || y < 0 {
            return steps;
        }
        let pos = (y * witdh as i32 + x) as usize;
        //println!("x: {}, y: {}, steps: {}, pos: {}", x, y, steps, pos);
        let car = input.chars().nth(pos).unwrap();
        //println!("car: {}", car);
        if car == '#' {
            match verso {
                Verso::Up => {
                    x += 1;
                    y += 1;
                    verso = Verso::Right;
                }
                Verso::Down => {
                    x -= 1;
                    y -= 1;
                    verso = Verso::Left;
                }
                Verso::Left => {
                    y -= 1;
                    x += 1;
                    verso = Verso::Up;
                }
                Verso::Right => {
                    y += 1;
                    x -= 1;
                    verso = Verso::Down;
                }
            }
        } else {
            input.replace_range(pos..pos + 1, "X");
            match verso {
                Verso::Up => {
                    y -= 1;
                }
                Verso::Down => {
                    y += 1;
                }
                Verso::Left => {
                    x -= 1;
                }
                Verso::Right => {
                    x += 1;
                }
            }
            if car != 'X' {
                steps += 1;
            }
        }
    }
    0
}

struct Replace<'a> {
    input: &'a mut [u8],
    pos: usize,
    car: u8,
    debug: usize,
    witdh: usize,
    length: usize,
    touched: usize,
    replace_car: u8,
    last_pos: usize,
}
impl<'a> Replace<'a> {
    fn new(input: &'a mut [u8], debug: usize, witdh: usize, length: usize) -> Self {
        Self {
            input,
            pos: 0,
            car: b'.',
            debug,
            witdh,
            length,
            touched: 0,
            replace_car: b'5',
            last_pos: 0,
        }
    }
    fn replace(&mut self, set_car: u8) -> Option<usize> {
        if self.car == self.replace_car {
            self.input[self.pos] = set_car;
            if self.debug >= 1 {
                let x = (self.pos % self.witdh) as i32;
                let y = (self.pos / self.witdh) as i32;

                println!(
                    "\t\tExit for {} @ {}:[{},{}] -> {}",
                    self.car as char, self.pos, x, y, self.touched
                );
                self.pretty_print();
            }
            return Some(self.touched);
        }
        if self.debug >= 2 {
            let x = (self.last_pos % self.witdh) as i32;
            let y = (self.last_pos / self.witdh) as i32;
            println!(
                "\t\tReplace '{}' -> '{}' at last_pos: {}:[{},{}]",
                self.input[self.last_pos] as char, self.replace_car as char, self.last_pos, x, y
            );
        }
        self.input[self.last_pos] = self.replace_car;
        None
    }

    fn pretty_print(&mut self) {
        let output = String::from_utf8_lossy(self.input);
        let mut output = output.into_owned();
        for i in 1..self.length {
            output.insert(i * self.witdh + i - 1, '\n');
        }
        println!("{}", output);
    }
}

fn stuck(input: &mut [u8], witdh: usize, length: usize, pos_new_start: usize) -> usize {
    let mut verso = Verso::Up;
    let debug = 0;

    let mut replaced = Replace::new(input, debug, witdh, length);
    replaced.pos = pos_new_start;

    let dict_y = HashMap::from([
        (Verso::Up, (1, 1, Verso::Right)),
        (Verso::Down, (-1, -1, Verso::Left)),
        (Verso::Left, (1, -1, Verso::Up)),
        (Verso::Right, (-1, 1, Verso::Down)),
    ]);

    let dict_x = HashMap::from([
        (Verso::Up, (b'1', 0, -1)),
        (Verso::Down, (b'2', 0, 1)),
        (Verso::Left, (b'3', -1, 0)),
        (Verso::Right, (b'4', 1, 0)),
    ]);

    loop {
        replaced.car = replaced.input[replaced.pos];
        if debug >= 3 {
            let x = (replaced.pos % replaced.witdh) as i32;
            let y = (replaced.pos / replaced.witdh) as i32;

            println!(
                "\t\tx: {}, y: {}, pos: {}, car: [{}], verso: {:?}",
                x, y, replaced.pos, replaced.car as char, verso
            );
        }

        let (new_x, new_y) = match replaced.car {
            b'#' => {
                let (x, y, new_verso) = &dict_y[&verso];
                verso = *new_verso;
                (x, y)
            }
            b'O' => {
                if debug >= 3 {
                    println!("Touched: {}", replaced.pos);
                }
                replaced.touched = 1;
                let (x, y, new_verso) = &dict_y[&verso];
                verso = *new_verso;
                (x, y)
            }
            b'^' => {
                let (_, x, y) = &dict_x[&verso];
                (x, y)
            }
            _ => {
                replaced.last_pos = replaced.pos;

                let (replace_car, x, y) = &dict_x[&verso];

                replaced.replace_car = *replace_car;
                if let Some(touched) = replaced.replace(b'X') {
                    return touched;
                }
                (x, y)
            }
        };

        let x = (replaced.pos % replaced.witdh) as i32;
        let y = (replaced.pos / replaced.witdh) as i32;

        if debug >= 3 {
            println!(
                "\t, x: {}, y: {}, pos: {}, new_x: {}, new_y: {}",
                x, y, replaced.pos, new_x, new_y
            );
        }

        if (new_x == &1 && x >= witdh as i32 - 1)
            || (new_x == &-1 && x == 0)
            || (new_y == &1 && y >= length as i32 - 1)
            || (new_y == &-1 && y == 0)
        {
            if debug >= 3 {
                println!(
                    "\t\tExit for pos:{} > len: {}",
                    replaced.pos,
                    replaced.input.len()
                );
                replaced.pretty_print();
            }
            return 0;
        }

        replaced.pos = ((replaced.pos as isize) + (witdh as isize * new_y + new_x)) as usize;
    }
    0
}

fn guard_2(input: &str, witdh: usize, length: usize) -> usize {
    let mut stuck_sum = 0;
    // Remove new lines and replace ^ with .
    let input = input.replace('\n', "").into_bytes();
    let pos_new_start = input.iter().position(|&c| c == b'^').unwrap();
    for (pos_new_obstacle, car) in input.iter().enumerate() {
        // If the character is an obstacle already, skip it
        if *car == b'#' || *car == b'^' {
            continue;
        }
        // println!(
        //     "\n\npos_new_obstacle: {}, car: [{}]",
        //     pos_new_obstacle, *car as char
        // );
        // Replace the free spot with a new obstacle
        let mut new_input = input.clone();
        new_input[pos_new_obstacle] = b'O';
        // println!(
        //     "Stuck sum: {}, pos_new_obstacle:{}",
        //     stuck_sum, pos_new_obstacle
        // );

        let ret = stuck(&mut new_input, witdh, length, pos_new_start);
        //if ret == 1 {
        //    println!("sum: {} - pos: {}\n\n\n", stuck_sum, pos_new_obstacle);
        //}
        stuck_sum += ret;

        //println!("\tverso: {:?}, stuck: {}", verso, stuck_sum);
        // if stuck_sum > 0 {
        //     println!("\tStuck sum: {}", stuck_sum);
        //     return 0;
        // }
    }
    stuck_sum
}

fn main() {
    let values =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-06/src/input.txt").unwrap();
    let mut width = 0;
    if let Some(line) = values.lines().next() {
        width = line.len();
    }
    println!(
        "Result 1: {:?}",
        guard_1(values.as_str(), width, values.lines().count())
    );
    println!(
        "Result 2: {:?}",
        guard_2(values.as_str(), width, values.lines().count())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, guard_1(input, 10, 10));
    }

    #[test]
    fn test2() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(6, guard_2(input, 10, 10));
    }

    #[test]
    fn test3() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[63] = b'O';
        assert_eq!(1, stuck(&mut input, 10, 10, 64));
    }

    #[test]
    fn test4() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[76] = b'O';
        assert_eq!(1, stuck(&mut input, 10, 10, 64));
    }

    #[test]
    fn test5() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[77] = b'O';
        assert_eq!(1, stuck(&mut input, 10, 10, 64));
    }

    #[test]
    fn test6() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[81] = b'O';
        assert_eq!(1, stuck(&mut input, 10, 10, 64));
    }

    #[test]
    fn test7() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[83] = b'O';
        assert_eq!(1, stuck(&mut input, 10, 10, 64));
    }

    #[test]
    fn test8() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[97] = b'O';
        assert_eq!(1, stuck(&mut input, 10, 10, 64));
    }

    #[test]
    fn test9() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut input = input.replace('\n', "").into_bytes();
        input[99] = b'O';
        assert_eq!(0, stuck(&mut input, 10, 10, 64));
    }

    // #[test]
    // fn test10() {
    //     let values =
    //         read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-06/src/input.txt").unwrap();
    //     let mut width = 0;
    //     if let Some(line) = values.lines().next() {
    //         width = line.len();
    //     }
    //     let mut input = values.replace('\n', "").into_bytes();
    //     input[1621] = b'O';
    //     let pos_new_start = input.iter().position(|&c| c == b'^').unwrap();
    //     assert!(stuck(
    //         &mut input,
    //         width,
    //         values.lines().count(),
    //         pos_new_start
    //     ));
    // }
}
