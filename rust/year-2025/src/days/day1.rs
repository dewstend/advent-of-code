use std::{error::Error, fs, str::FromStr};

const DIAL_MIN: u8 = 0;
const DIAL_MAX: u8 = 99;
const RANGE_SIZE: i32 = (DIAL_MAX - DIAL_MIN + 1) as i32;

enum Direction {
    Left,
    Right
}

struct Rotation {
    direction: Direction,

    turns: u16
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Empty Line".to_string());
        }

        let (dir_char, turn_char) = s.split_at(1);

        let direction =  match dir_char {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err("Unknown direction".to_string())
        };

        let turns = turn_char.parse::<u16>()
            .map_err(|_| "Invalid number format")?;


        Ok(Rotation { direction, turns })
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {

    let input_string: String = fs::read_to_string("./input/day1.txt")?;

    let rotations: Vec<Rotation> = input_string
        .lines()
        .map(|line| line.parse()
            .expect("Failed to parse rotation"))
        .collect();

    let mut zero_hits: u16 = 0;

    let mut dial_pos: u8 = 50;

    for rotation in rotations {
        let mut new_pos: i32 = dial_pos as i32;

        match rotation.direction {
            Direction::Left => new_pos-=rotation.turns as i32,
            Direction::Right => new_pos+=rotation.turns as i32
        }

        if new_pos > DIAL_MAX as i32 {
            new_pos %= RANGE_SIZE as i32;
        }

        if new_pos < DIAL_MIN as i32 {
            new_pos = (new_pos % RANGE_SIZE + RANGE_SIZE) % RANGE_SIZE;
        }

        dial_pos = new_pos as u8;

        if dial_pos == 0 { zero_hits+=1 }
    }
    
    println!("Zero hits: {zero_hits}");

    Ok(())
}