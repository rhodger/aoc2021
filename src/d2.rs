use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum DIRECTION {
    Forward,
    Up,
    Down,
}

pub struct Instructions {
    directions: Vec<DIRECTION>,
    values: Vec<u32>,
}

impl Instructions {
    pub fn new(filename: &str) -> Instructions {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut read_directions = Vec::<DIRECTION>::new();
        let mut read_values = Vec::<u32>::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line :(");
            let mut line = line.split(" ");

            let direction = line.next().expect("Failed to split out direction");
            read_directions.push(match direction {
                "forward" => DIRECTION::Forward,
                "up" => DIRECTION::Up,
                "down" => DIRECTION::Down,
                _ => panic!("Invalid direction read"),
            });

            let value = line.next().expect("Failed to split out value");
            read_values.push(value.parse::<u32>().expect("Failed to parse :O"));
        }

        Instructions {
            directions: read_directions,
            values: read_values,
        }
    }

    pub fn process(&self) -> (u32, u32) {
        let mut x: u32 = 0;
        let mut z: u32 = 0;

        for i in 0..self.directions.len() {
            match self.directions[i] {
                DIRECTION::Forward => x = x + self.values[i],
                DIRECTION::Up => z = z - self.values[i],
                DIRECTION::Down => z = z + self.values[i],
            }
        }

        (x, z)
    }

    pub fn adv_process(&self) -> (u32, u32) {
        let mut x: u32 = 0;
        let mut z: u32 = 0;
        let mut aim: u32 = 0;

        for i in 0..self.directions.len() {
            match self.directions[i] {
                DIRECTION::Forward => {
                    x = x + self.values[i];
                    z = z + self.values[i] * aim;
                },
                DIRECTION::Up => aim = aim - self.values[i],
                DIRECTION::Down => aim = aim + self.values[i],
            }
        }

        (x, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Instructions_new() {
        let instructions = Instructions::new("data/d2_example.txt");

        assert_eq!(instructions.directions.len(), instructions.values.len());
        assert_eq!(instructions.directions.len(), 6);
        assert_eq!(instructions.directions[0], DIRECTION::Forward);
        assert_eq!(instructions.values[0], 5);
    }

    #[test]
    fn test_Instructions_process() {
        let instructions = Instructions::new("data/d2_example.txt");

        assert_eq!(instructions.process(), (15, 10));
    }

    #[test]
    fn test_Instructions_adv_process() {
        let instructions = Instructions::new("data/d2_example.txt");

        assert_eq!(instructions.adv_process(), (15, 60));
    }
}