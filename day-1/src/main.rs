use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let answer = solve().unwrap();
    println!("{}", answer);
}

fn solve() -> Result<i32, Box<dyn std::error::Error>> {
    let lines = read_lines("./input.txt")?;
    let mut answer = 0;
    for line in lines {
        let input_string = line?;
        let calibration_number = get_calibration_value(&input_string)?;
        answer += calibration_number;
    }

    Ok(answer)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

    match (digits.first(), digits.last()) {
        (Some(first_digit), Some(last_digit)) => {
            return Ok(format!("{}{}", first_digit, last_digit).parse::<i32>()?)
        }
        (Some(first_digit), None) => {
            return Ok(format!("{}{}", first_digit, first_digit).parse::<i32>()?)
        }
        _ => return Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value("1abc2").unwrap(), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(get_calibration_value("treb7uchet").unwrap(), 77);
        assert_eq!(get_calibration_value("abc").unwrap(), 0);
    }
}
