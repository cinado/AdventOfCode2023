use std::{fs::File, io::{self, BufRead}};

#[derive(PartialEq, Eq)]
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

pub fn main() {    
    let file_path: &str = "day-02/input/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let parsed_games: Vec<Game> = reader.lines().map(|line| parse_input(&line.unwrap())).collect();

    let mut sum_of_power: u32 = 0;

    for game in parsed_games.iter() {
        let max_red_cubes = game.game_sets.iter().flat_map(|game_set| game_set.iter()).filter(|cube| cube.colour_name == Colour::Red).map(|cube| cube.count).max().unwrap_or(0);
        let max_green_cubes = game.game_sets.iter().flat_map(|game_set| game_set.iter()).filter(|cube| cube.colour_name == Colour::Green).map(|cube| cube.count).max().unwrap_or(0);
        let max_blue_cubes = game.game_sets.iter().flat_map(|game_set| game_set.iter()).filter(|cube| cube.colour_name == Colour::Blue).map(|cube| cube.count).max().unwrap_or(0);

        let power = max_red_cubes * max_green_cubes * max_blue_cubes;
        sum_of_power += power;
    }
    
    print!("Accumulated result: {}", sum_of_power);
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