use std::{fs::File, io::{self, BufRead}};

pub fn main() {

    let mut parsed_numbers: Vec<u32> = vec![];
    let mut parsed_char_numbers: Vec<char> = vec![];
    
    let file_path: &str = "day-01/input/inputPart1.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        for character in line.unwrap().chars(){
            if character.is_numeric() {
                parsed_char_numbers.push(character.clone());
            }
        }
        if parsed_char_numbers.len() > 0 {
            parsed_numbers.push(concatenate_and_return_number(&parsed_char_numbers));
            parsed_char_numbers.clear();
        }
    }
    let sum_of_parsed_numbers: u32 = parsed_numbers.iter().sum();
    print!("Accumulated result: {}", sum_of_parsed_numbers);
}

fn concatenate_and_return_number(parsed_char_numbers: &Vec<char>) -> u32 {
    let first_number = parsed_char_numbers.get(0).unwrap();
    let second_number = parsed_char_numbers.get(parsed_char_numbers.len()-1).unwrap();

    let number_as_string: String = first_number.to_string() + &second_number.to_string();

    match number_as_string.parse::<u32>(){
        Ok(parsed_number) => return parsed_number,
        Err(_) => return 0,
    }
}