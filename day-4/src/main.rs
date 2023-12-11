use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = "input.txt";
    let answer = solve_part_one(filename).unwrap();
    println!("The answer is {}", answer);
}

#[derive(Debug)]
struct Card {
    card_number: usize,
    value: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let card_number = line
            .split(":")
            .next()
            .expect("Input is always valid")
            .split_whitespace()
            .last()
            .expect("Input is always valid")
            .parse::<usize>()
            .expect("Input is always valid");

        let card_values = line.split(":").last().expect("Input is always valid");

        let winning_numbers: Vec<u32> = card_values
            .split("|")
            .next()
            .expect("Input is always valid")
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("Input is always valid"))
            .collect::<Vec<u32>>();

        let numbers: Vec<u32> = card_values
            .split("|")
            .last()
            .expect("Input is always valid")
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("Input is always valid"))
            .collect::<Vec<u32>>();

        Card {
            card_number,
            value: 0,
            winning_numbers,
            numbers,
        }
    }

    fn calculate_value(&mut self) {
        // Assuming numbers can't be duplicates
        for number in &self.numbers {
            if self.winning_numbers.contains(&number) {
                if self.value == 0 {
                    self.value = 1;
                } else {
                    self.value *= 2;
                }
            }
        }
    }
}

fn parse_input(filename: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file).lines();

    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        let line = line?;

        let card = Card::new(&line);
        cards.push(card);
    }

    Ok(cards)
}

fn solve_part_one(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut cards = parse_input(filename)?;

    let answer: u32 = cards
        .iter_mut()
        .map(|card| {
            card.calculate_value();
            card.value
        })
        .sum();

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let filename = "test.txt";

        assert_eq!(solve_part_one(filename).unwrap(), 13);
    }
}
