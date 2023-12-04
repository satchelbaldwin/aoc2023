use std::collections::HashMap;
use std::env;
use std::fs;

const EMPTY_SPACE: char = '.';

#[derive(Debug)]
struct PartNumber {
    x: u16,
    y: u16,
    width: u8,
    value: u16,
}

impl PartNumber {
    pub fn neighbors(&self, width: u16, height: u16) -> Vec<(usize, usize)> {
        (self.x as i32 - 1..=self.x as i32 + self.width as i32)
            .fold(Vec::new(), |mut acc, e| {
                acc.push((e, self.y as i32 - 1));
                acc.push((e, self.y as i32 + 1));
                if e < self.x as i32 || e >= self.x as i32 + self.width as i32 {
                    acc.push((e, self.y as i32));
                }
                acc
            })
            .iter()
            .filter(|(x, y)| *x >= 0 && *x < width as i32 && *y >= 0 && *y < height as i32)
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect()
    }
    pub fn is_valid_part_number(&self, g: &Grid) -> bool {
        self.neighbors(g.width, g.height)
            .iter()
            .map(|(x, y)| g.data[*y][*x])
            .any(|c| c != EMPTY_SPACE && !c.is_digit(10))
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    width: u16,
    height: u16,
}
impl Grid {
    fn new(s: String) -> Grid {
        let mut g = Grid {
            data: s.lines().map(|line| line.chars().collect()).collect(),
            width: 0,
            height: 0,
        };
        g.width = g.data[0].len() as u16;
        g.height = g.data.len() as u16;
        g
    }
    fn extract_part_numbers(&self) -> Vec<PartNumber> {
        self.data
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut parts, (line_number, line)| {
                let mut number_stack: Vec<char> = Vec::new();
                let mut start_position = 0;
                line.iter().enumerate().for_each(|(char_number, c)| {
                    if c.is_digit(10) {
                        if number_stack.is_empty() {
                            start_position = char_number;
                        }
                        number_stack.push(*c);
                    }
                    if (!c.is_digit(10) || char_number >= line.len() - 1)
                        && !number_stack.is_empty()
                    {
                        parts.push(PartNumber {
                            x: start_position as u16,
                            y: line_number as u16,
                            width: number_stack.len() as u8,
                            value: number_stack.iter().collect::<String>().parse().unwrap(),
                        });
                        number_stack.clear();
                    }
                });
                parts
            })
            .into_iter()
            .collect()
    }
    fn extract_gear_ratios(&self) -> u64 {
        let mut gears: HashMap<(usize, usize), (u64, Vec<u64>)> = HashMap::new();
        self.extract_part_numbers().into_iter().for_each(|part| {
            part.neighbors(self.width, self.height)
                .iter()
                .map(|(x, y)| ((x, y), self.data[*y][*x]))
                .filter(|(_, c)| *c == '*')
                .for_each(|((x, y), _)| {
                    gears
                        .entry((*x, *y))
                        .and_modify(|(count, parts)| {
                            *count += 1;
                            parts.push(part.value.into())
                        })
                        .or_insert((1, vec![part.value.into()]));
                });
        });
        gears
            .iter()
            .filter(|(_, (c, _))| *c == 2)
            .map(|(_, (_, vs))| vs.iter().fold(1, |acc, e| acc * e))
            .sum::<u64>()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("failed to read file.");
    let grid = Grid::new(file);

    let valid_parts: Vec<PartNumber> = grid
        .extract_part_numbers()
        .into_iter()
        .filter(|part| part.is_valid_part_number(&grid))
        .collect();
    let part_1 = valid_parts.iter().map(|p| p.value as u64).sum::<u64>();

    let gears = grid.extract_gear_ratios();

    println!("{:?}", part_1);
    println!("{:?}", gears);
}
