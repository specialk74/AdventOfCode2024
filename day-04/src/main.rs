use std::{fs::read_to_string, process::exit};

fn find_xmas_by_dir(
    input: &str,
    width: i32,
    lenght: i32,
    start: i32,
    vertical_direction: i32,
    horizontal_direction: i32,
    debug: bool,
) -> usize {
    let name = "find_xmas_by_dir";

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
    if vertical_direction > 0 && num_letters > (((lenght * width) - start) / width) {
        if debug {
            println!(
                "{} -> vertical_direction > 0 && {} > {}|{}|{})",
                name,
                num_letters,
                lenght * width,
                (lenght * width) - start,
                (((lenght * width) - start) / width)
            );
        }
        return 0;
    }
    if vertical_direction < 0 && (start < width * num_letters) {
        if debug {
            println!(
                "{} -> vertical_direction < 0 && ({} < {} * {} = {})",
                name,
                start,
                width,
                num_letters,
                width * num_letters
            );
        }
        return 0;
    }
    if horizontal_direction == 1 && (start % width) + num_letters >= width {
        if debug {
            println!(
                "{} -> horizontal_direction == 1 && ({} % {} = {}) + {} > {}",
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
                "{} -> horizontal_direction == -1 && ({} % {} = {}) < {}",
                name,
                start,
                width,
                start % width,
                num_letters
            );
        }
        return 0;
    }

    let params = ['X', 'M', 'A', 'S'];
    for (index, c) in params.iter().enumerate() {
        let position = (start
            + index as i32 * vertical_direction
            + index as i32 * horizontal_direction) as usize;
        //println!("{} -> input.chars().nth({})", name, position);
        if input.chars().nth(position) != Some(*c) {
            if debug {
                println!(
                    "{} -> input.chars().nth({} -> {}) = {:?} expected '{}'",
                    name,
                    start,
                    position,
                    input.chars().nth(position),
                    c
                );
            }
            return 0;
        }
    }
    1
}

fn find_xmas(input: &str, width: i32, lenght: i32, debug: bool) -> usize {
    let mut result = 0;
    let params = [
        (0, 1),
        (0, -1),
        (width, 0),
        (-width, 0),
        (width, 1),
        (width, -1),
        (-width, 1),
        (-width, -1),
    ];
    for (index, c) in input.chars().enumerate() {
        if c == 'X' {
            if debug {
                println!();
            }
            let mut partial = 0;
            for param in params.iter() {
                let val =
                    find_xmas_by_dir(input, width, lenght, index as i32, param.0, param.1, debug);
                if val != 0 {
                    result += val;
                    partial += 1;
                }
            }

            if partial > 0 && debug {
                println!("find_xmas -> index: {}, partial: {}", index, partial);
            }
        } else if c == '\n' {
            println!("Error: unexpected new line");
            exit(1);
        }
    }
    result
}

fn find_mas_by_dir(input: &str, width: i32, lenght: i32, start: i32, debug: bool) -> usize {
    let name = "find_mas_by_dir";

    if debug {
        println!("{} -> start: {}", name, start);
    }

    let pos_x = start % width;
    let pos_y = start / width;
    if pos_x == 0 || pos_x == width - 1 {
        if debug {
            println!("{} -> pos_x == 0 || pos_x == width - 1", name);
        }
        return 0;
    }
    if pos_y == 0 || pos_y == lenght - 1 {
        if debug {
            println!("{} -> pos_y == 0 || pos_y == lenght - 1", name);
        }
        return 0;
    }
    // Posizione in alto a sx
    let pos_up_sx = (pos_y - 1) * width + pos_x - 1;
    // Posizione in alto a dx
    let pos_up_dx = (pos_y - 1) * width + pos_x + 1;
    // Posizione in basso a sx
    let pos_down_sx = (pos_y + 1) * width + pos_x - 1;
    // Posizione in basso a dx
    let pos_down_dx = (pos_y + 1) * width + pos_x + 1;

    let char_at_up_sx = input.chars().nth(pos_up_sx as usize);
    let char_at_up_dx = input.chars().nth(pos_up_dx as usize);
    let char_at_down_sx = input.chars().nth(pos_down_sx as usize);
    let char_at_down_dx = input.chars().nth(pos_down_dx as usize);

    let source = [
        char_at_up_sx,
        char_at_up_dx,
        char_at_down_sx,
        char_at_down_dx,
    ];

    if source == [Some('M'), Some('M'), Some('S'), Some('S')] {
        return 1;
    }
    if source == [Some('S'), Some('S'), Some('M'), Some('M')] {
        return 1;
    }
    if source == [Some('S'), Some('M'), Some('S'), Some('M')] {
        return 1;
    }
    if source == [Some('M'), Some('S'), Some('M'), Some('S')] {
        return 1;
    }

    0
}

fn find_mas(input: &str, width: i32, lenght: i32, debug: bool) -> usize {
    let mut result = 0;
    for (index, c) in input.chars().enumerate() {
        if c == 'A' {
            if debug {
                println!();
            }
            let mut partial = 0;
            let val = find_mas_by_dir(input, width, lenght, index as i32, debug);
            if val != 0 {
                result += val;
                partial += 1;
            }
            if partial > 0 && debug {
                println!("find_xmas -> index: {}, partial: {}", index, partial);
            }
        } else if c == '\n' {
            println!("Error: unexpected new line");
            exit(1);
        }
    }
    result
}

fn main() {
    let content =
        read_to_string("/Users/specialk74/Rust/AdventOfCode2024/day-04/src/input.txt").unwrap();
    let content_str = content.trim().replace("\n", "").to_string();
    let mut width = 0;
    if let Some(line) = content.lines().next() {
        width = line.len() as i32;
    }
    println!("width: {} - length: {}", width, content.lines().count());
    // println!("content: {}", content_str);
    println!(
        "Result 1: {}",
        find_xmas(&content_str, width, content.lines().count() as i32, false)
    );

    println!(
        "Result 2: {}",
        find_mas(&content_str, width, content.lines().count() as i32, false)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";
        assert_eq!(18, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test2() {
        let input = "XMAS------------------------------------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test3() {
        let input = "X---------M---------A---------S---------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test4() {
        let input = "X----------M----------A----------S------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test5() {
        let input = "---------X---------M---------A---------S------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test6() {
        let input = "---------X--------M--------A--------S---------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test7() {
        let input = "------SAMX------------------------------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test8() {
        let input = "---------------------------------------------------------------------S---------A---------M---------X";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test9() {
        let input = "------------------------------------------------------------------S----------A----------M----------X";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test10() {
        let input = "------------------------------------------------------------------------------------------------SAMX";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test11() {
        let input = "------------------------------------------------------------------------------------------XMAS------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test12() {
        let input = "---------------------------------------------------------------S--------A--------M--------X---------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test13() {
        let input = "------------------------------------------------------------S---------A---------M---------X---------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test14() {
        let input = "S---------A---------M---------X---------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test15() {
        let input = "S----------A----------M----------X------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test16() {
        let input = "SAMX------------------------------------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test17() {
        let input = "------XMAS------------------------------------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test18() {
        let input = "---------S--------A--------M--------X---------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test19() {
        let input = "---------S---------A---------M---------X------------------------------------------------------------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test20() {
        let input = "---------------------------------------------------------------------X---------M---------A---------S";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test21() {
        let input = "------------------------------------------------------------------------------------------------XMAS";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test22() {
        let input = "------------------------------------------------------------------X----------M----------A----------S";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test23() {
        let input = "------------------------------------------------------------------------------------------SAMX------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test24() {
        let input = "------------------------------------------------------------X---------M---------A---------S---------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test25() {
        let input = "---------------------------------------------------------------X--------M--------A--------S---------";
        assert_eq!(1, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test26() {
        let input = "SAMX--XMASAA------AAM-M----M-MX--X--X--X--------------------X--X--X--XM-M----M-MAA------AASAMX--XMAS";
        assert_eq!(12, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test27() {
        let input = "XMAS--SAMXMM------MMA-A----A-AS--S--S--S--------------------S--S--S--SA-A----A-AMM------MMXMAS--SAMX";
        assert_eq!(12, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test28() {
        let input = "-------XMAS-----------------------------------------------------------------------------------------";
        assert_eq!(0, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test29() {
        let input = "-------SAMX-----------------------------------------------------------------------------------------";
        assert_eq!(0, find_xmas(input, 10, 10, false));
    }
    #[test]
    fn test30() {
        let input = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";
        assert_eq!(9, find_mas(input, 10, 10, true));
    }
    #[test]
    fn test31() {
        let input = "M-S--------A--------M-S-----------------------------------------------------------------------------";
        assert_eq!(1, find_mas(input, 10, 10, true));
    }
}
