use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAkind,
    FourOfAkind,
    FullHouse,
    ThreeOfAkind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: String,
    bid: u32,
    kind: HandType,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut line = line.split_whitespace();

        let cards = line.next().expect("The input is always valid").to_owned();
        let bid: u32 = line
            .next()
            .expect("The input is alwasy valid")
            .parse()
            .expect("The input is always valid");

        let mut card_table: HashMap<char, u32> = HashMap::new();

        for card in cards.chars() {
            *card_table.entry(card).or_insert(0) += 1;
        }

        let mut count: Vec<&u32> = card_table.values().collect();
        count.sort();

        let kind = match count.as_slice() {
            [5] => HandType::FiveOfAkind,
            [1, 4] => HandType::FourOfAkind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAkind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        };

        return Self { cards, bid, kind };
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_parsing() {
        let input = "AAAAA 12";
        let expected_result = Hand {
            cards: String::from("AAAAA"),
            bid: 12,
            kind: HandType::FiveOfAkind,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "AA8AA 100";
        let expected_result = Hand {
            cards: String::from("AA8AA"),
            bid: 100,
            kind: HandType::FourOfAkind,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "23332 14";
        let expected_result = Hand {
            cards: String::from("23332"),
            bid: 14,
            kind: HandType::FullHouse,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "TTT98 14";
        let expected_result = Hand {
            cards: String::from("TTT98"),
            bid: 14,
            kind: HandType::ThreeOfAkind,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "23432 0";
        let expected_result = Hand {
            cards: String::from("23432"),
            bid: 0,
            kind: HandType::TwoPair,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "A23A4 124";
        let expected_result = Hand {
            cards: String::from("A23A4"),
            bid: 124,
            kind: HandType::OnePair,
        };

        assert_eq!(Hand::new(input), expected_result);

        assert_eq!(Hand::new(input), expected_result);

        let input = "23456 121";
        let expected_result = Hand {
            cards: String::from("23456"),
            bid: 121,
            kind: HandType::HighCard,
        };

        assert_eq!(Hand::new(input), expected_result);
    }
}
