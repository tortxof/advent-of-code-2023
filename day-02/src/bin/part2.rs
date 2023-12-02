use std::fs;

struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

#[allow(dead_code)]
struct Game {
    id: usize,
    hands: Vec<Hand>,
}

fn parse_count(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(number) => number,
        Err(_error) => 0usize,
    }
}

fn parse_hand(input: &str) -> Hand {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube_part in input.split(",") {
        match cube_part.trim().split_once(" ") {
            Some(parts) => {
                let (count, color) = parts;
                match color {
                    "red" => red += parse_count(count),
                    "green" => green += parse_count(count),
                    "blue" => blue += parse_count(count),
                    _ => (),
                }
            }
            None => (),
        }
    }
    Hand {
        red: red,
        green: green,
        blue: blue,
    }
}

fn get_game_id(input: &str) -> Option<usize> {
    match input.split_once(" ") {
        Some(parts) => {
            let (_, id_str) = parts;
            match id_str.parse::<usize>() {
                Ok(number) => Some(number),
                Err(_error) => None,
            }
        }
        None => {
            return None;
        }
    }
}

fn parse_input_line(input: &str) -> Option<Game> {
    match input.split_once(":") {
        Some(parts) => {
            let (game_part, hands_part) = parts;
            let game_id = get_game_id(game_part.trim()).unwrap();
            let mut hands: Vec<Hand> = Vec::new();
            for hand_str in hands_part.trim().split(";") {
                hands.push(parse_hand(hand_str.trim()));
            }
            return Some(Game {
                id: game_id,
                hands: hands,
            });
        }
        None => {
            return None;
        }
    }
}

fn main() {
    let mut games: Vec<Game> = Vec::new();
    for line in fs::read_to_string("./input.txt").unwrap().lines() {
        if line.len() == 0 {
            continue;
        }
        match parse_input_line(line) {
            Some(game) => games.push(game),
            None => (),
        }
    }
    let mut power_sum = 0usize;
    for game in games {
        let mut min_hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };
        for hand in game.hands {
            if hand.red > min_hand.red {
                min_hand.red = hand.red;
            }
            if hand.green > min_hand.green {
                min_hand.green = hand.green;
            }
            if hand.blue > min_hand.blue {
                min_hand.blue = hand.blue;
            }
        }
        let game_power = min_hand.red * min_hand.green * min_hand.blue;
        power_sum += game_power;
    }
    println!("{power_sum}");
}
