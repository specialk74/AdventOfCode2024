use std::fs::read_to_string;

struct Garden {
    x: isize,
    y: isize,
    visited: bool,
    fences: usize,
}

impl Garden {
    fn new(x: isize, y: isize) -> Self {
        Garden {
            x,
            y,
            visited: false,
            fences: 0,
        }
    }
}

struct Group {
    positions: Vec<Garden>,
}

impl Group {
    fn new() -> Self {
        Group {
            positions: Vec::new(),
        }
    }
    fn add(&mut self, x: isize, y: isize) {
        self.positions.push(Garden::new(x, y));
    }
}

struct Input {
    input: String,
    width: usize,
    height: usize,
    groups: Vec<Group>,
}

impl Input {
    fn has_child(&self, garden: &mut Garden, letter: char, x: isize, y: isize) -> bool {
        let new_x = garden.x + x;
        let new_y = garden.y + y;

        if new_x < 0 || new_y < 0 {
            garden.fences += 1;
            // println!(
            //     "x: {} | y: {} | new_x: {} < 0 | new_y: {} < 0 | fences: {} | return false",
            //     x, y, new_x, new_y, garden.fences
            // );
            return false;
        }
        if new_x >= self.width as isize || new_y >= self.height as isize {
            garden.fences += 1;
            // println!(
            //     "x: {} | y: {} | new_x: {} > {} | new_y: {} > {} | fences: {} | return false",
            //     x, y, new_x, self.width, new_y, self.height, garden.fences
            // );
            return false;
        }
        let index = (new_y * self.width as isize + new_x) as usize;
        let car = self.input.chars().nth(index).unwrap();
        if car == '.' {
            // println!(
            //     "x: {} | y: {} | new_x: {} | new_y: {} | trovato '.' | fences: '{}' | return false",
            //     x, y, new_x, new_y, garden.fences
            // );
            return false;
        }

        if car != letter {
            garden.fences += 1;
            // println!(
            //     "x: {} | y: {} | new_x: {} | new_y: {} | trovato '{}' while expecting {} | fences: {} | return false",
            //     x, y, new_x, new_y, car, letter, garden.fences
            // );
            return false;
        }

        // println!(
        //     "x: {} | y: {} | new_x: {} | new_y: {} | letter {} | fences: {} | return true",
        //     x, y, new_x, new_y, letter, garden.fences
        // );
        true
    }
}

fn fence_1(input: &str, width: usize, height: usize) -> usize {
    let input = input.trim().replace("\n", "").replace("\r", "");
    let mut input = Input {
        input,
        width,
        height,
        groups: Vec::new(),
    };

    // println!("\nPrima input: {:?}\n", input.input);

    for position in 0..width * height {
        let car = input.input.chars().nth(position).unwrap();
        if car != '.' && car != ';' {
            let mut group = Group::new();
            group.add(
                position as isize % width as isize,
                position as isize / width as isize,
            );
            let mut childs = true;
            while childs {
                let mut new_positions = Vec::new();
                childs = false;
                for garden in group.positions.iter_mut() {
                    if garden.visited {
                        continue;
                    }
                    for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                        if input.has_child(garden, car, dir.0, dir.1) {
                            new_positions.push(Garden::new(garden.x + dir.0, garden.y + dir.1));
                        }
                    }
                    if !new_positions.is_empty() {
                        childs = true;
                    }
                    garden.visited = true;
                    let position = garden.x + garden.y * width as isize;
                    // println!(
                    //     "input: {:?} | fence: {} | position: {}\n",
                    //     input.input, garden.fences, position
                    // );
                    input
                        .input
                        .replace_range(position as usize..(position as usize + 1), ".");
                }
                for position in new_positions.iter() {
                    if group
                        .positions
                        .iter()
                        .any(|g| g.x == position.x && g.y == position.y)
                    {
                        continue;
                    }
                    group.add(position.x, position.y);
                }
            }
            for garden in group.positions.iter_mut() {
                let position = garden.x + garden.y * width as isize;
                input
                    .input
                    .replace_range(position as usize..(position as usize + 1), ";");
            }
            input.groups.push(group);
        }
    }

    // println!("Dopo input:  {:?}", input.input);
    input
        .groups
        .iter()
        .map(|group| {
            let mut fences = 0;
            for garden in group.positions.iter() {
                fences += garden.fences;
            }
            fences * group.positions.len()
        })
        .sum::<usize>()
}

fn fence_2(input: &str, width: usize, height: usize) -> usize {
    0
}

fn main() {
    let values =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-12/src/input.txt").unwrap();
    let mut width = 0;
    if let Some(line) = values.lines().next() {
        width = line.len();
    }

    println!(
        "Result 1: {:?}",
        fence_1(values.as_str(), width, values.lines().count())
    );
    println!(
        "Result 2: {:?}",
        fence_2(values.as_str(), width, values.lines().count())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let mut width = 0;
        if let Some(line) = input.lines().next() {
            width = line.len();
        }
        assert_eq!(140, fence_1(input, width, input.lines().count()));
    }

    #[test]
    fn test2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let mut width = 0;
        if let Some(line) = input.lines().next() {
            width = line.len();
        }
        assert_eq!(1930, fence_1(input, width, input.lines().count()));
    }
}
