use std::collections::HashMap;
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

#[derive(Debug)]
struct GameRecord {
    red: Option<u16>,
    green: Option<u16>,
    blue: Option<u16>,
}

impl GameRecord {
    pub fn new() -> Self {
        GameRecord {
            red: None,
            green: None,
            blue: None,
        }
    }
    pub fn is_possible(&self, r: u16, g: u16, b: u16) -> bool {
        self.red.unwrap_or(0) <= r && self.green.unwrap_or(0) <= g && self.blue.unwrap_or(0) <= b
    }
    pub fn power(&self) -> u64 {
        self.red.unwrap_or(0) as u64
            * self.green.unwrap_or(0) as u64
            * self.blue.unwrap_or(0) as u64
    }
}

fn process_line(line: String) -> u64 {
    let (id, games) = line.split_once(": ").unzip();
    let games: Vec<GameRecord> = games
        .map(|g| {
            String::from(g)
                .split("; ")
                .map(|set| {
                    let mut record = GameRecord::new();
                    String::from(set).split(", ").for_each(|color_pair| {
                        String::from(color_pair)
                            .split_once(" ")
                            .map(|(num, color)| {
                                let p = Some(num.parse().unwrap());
                                match color {
                                    "red" => record.red = p,
                                    "green" => record.green = p,
                                    "blue" => record.blue = p,
                                    _ => {}
                                };
                            });
                    });
                    record
                })
                .collect::<Vec<GameRecord>>()
        })
        .unwrap();
    // part 1:
    //
    // if games.iter().map(|g| g.is_possible(12, 13, 14)).all(|x| x) {
    //     println!("game is possible:");
    //     id.map(|s| s[5..].parse().unwrap()).unwrap()
    // } else {
    //     0
    // }
    games
        .iter()
        .fold(GameRecord::new(), |acc, e| GameRecord {
            red: std::cmp::max(e.red, acc.red),
            green: std::cmp::max(e.green, acc.green),
            blue: std::cmp::max(e.blue, acc.blue),
        })
        .power()
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
