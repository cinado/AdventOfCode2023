use std::{fs::File, io::{self, BufRead}};

enum Colour{
    Blue,
    Red,
    Green
}

impl Colour{ 
    fn from_string(string: &str) -> Option<Colour>{
        match string.to_lowercase().as_str() {
            "blue" => Some(Colour::Blue),
            "red" => Some(Colour::Red),
            "green" => Some(Colour::Green),
            _ => None
        }
    }
}

pub struct CubeColour {
    count: u32,
    colour_name: Colour
}

pub struct Game {
    game_id: u32,
    game_sets: Vec<Vec<CubeColour>>
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

pub fn main() {    
    let file_path: &str = "day-02/input/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let parsed_games: Vec<Game> = reader.lines().map(|line| parse_input(&line.unwrap())).collect();

    let filtered_games: Vec<&Game> = parsed_games.iter().filter(|current_game| current_game.game_sets.iter().all(|game_set| {
        game_set.iter().all(|cube_colour| {
            match cube_colour.colour_name {
                Colour::Red => cube_colour.count <= MAX_RED_CUBES,
                Colour::Blue => cube_colour.count <= MAX_BLUE_CUBES,
                Colour::Green => cube_colour.count <= MAX_GREEN_CUBES
            }
        })
    })).collect();

    let sum_of_game_ids: u32 = filtered_games.iter().map(|game| game.game_id).sum();
    print!("Accumulated result: {}", sum_of_game_ids);
}

pub fn parse_input(line: &str) -> Game {
    let line_splitted: Vec<&str> = line.split(":").collect();
    let game_id = line_splitted[0].split_whitespace().collect::<Vec<&str>>()[1];

    let game_sets: Vec<Vec<CubeColour>> = line_splitted[1].trim().split(";").map(|current_game_round| process_game_round(current_game_round)).collect();
    Game { game_id: game_id.parse().unwrap(), game_sets: game_sets }
}

pub fn process_game_round(current_game_round: &str) -> Vec<CubeColour> {
    current_game_round.trim().split(",").map(|cube_entry| {
        let cube_entry_parts: Vec<&str> = cube_entry.split_whitespace().collect();
        CubeColour { count: cube_entry_parts[0].parse().unwrap(), colour_name: Colour::from_string(cube_entry_parts[1]).unwrap() }
    }).collect()
}