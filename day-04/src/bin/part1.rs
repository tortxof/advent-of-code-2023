use std::fs;

struct Card {
    winning_nums: Vec<i32>,
    own_nums: Vec<i32>,
}

fn load_nums_to_vec(nums_str: &str) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    for num_str in nums_str.trim().split(" ") {
        if num_str.len() == 0 {
            continue;
        }
        match num_str.parse::<i32>() {
            Ok(num) => nums.push(num),
            _ => (),
        }
    }
    nums
}

fn parse_card(line: &str) -> Card {
    let (winning_str, own_str) = line.split_once(":").unwrap().1.split_once("|").unwrap();

    Card {
        winning_nums: load_nums_to_vec(winning_str),
        own_nums: load_nums_to_vec(own_str),
    }
}

fn get_num_matches(card: &Card) -> i32 {
    let mut matches: i32 = 0;
    for own_num in card.own_nums.clone() {
        for winning_num in card.winning_nums.clone() {
            if own_num == winning_num {
                matches += 1;
            }
        }
    }
    matches
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

    let mut cards: Vec<Card> = Vec::new();
    for line in input_lines {
        cards.push(parse_card(&line));
    }

    let mut total_value: i32 = 0;
    for card in cards {
        let num_matches = get_num_matches(&card);
        if num_matches == 0 {
            continue;
        }
        total_value += 2i32.pow((num_matches - 1) as u32);
    }
    println!("{}", total_value);
}
