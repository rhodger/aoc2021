use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

pub struct Sonar {
    reading: Vec<u16>,
}

impl Sonar {
    pub fn new(filename: &str) -> Sonar {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut nums = Vec::<u16>::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read");
            let num = line.parse::<u16>();
            let num = num.expect("Didn't get a number :/");
            nums.push(num);
        }

        Sonar {
            reading: nums,
        }
    }

    pub fn count_increases(&self) -> u16 {
        let mut count: u16 = 0;
        for i in 1..self.reading.len() {
            if self.reading[i] > self.reading[i - 1] {
                count = count + 1;
            }
        }
        count
    }

    pub fn count_window_increases(&self) -> u16 {
        let mut count: u16 = 0;
        for i in 3..self.reading.len() {
            let reading_b = self.reading[i]
                            + self.reading[i - 1]
                            + self.reading[i - 2];

            let reading_a = self.reading[i - 1]
                            + self.reading[i - 2]
                            + self.reading[i - 3];

            if reading_b > reading_a {
                count = count + 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super ::*;

    #[test]
    fn test_Sonar_new() {
        let sonar = Sonar::new("data/d1_example.txt");

        assert_eq!(sonar.reading.len(), 10);
        assert_eq!(sonar.reading[0], 199);
    }

    #[test]
    fn test_count_increases() {
        let sonar = Sonar::new("data/d1_example.txt");

        assert_eq!(sonar.count_increases(), 7);
    }

    #[test]
    fn test_count_window_increases() {
        let sonar = Sonar::new("data/d1_example.txt");

        assert_eq!(sonar.count_window_increases(), 5);
    }
}