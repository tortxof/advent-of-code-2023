use std::collections::HashMap;
use std::fs;

struct Card {
    id: i32,
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
    let (header_str, numbers_str) = line.split_once(":").unwrap();
    let (winning_str, own_str) = numbers_str.split_once("|").unwrap();

    Card {
        id: header_str
            .split_once("Card")
            .unwrap()
            .1
            .trim()
            .parse::<i32>()
            .unwrap(),
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

    let mut cards = Vec::new();
    let mut card_copies = HashMap::new();
    for line in input_lines {
        let card = parse_card(&line);
        card_copies.insert(card.id, 1i32);
        cards.push(card);
    }

    for card in cards {
        let num_matches = get_num_matches(&card);
        if num_matches == 0 {
            continue;
        }
        let num_copies = card_copies.get(&card.id).unwrap().clone();
        for _ in 0..num_copies {
            for card_id in (card.id + 1)..(card.id + 1 + num_matches) {
                match card_copies.get(&card_id) {
                    Some(copies) => {
                        card_copies.insert(card_id, copies + 1);
                    }
                    _ => (),
                }
            }
        }
    }
    let mut total_cards = 0;
    for copies in card_copies.values() {
        total_cards += copies;
    }
    println!("{}", total_cards);
}
