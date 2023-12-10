use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(
    f: fn(&str) -> Result<Option<u32>, Box<dyn std::error::Error>>,
    filename: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let lines = read_lines(filename)?;

    let mut answer = 0;
    for line in lines {
        if let Some(game_id) = f(&line?)? {
            answer += game_id;
        }
    }

    Ok(answer)
}

fn parse_line_part_one(line: &str) -> Result<Option<u32>, Box<dyn std::error::Error>> {
    let mut cube_constraints: HashMap<String, u32> = HashMap::new();
    cube_constraints.insert(String::from("red"), 12);
    cube_constraints.insert(String::from("green"), 13);
    cube_constraints.insert(String::from("blue"), 14);

    // Just in case the elf is funny and the input is not sorted by the game id
    let game_id = line
        .split(":")
        .next()
        .expect("The input is always valid")
        .split_whitespace()
        .last()
        .expect("The input is always valid")
        .parse::<u32>()?;

    let modified_input = line.replace(";", ",");
    let game_results: Vec<&str> = modified_input
        .split(":")
        .last()
        .expect("The input is always valid")
        .trim()
        .split(", ")
        .collect();
    for result in game_results {
        let mut parts = result.split_whitespace();

        let cube_count = parts
            .next()
            .expect("Input is always valid")
            .parse::<u32>()?;
        let cube_color = parts.last().expect("Input is always valid");

        let max_cube_count = cube_constraints
            .get(cube_color)
            .expect("The color is always correct");

        if cube_count > *max_cube_count {
            return Ok(None);
        }
    }

    return Ok(Some(game_id));
}

fn main() {
    let answer = solve(parse_line_part_one, "input.txt").unwrap();
    println!("The answer is {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_part_one() {
        let correct_game_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(parse_line_part_one(correct_game_1).unwrap(), Some(1));

        let incorrect_game_2 =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        assert_eq!(parse_line_part_one(incorrect_game_2).unwrap(), None);
    }

    #[test]
    fn test_part_one_solution() {
        let input_filename = "test.txt";
        assert_eq!(solve(parse_line_part_one, input_filename).unwrap(), 8);
    }
}
