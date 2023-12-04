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

fn get_row_match_count(row: (bool, bool, bool)) -> usize {
    match row {
        (false, false, false) => 0,
        (true, false, true) => 2,
        _ => 1,
    }
}

fn find_first_digit_pos(lines: &Vec<String>, x_start: isize, y_start: isize) -> (isize, isize) {
    let mut x = x_start;
    loop {
        if !get_char_at_position(&lines, x - 1, y_start).is_digit(10) {
            return (x, y_start);
        }
        x -= 1;
    }
}

fn get_gear(lines: &Vec<String>, x_start: isize, y_start: isize) -> usize {
    let mut values: Vec<usize> = Vec::new();

    if get_char_at_position(&lines, x_start - 1, y_start).is_digit(10) {
        let first_digit_pos = find_first_digit_pos(&lines, x_start - 1, y_start);
        values.push(get_part_number(&lines, first_digit_pos.0, first_digit_pos.1).unwrap());
    }
    if get_char_at_position(&lines, x_start + 1, y_start).is_digit(10) {
        values.push(get_part_number(&lines, x_start + 1, y_start).unwrap());
    }

    let top_row_match_count = get_row_match_count((
        get_char_at_position(&lines, x_start - 1, y_start - 1).is_digit(10),
        get_char_at_position(&lines, x_start, y_start - 1).is_digit(10),
        get_char_at_position(&lines, x_start + 1, y_start - 1).is_digit(10),
    ));
    if top_row_match_count == 2 {
        let first_digit_pos = find_first_digit_pos(&lines, x_start - 1, y_start - 1);
        values.push(get_part_number(&lines, first_digit_pos.0, first_digit_pos.1).unwrap());
        values.push(get_part_number(&lines, x_start + 1, y_start - 1).unwrap());
    } else if top_row_match_count == 1 {
        for x in ((x_start - 1)..=(x_start + 1)).rev() {
            if get_char_at_position(&lines, x, y_start - 1).is_digit(10) {
                let first_digit_pos = find_first_digit_pos(&lines, x, y_start - 1);
                values.push(get_part_number(&lines, first_digit_pos.0, first_digit_pos.1).unwrap());
                break;
            }
        }
    }

    let bottom_row_match_count = get_row_match_count((
        get_char_at_position(&lines, x_start - 1, y_start + 1).is_digit(10),
        get_char_at_position(&lines, x_start, y_start + 1).is_digit(10),
        get_char_at_position(&lines, x_start + 1, y_start + 1).is_digit(10),
    ));
    if bottom_row_match_count == 2 {
        let first_digit_pos = find_first_digit_pos(&lines, x_start - 1, y_start + 1);
        values.push(get_part_number(&lines, first_digit_pos.0, first_digit_pos.1).unwrap());
        values.push(get_part_number(&lines, x_start + 1, y_start + 1).unwrap());
    } else if bottom_row_match_count == 1 {
        for x in ((x_start - 1)..=(x_start + 1)).rev() {
            if get_char_at_position(&lines, x, y_start + 1).is_digit(10) {
                let first_digit_pos = find_first_digit_pos(&lines, x, y_start + 1);
                values.push(get_part_number(&lines, first_digit_pos.0, first_digit_pos.1).unwrap());
                break;
            }
        }
    }

    if values.len() == 2 {
        return values[0] * values[1];
    }

    return 0;
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
    let mut gear_ratio_sum = 0;
    for y in 0..height {
        for x in 0..width {
            if get_char_at_position(&input_lines, x as isize, y as isize) == '*' {
                gear_ratio_sum += get_gear(&input_lines, x as isize, y as isize);
            }
        }
    }
    println!("{gear_ratio_sum}");
}
