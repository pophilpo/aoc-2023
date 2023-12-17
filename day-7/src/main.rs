use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAkind,
    FullHouse,
    FourOfAkind,
    FiveOfAkind,
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
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => compare_cards(&self.cards, &other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_cards(cards1: &str, cards2: &str) -> Ordering {
    let mut dict: HashMap<char, u32> = HashMap::new();
    dict.insert('2', 2);
    dict.insert('3', 3);
    dict.insert('4', 4);
    dict.insert('5', 5);
    dict.insert('6', 6);
    dict.insert('7', 7);
    dict.insert('8', 8);
    dict.insert('9', 9);
    dict.insert('T', 10);
    dict.insert('J', 11);
    dict.insert('Q', 12);
    dict.insert('K', 13);
    dict.insert('A', 14);

    for (c1, c2) in cards1.chars().zip(cards2.chars()) {
        if dict.get(&c1) > dict.get(&c2) {
            return Ordering::Greater;
        } else if dict.get(&c1) < dict.get(&c2) {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_sort() {
        let card_1 = Hand::new("AAAAA 12");
        let card_2 = Hand::new("AA8AA 12");
        let card_3 = Hand::new("23332 12");
        let card_4 = Hand::new("TTT98 12");
        let card_5 = Hand::new("23432 12");
        let card_6 = Hand::new("A23A4 12");
        let card_7 = Hand::new("23456 12");
        let card_8 = Hand::new("JJJJJ 12");

        assert!(card_1 > card_2);
        assert!(card_2 > card_3);
        assert!(card_3 > card_4);
        assert!(card_4 > card_5);
        assert!(card_5 > card_6);
        assert!(card_6 > card_7);

        assert!(card_1 > card_8);
    }

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
