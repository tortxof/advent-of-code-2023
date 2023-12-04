use std::fs;

fn char_is_symbol(input: char) -> bool {
    return !(input.is_digit(10) || input == '.');
}

fn get_char_at_position(lines: &Vec<String>, x: isize, y: isize) -> char {
    if x < 0 || y < 0 {
        return '.';
    }
    match lines.get(y as usize) {
        Some(line) => match line.chars().collect::<Vec<char>>().get(x as usize) {
            Some(char_) => return char_.clone(),
            _ => return '.',
        },
        _ => return '.',
    }
}

fn get_part_number(lines: &Vec<String>, x_start: isize, y_start: isize) -> Option<usize> {
    if get_char_at_position(&lines, x_start - 1, y_start).is_digit(10) {
        return None;
    }
    let mut number_str = String::from("");
    number_str.push(get_char_at_position(&lines, x_start, y_start));
    let mut x = x_start;
    loop {
        x += 1;
        let next_char = get_char_at_position(&lines, x, y_start);
        if !next_char.is_digit(10) {
            break;
        }
        number_str.push(next_char);
    }
    let part_number = number_str.parse().unwrap();
    for sym_y in (y_start - 1)..=(y_start + 1) {
        for sym_x in (x_start - 1)..=x {
            if char_is_symbol(get_char_at_position(&lines, sym_x, sym_y)) {
                return Some(part_number);
            }
        }
    }
    return None;
}

fn load_input_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in fs::read_to_string("./input.txt").unwrap().lines() {
        if line.len() == 0 {
            continue;
        }
        lines.push(line.to_string());
    }
    lines
}

fn main() {
    let input_lines = load_input_lines();
    let height = input_lines.len();
    let width = input_lines[0].len();
    let mut part_numbers: Vec<usize> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if get_char_at_position(&input_lines, x as isize, y as isize).is_digit(10) {
                match get_part_number(&input_lines, x as isize, y as isize) {
                    Some(part_number) => part_numbers.push(part_number),
                    _ => (),
                }
            }
        }
    }
    let mut part_number_sum = 0;
    for part_number in part_numbers {
        part_number_sum += part_number;
    }
    println!("{part_number_sum}");
}
