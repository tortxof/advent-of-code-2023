use std::fs;

struct CharPosition(char, usize);

struct NumberWord(String, usize);

fn get_value_from_line(input_line: String) -> u32 {
    let mut digit_pos: Vec<CharPosition> = Vec::new();
    for find_digit in 1..=9 {
        for match_ in input_line.match_indices(&find_digit.to_string()) {
            digit_pos.push(CharPosition(
                match_.1.chars().collect::<Vec<char>>()[0],
                match_.0,
            ));
        }
    }
    for find_digit in [
        NumberWord(String::from("one"), 1),
        NumberWord(String::from("two"), 2),
        NumberWord(String::from("three"), 3),
        NumberWord(String::from("four"), 4),
        NumberWord(String::from("five"), 5),
        NumberWord(String::from("six"), 6),
        NumberWord(String::from("seven"), 7),
        NumberWord(String::from("eight"), 8),
        NumberWord(String::from("nine"), 9),
    ]
    .into_iter()
    {
        for match_ in input_line.match_indices(&find_digit.0) {
            digit_pos.push(CharPosition(
                find_digit.1.to_string().chars().collect::<Vec<char>>()[0],
                match_.0,
            ));
        }
    }
    digit_pos.sort_by_key(|k| k.1);
    let mut output_str = String::new();
    output_str.push(digit_pos.first().expect("Missing digit").0.clone());
    output_str.push(digit_pos.last().expect("Missing digit").0.clone());
    return output_str.parse::<u32>().expect("Could not parse digits");
}

fn main() {
    let mut calib_values: Vec<u32> = Vec::new();
    let input_str = fs::read_to_string("./input.txt").expect("Could not find file");
    for input_line in input_str.lines() {
        if input_line.len() > 0 {
            calib_values.push(get_value_from_line(input_line.to_string()));
        }
    }
    let mut sum: u32 = 0;
    for calib_value in calib_values {
        sum += calib_value;
    }
    println!("{}", sum);
}
