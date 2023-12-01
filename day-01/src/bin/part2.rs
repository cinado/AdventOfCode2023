use std::{fs::File, io::{self, BufRead}};

const NUMBERS_STRING_REPRESENTATION: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn main() {

    let mut parsed_numbers: Vec<u32> = vec![];
    
    let file_path: &str = "day-01/input/inputPart1.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        parsed_numbers.push(process_line(line.unwrap()));
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

fn process_line(mut line: String) -> u32 {
    let mut parsed_char_numbers: Vec<char> = vec![];

    for (index, number_as_string) in NUMBERS_STRING_REPRESENTATION.iter().enumerate(){
        line = line.replace(number_as_string, &(number_as_string.to_string() + &(index+1).to_string() + &number_as_string))
    }

    for character in line.chars(){
        if character.is_numeric() {
            parsed_char_numbers.push(character.clone());
        }
    }

    concatenate_and_return_number(&parsed_char_numbers)
}