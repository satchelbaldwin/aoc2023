use std::env;
use std::fs;

type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct PartNumber {
    x: u16,
    y: u16,
    width: u8,
}

impl PartNumber {
    pub fn neighbors(&self, width: u16, height: u16) -> Vec<(u32, u32)> {
        (self.x as i32 - 1..self.x as i32 + self.width as i32 + 1)
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
            .map(|(x, y)| (*x as u32, *y as u32))
            .collect()
    }
}

fn string_to_grid(s: String) -> Grid {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("failed to read file.");
    println!(
        "{:?}",
        (PartNumber {
            x: 3,
            y: 2,
            width: 4
        })
        .neighbors(10, 10)
    )
}
