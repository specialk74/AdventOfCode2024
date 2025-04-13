use std::{fs::read_to_string, process::exit};

fn find_xmas_by_dir(
    input: &str,
    width: i32,
    lenght: i32,
    start: i32,
    vertical_direction: i32,
    horizontal_direction: i32,
) -> usize {
    let name = "find_xmas_by_dir";
    let debug = true;

    if debug {
        println!(
            "{} -> start: {}, vertical_direction: {}, horizontal_direction: {}",
            name, start, vertical_direction, horizontal_direction
        );
    }
    if vertical_direction == 0 && horizontal_direction == 0 {
        exit(1);
    }

    let num_letters = 3;
    if vertical_direction > 0 && ((start / width) % lenght) > num_letters {
        if debug {
            println!(
                "{} -> vertical_direction > 0 && (({} / {}:{}) % {}: {}) > {}",
                name,
                start,
                width,
                start / width,
                lenght,
                (start / width) % lenght,
                num_letters
            );
        }
        return 0;
    }
    if vertical_direction < 0 && (start < width * num_letters) {
        if debug {
            println!(
                "{} -> vertical_direction < 0 && ({} < {} * {}: {})",
                name,
                start,
                width,
                num_letters,
                width * num_letters
            );
        }
        return 0;
    }
    if horizontal_direction == 1 && (start % width) + num_letters > width {
        if debug {
            println!(
                "{} -> horizontal_direction == 1 && ({} % {}: {}) + {} > {}",
                name,
                start,
                width,
                start % width,
                num_letters,
                width
            );
        }
        return 0;
    }
    if horizontal_direction == -1 && (start % width) < num_letters {
        if debug {
            println!(
                "{} -> horizontal_direction == -1 && ({} % {}: {}) > {}",
                name,
                start,
                width,
                start % width,
                num_letters
            );
        }
        return 0;
    }

    let position = (start + vertical_direction + horizontal_direction) as usize;
    //println!("{} -> input.chars().nth({})", name, position);
    if input.chars().nth(position) != Some('M') {
        if debug {
            println!(
                "{} -> input.chars().nth({} -> {}): {:?} expcted 'M'",
                name,
                start,
                position,
                input.chars().nth(position)
            );
        }
        return 0;
    }
    let position = (start + 2 * vertical_direction + 2 * horizontal_direction) as usize;
    //println!("{} -> input.chars().nth({})", name, position);
    if input.chars().nth(position) != Some('A') {
        if debug {
            println!(
                "{} -> input.chars().nth({} -> {}): {:?} expected 'A'",
                name,
                start,
                position,
                input.chars().nth(position)
            );
        }
        return 0;
    }
    let position = (start + 3 * vertical_direction + 3 * horizontal_direction) as usize;
    //println!("{} -> input.chars().nth({})", name, position);
    if input.chars().nth(position) != Some('S') {
        if debug {
            println!(
                "{} -> input.chars().nth({} -> {}): {:?} expected 'S'",
                name,
                start,
                position,
                input.chars().nth(position)
            );
        }
        return 0;
    }
    if debug {
        println!("{} -> trovato: {}", name, start);
    }
    1
}

fn find_xmas(input: &str, width: i32, lenght: i32) -> usize {
    let mut result = 0;
    let debug = true;
    for (index, c) in input.chars().enumerate() {
        if c == 'X' {
            if debug {
                println!();
            }
            let mut partial = 0;
            let val = find_xmas_by_dir(input, width, lenght, index as i32, 0, 1);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, 0, -1);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, width, 0);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, -width, 0);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, width, 1);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, width, -1);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, -width, 1);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            let val = find_xmas_by_dir(input, width, lenght, index as i32, -width, -1);
            if val != 0 {
                result += val;
                partial += 1;
                //continue;
            }
            if partial > 0 && debug {
                println!("find_xmas_dir_6 -> index: {}, partial: {}", index, partial);
            }
        }
    }
    result
}

fn main() {
    let input = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";

    assert_eq!(18, find_xmas(input, 10, 10));

    let content =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-04/src/input.txt").unwrap();
    let mut width = 0;
    if let Some(line) = content.lines().next() {
        width = line.len() as i32;
    }
    println!("width: {} - length: {}", width, content.lines().count());
    println!(
        "Result 1: {}",
        find_xmas(&content, width, content.lines().count() as i32)
    );
}
