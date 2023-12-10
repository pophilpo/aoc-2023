use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let answser = solve_part_two("input.txt").unwrap();
    println!("The answer is {}", answser);
}

#[derive(Debug, Clone)]
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

fn _parse_input(
    filename: &str,
) -> Result<(Vec<PartNumber>, Vec<Coordinates>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Coordinates> = Vec::new();

    for (y, line) in lines.enumerate() {
        let line = line?;
        let mut curent_number_buffer: Vec<char> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let location = Coordinates { x: x + 1, y: y + 1 };

            if !c.is_digit(10) {
                if !curent_number_buffer.is_empty() {
                    let number: String = curent_number_buffer.iter().collect();

                    let part_number = PartNumber::new(&number, location.clone())?;
                    part_numbers.push(part_number);
                    curent_number_buffer.clear();
                }

                match c {
                    '.' => {
                        continue;
                    }
                    _ => {
                        symbols.push(location);
                    }
                }
            } else {
                curent_number_buffer.push(c);
            }
        }

        if !curent_number_buffer.is_empty() {
            let location = Coordinates {
                x: line.len(),
                y: y + 1,
            };
            let number: String = curent_number_buffer.iter().collect();
            let part_number = PartNumber::new(&number, location.clone())?;
            part_numbers.push(part_number);
            curent_number_buffer.clear();
        }
    }

    Ok((part_numbers, symbols))
}

fn parse_input_part_two(
    filename: &str,
) -> Result<(Vec<PartNumber>, Vec<Coordinates>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Coordinates> = Vec::new();

    for (y, line) in lines.enumerate() {
        let line = line?;
        let mut curent_number_buffer: Vec<char> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let location = Coordinates { x: x + 1, y: y + 1 };

            if !c.is_digit(10) {
                if !curent_number_buffer.is_empty() {
                    let number: String = curent_number_buffer.iter().collect();

                    let part_number = PartNumber::new(&number, location.clone())?;
                    part_numbers.push(part_number);
                    curent_number_buffer.clear();
                }

                match c {
                    '*' => {
                        symbols.push(location);
                    }
                    _ => {
                        continue;
                    }
                }
            } else {
                curent_number_buffer.push(c);
            }
        }

        if !curent_number_buffer.is_empty() {
            let location = Coordinates {
                x: line.len(),
                y: y + 1,
            };
            let number: String = curent_number_buffer.iter().collect();
            let part_number = PartNumber::new(&number, location.clone())?;
            part_numbers.push(part_number);
            curent_number_buffer.clear();
        }
    }

    Ok((part_numbers, symbols))
}

fn _solve_part_one(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut answer = 0;
    let (part_numbers, symbols) = _parse_input(filename)?;

    for part in part_numbers {
        for symbol in &symbols {
            if symbol.y >= part.location.y - 1
                && symbol.y <= part.location.y + 1
                && symbol.x >= part.location.x - 1
                && symbol.x <= part.location.x + part.length
            {
                answer += part.value;
                break;
            }
        }
    }

    Ok(answer)
}

fn solve_part_two(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut answer = 0;

    let (part_numbers, symbols) = parse_input_part_two(filename)?;

    for symbol in symbols {
        let mut count = 0;

        let mut gear_ratio = 1;

        for part in &part_numbers {
            if count > 2 {
                break;
            }

            if symbol.y >= part.location.y - 1
                && symbol.y <= part.location.y + 1
                && symbol.x >= part.location.x - 1
                && symbol.x <= part.location.x + part.length
            {
                count += 1;
                gear_ratio = gear_ratio * part.value;
            }
        }

        if count == 2 {
            answer += gear_ratio;
        }
    }

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_solution_part_one() {
        let filename = "test.txt";
        assert_eq!(_solve_part_one(filename).unwrap(), 4361);
    }

    #[test]
    fn test_solution_part_two() {
        let filename = "test.txt";
        assert_eq!(solve_part_two(filename).unwrap(), 467835);
    }
}
