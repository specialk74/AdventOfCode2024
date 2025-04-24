use std::fs::read_to_string;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Antidote {
    position_up: Option<Position>,
    position_down: Option<Position>,
}

#[derive(Debug)]
struct Antenna {
    position: Position,
    antidotes: Vec<Antidote>,
}

impl Antenna {
    fn new(x: usize, y: usize) -> Self {
        Antenna {
            position: Position { x, y },
            antidotes: Vec::new(),
        }
    }

    fn add_same_antenna(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let diff_x = if self.position.x > x {
            self.position.x - x
        } else {
            x - self.position.x
        };
        let diff_y = y - self.position.y;

        let mut position_up: Option<Position> = None;
        let mut position_down: Option<Position> = None;

        let x_position_up = if self.position.x > x {
            (self.position.x + diff_x) as isize
        } else {
            self.position.x as isize - diff_x as isize
        };

        let y_position_up = self.position.y as isize - diff_y as isize;

        if x_position_up >= 0 && x_position_up < width as isize && y_position_up >= 0 {
            position_up = Some(Position {
                x: x_position_up as usize,
                y: y_position_up as usize,
            });
        }

        let x_position_down = if self.position.x > x {
            x as isize - diff_x as isize
        } else {
            (x + diff_x) as isize
        };

        let y_position_down = y + diff_y;
        if x_position_down >= 0 && x_position_down < width as isize && y_position_down < height {
            position_down = Some(Position {
                x: x_position_down as usize,
                y: y_position_down,
            });
        }
        let a = Antidote {
            position_up,
            position_down,
        };
        self.antidotes.push(a);
    }

    fn add_same_armonica(&mut self, mut x: usize, mut y: usize, width: usize, height: usize) {
        let diff_x = if self.position.x > x {
            self.position.x - x
        } else {
            x - self.position.x
        };
        let diff_y = y - self.position.y;

        let mut my_position_x = self.position.x;
        let mut my_position_y = self.position.y;

        let original_up_x = my_position_x;
        let original_up_y = my_position_y;
        let original_down_x = x;
        let original_down_y = y;

        let position_up = Position {
            x: original_up_x,
            y: original_up_y,
        };
        let position_down = Position {
            x: original_down_x,
            y: original_down_y,
        };

        self.antidotes.push(Antidote {
            position_up: Some(position_up),
            position_down: Some(position_down),
        });

        loop {
            let mut position_up: Option<Position> = None;
            let x_position_up = if my_position_x > x {
                (my_position_x + diff_x) as isize
            } else {
                my_position_x as isize - diff_x as isize
            };

            let y_position_up = my_position_y as isize - diff_y as isize;

            if x_position_up >= 0 && x_position_up < width as isize && y_position_up >= 0 {
                position_up = Some(Position {
                    x: x_position_up as usize,
                    y: y_position_up as usize,
                });
            }

            let mut position_down: Option<Position> = None;
            let x_position_down = if my_position_x > x {
                x as isize - diff_x as isize
            } else {
                (x + diff_x) as isize
            };

            let y_position_down = y + diff_y;
            if x_position_down >= 0 && x_position_down < width as isize && y_position_down < height
            {
                position_down = Some(Position {
                    x: x_position_down as usize,
                    y: y_position_down,
                });
            }

            if position_up.is_none() && position_down.is_none() {
                break;
            }

            if let Some(Position { x: new_x, y: new_y }) = position_up {
                my_position_x = new_x;
                my_position_y = new_y;
            }

            if let Some(Position { x: new_x, y: new_y }) = position_down {
                x = new_x;
                y = new_y;
            }

            let a = Antidote {
                position_up,
                position_down,
            };
            self.antidotes.push(a);
        }
    }
}

#[derive(Debug)]
struct Map {
    input: String,
    height: usize,
    antennas: Vec<Antenna>,
    width: usize,
}
impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().next().map_or(0, |line| line.len());
        let height = input.lines().count();

        let input_clone = input.trim().replace('\n', "");

        Self {
            input: input_clone,
            antennas: Vec::new(),
            width,
            height,
        }
    }

    fn search_antenna(&mut self, function: fn(&mut Antenna, usize, usize, usize, usize)) {
        for (i, c1) in self.input.chars().enumerate() {
            if c1 != '.' {
                let x = i % self.width;
                let y = i / self.width;
                self.antennas.push(Antenna::new(x, y));

                for (j, c2) in self.input.chars().enumerate() {
                    if j <= i {
                        continue;
                    }
                    if c1 == c2 {
                        let x2 = j % self.width;
                        let y2 = j / self.width;
                        function(
                            self.antennas.last_mut().unwrap(),
                            x2,
                            y2,
                            self.width,
                            self.height,
                        );
                    }
                }
            }
        }
    }
}

fn search_antidote_1(input: &str) -> usize {
    let mut input_clone: Vec<char> = input.trim().replace('\n', "").chars().collect();
    let mut map = Map::new(input);
    map.search_antenna(Antenna::add_same_antenna);

    for a in map.antennas {
        for antitode in a.antidotes {
            /* println!(
                "Antidote Up found at: {:?} related to {:?}",
                antitode.position_up, a.position
            ); */
            if let Some(position_up) = antitode.position_up {
                input_clone[position_up.x + position_up.y * map.width] = '-';
            }
            /* println!(
                "Antidote down found at: {:?} related to {:?}",
                antitode.position_down, a.position
            ); */
            if let Some(position_down) = antitode.position_down {
                input_clone[position_down.x + position_down.y * map.width] = '-';
            }
        }
    }

    input_clone.iter().filter(|&&x| x == '-').count()
}
fn search_antidote_2(input: &str) -> usize {
    let mut input_clone: Vec<char> = input.trim().replace('\n', "").chars().collect();
    let mut map = Map::new(input);
    map.search_antenna(Antenna::add_same_armonica);

    for a in map.antennas {
        for antitode in a.antidotes {
            if let Some(position_up) = antitode.position_up {
                input_clone[position_up.x + position_up.y * map.width] = '-';
            }
            if let Some(position_down) = antitode.position_down {
                input_clone[position_down.x + position_down.y * map.width] = '-';
            }
        }
    }

    println!("Antidote: {:?}", input_clone.iter().collect::<String>());
    input_clone.iter().filter(|&&x| x == '-').count()
}

fn main() {
    let values =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-08/src/input.txt").unwrap();

    println!("Result 1: {:?}", search_antidote_1(values.as_str()));
    println!("Result 2: {:?}", search_antidote_2(values.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = search_antidote_1(input);
        assert_eq!(14, result);
    }

    #[test]
    fn test2() {
        let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........";
        let result = search_antidote_1(input);
        assert_eq!(4, result);
    }

    #[test]
    fn test3() {
        let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";
        let result = search_antidote_1(input);
        assert_eq!(4, result);
    }

    #[test]
    fn test4() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = search_antidote_2(input);
        assert_eq!(34, result);
    }

    #[test]
    fn test5() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let result = search_antidote_2(input);
        assert_eq!(9, result);
    }
}
