use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    parse_input().unwrap();
}

#[derive(Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    length: usize,
    location: Coordinates,
}

impl PartNumber {
    fn new(
        digit_string: &str,
        mut location: Coordinates,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let length = digit_string.len();
        let value = digit_string.parse::<u32>()?;

        location.x = location.x - length;

        Ok(PartNumber {
            value,
            length,
            location,
        })
    }
}

fn parse_input() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "test.txt";
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file).lines();

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Coordinates> = Vec::new();

    for (y, line) in lines.enumerate() {
        let line = line?;
        println!("{}", line);
        let mut curent_number_buffer: Vec<char> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if !c.is_digit(10) {
                if !curent_number_buffer.is_empty() {
                    let number: String = curent_number_buffer.iter().collect();

                    let location = Coordinates { x, y };
                    let part_number = PartNumber::new(&number, location)?;
                    part_numbers.push(part_number);
                    curent_number_buffer.clear();
                }

                match c {
                    '.' => {
                        continue;
                    }
                    _ => {
                        let location = Coordinates { x, y };
                        symbols.push(location);
                    }
                }
            } else {
                curent_number_buffer.push(c);
            }
        }
    }

    for part_number in part_numbers {
        println!("{:?}", part_number);
    }

    for symbol in symbols {
        println!("{:?}", symbol);
    }

    Ok(())
}
