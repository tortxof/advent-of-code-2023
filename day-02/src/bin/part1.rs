use std::fs;

struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

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

fn hand_is_impossible(bag: &Bag, hand: &Hand) -> bool {
    if hand.red > bag.red {
        return true;
    }
    if hand.green > bag.green {
        return true;
    }
    if hand.blue > bag.blue {
        return true;
    }
    return false;
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
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut id_sum: usize = 0;
    'game_loop: for game in games {
        for hand in game.hands {
            if hand_is_impossible(&bag, &hand) {
                continue 'game_loop;
            }
        }
        id_sum += game.id;
    }
    println!("{id_sum}");
}
