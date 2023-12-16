use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = "input.txt";
    let answer = solve_part_two(filename).unwrap();
    println!("The answer is {}", answer);
}

#[derive(Debug, Clone)]
struct Card {
    card_number: usize,
    value: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    winning_numbers_count: u32,
}

impl Card {
    fn new(line: &str) -> Self {
        let card_number = line
            .split(':')
            .next()
            .expect("Input is always valid")
            .split_whitespace()
            .last()
            .expect("Input is always valid")
            .parse::<usize>()
            .expect("Input is always valid");

        let card_values = line.split(':').last().expect("Input is always valid");

        let winning_numbers: Vec<u32> = card_values
            .split('|')
            .next()
            .expect("Input is always valid")
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("Input is always valid"))
            .collect::<Vec<u32>>();

        let numbers: Vec<u32> = card_values
            .split('|')
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
            winning_numbers_count: 0,
        }
    }

    fn count_winning_numbers(&mut self) {
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                self.winning_numbers_count += 1
            }
        }
    }

    fn _calculate_value(&mut self) {
        // Assuming numbers can't be duplicates
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
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

fn _solve_part_one(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut cards = parse_input(filename)?;

    let answer: u32 = cards
        .iter_mut()
        .map(|card| {
            card._calculate_value();
            card.value
        })
        .sum();

    Ok(answer)
}

fn walk(table: &HashMap<usize, Card>, card_number: usize, answer: &mut u32) -> u32 {
    let current_card = table.get(&card_number).unwrap();

    let current_winning_number = current_card.winning_numbers_count;

    let range = card_number + (current_winning_number + 1) as usize;

    for tmp_card_number in card_number + 1..range {
        *answer += 1;
        walk(table, tmp_card_number, answer);
    }

    *answer
}

fn solve_part_two(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut answer = 0;
    let mut cards = parse_input(filename)?;
    let mut table: HashMap<usize, Card> = HashMap::new();

    for card in cards.iter_mut() {
        card.count_winning_numbers();
        table.insert(card.card_number, card.clone());
        answer += 1;
    }

    for card in cards.iter() {
        walk(&table, card.card_number, &mut answer);
    }

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let filename = "test.txt";

        assert_eq!(_solve_part_one(filename).unwrap(), 13);
    }

    #[test]
    fn test_solve_part_two() {
        let filename = "test.txt";

        assert_eq!(solve_part_two(filename).unwrap(), 30);
    }
}
