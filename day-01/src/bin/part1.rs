use std::fs;

fn get_value_from_line(input_line: String) -> u32 {
    let mut digits: Vec<char> = Vec::new();
    for input_char in input_line.chars() {
        if input_char.is_digit(10) {
            digits.push(input_char);
        }
    }
    if digits.len() < 1 {
        println!("{}", digits.into_iter().collect::<String>());
        panic!("Input line has no digits");
    }
    let mut output_str = String::new();
    output_str.push(digits.first().expect("Missing digit").clone());
    output_str.push(digits.last().expect("Missing digit").clone());
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
