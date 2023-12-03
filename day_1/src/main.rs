use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_word_or_digit(s: String) -> Option<u64> {
    match &s[..] {
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn process_line(line: String) -> u64 {
    println!("{}", line);
    let mut first: Option<u64> = None;
    let mut second = 0u64;
    let mut iter = line.chars().peekable();
    // part 1
    // while let Some(char) = iter.next() {
    //     if char.is_digit(10) {
    //         second = char;
    //         first = if first == None { Some(char) } else { first };
    //     }
    // }
    while let Some(char) = iter.next() {
        match parse_word_or_digit(char.to_string()) {
            Some(number) => {
                println!("\tfound number: {}", number);
                second = number;
                first = if first == None { Some(number) } else { first };
            }
            None => {
                println!("\tfound char: {}", char);
                // longest word is 5 chars
                let mut word: String = char.to_string();
                let mut branch_iter = iter.clone();
                while word.len() <= 5 {
                    println!("\t\ttesting: {}", word);
                    if let Some(parse) = parse_word_or_digit(word.clone()) {
                        println!("\tpassed: {}", parse);
                        second = parse;
                        first = if first == None { Some(parse) } else { first };
                        break;
                    } else {
                        if let Some(next_char) = branch_iter.next() {
                            word = word + &next_char.to_string();
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    format!("{}{}", first.unwrap(), second)
        .parse::<u64>()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let output: u64 = read_lines(args[1].clone())
        .unwrap()
        .map(|line| process_line(line.unwrap()))
        .collect::<Vec<u64>>()
        .iter()
        .sum();
    println!("{:?}", output);
}
