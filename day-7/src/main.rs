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
    real_cards: String,
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

        let real_cards = cards.clone();

        let mut card_table: HashMap<char, u32> = HashMap::new();

        for card in cards.chars() {
            *card_table.entry(card).or_insert(0) += 1;
        }

        let mut count: Vec<&u32> = card_table.values().collect();
        count.sort();

        let mut kind = match count.as_slice() {
            [5] => HandType::FiveOfAkind,
            [1, 4] => HandType::FourOfAkind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAkind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        };

        let mut cards_candidate = cards.clone();
        // Iterate over all variants to find the strongest one
        if cards.contains('J') {
            let mut possible_replacements: Vec<char> = Vec::new();
            possible_replacements.push('2');
            possible_replacements.push('3');
            possible_replacements.push('4');
            possible_replacements.push('5');
            possible_replacements.push('6');
            possible_replacements.push('7');
            possible_replacements.push('8');
            possible_replacements.push('9');
            possible_replacements.push('T');
            possible_replacements.push('Q');
            possible_replacements.push('K');
            possible_replacements.push('A');

            for candidate in possible_replacements {
                let mut card_table: HashMap<char, u32> = HashMap::new();
                let new_cards = cards.clone().replace('J', candidate.to_string().as_str());

                for card in new_cards.chars() {
                    *card_table.entry(card).or_insert(0) += 1;
                }

                let mut count: Vec<&u32> = card_table.values().collect();
                count.sort();

                let new_kind = match count.as_slice() {
                    [5] => HandType::FiveOfAkind,
                    [1, 4] => HandType::FourOfAkind,
                    [2, 3] => HandType::FullHouse,
                    [1, 1, 3] => HandType::ThreeOfAkind,
                    [1, 2, 2] => HandType::TwoPair,
                    [1, 1, 1, 2] => HandType::OnePair,
                    [1, 1, 1, 1, 1] => HandType::HighCard,
                    _ => unreachable!(),
                };

                if new_kind >= kind {
                    kind = new_kind;
                    cards_candidate = new_cards;
                }
            }
        }

        return Self {
            real_cards,
            cards: cards_candidate,
            bid,
            kind,
        };
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => compare_cards(&self.real_cards, &other.real_cards),
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
    dict.insert('J', 1);
    dict.insert('2', 2);
    dict.insert('3', 3);
    dict.insert('4', 4);
    dict.insert('5', 5);
    dict.insert('6', 6);
    dict.insert('7', 7);
    dict.insert('8', 8);
    dict.insert('9', 9);
    dict.insert('T', 10);
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

fn solve_part_one(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut answer = 0;
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let line = line?;
        let hand = Hand::new(&line);
        hands.push(hand);
    }

    hands.sort();

    for (i, hand) in hands.iter().enumerate() {
        let current_bid = (i + 1) as u32 * hand.bid;
        answer += current_bid;
    }

    Ok(answer)
}

fn main() {
    let answer = solve_part_one("input.txt").unwrap();
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_sort() {
        let hand_1_input = "JKKK2 12";
        let hand_2_input = "QQQQ2 12";

        let hand_1 = Hand::new(hand_1_input);
        let hand_2 = Hand::new(hand_2_input);

        assert!(hand_1 < hand_2);
    }

    #[test]
    fn test_hand_parsing() {
        let input = "AAAAA 12";
        let expected_result = Hand {
            real_cards: String::from("AAAAA"),
            cards: String::from("AAAAA"),
            bid: 12,
            kind: HandType::FiveOfAkind,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "AA8AA 100";
        let expected_result = Hand {
            real_cards: String::from("AA8AA"),
            cards: String::from("AA8AA"),
            bid: 100,
            kind: HandType::FourOfAkind,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "23332 14";
        let expected_result = Hand {
            real_cards: String::from("23332"),
            cards: String::from("23332"),
            bid: 14,
            kind: HandType::FullHouse,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "TTT98 14";
        let expected_result = Hand {
            real_cards: String::from("TTT98"),
            cards: String::from("TTT98"),
            bid: 14,
            kind: HandType::ThreeOfAkind,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "23432 0";
        let expected_result = Hand {
            real_cards: String::from("23432"),
            cards: String::from("23432"),
            bid: 0,
            kind: HandType::TwoPair,
        };

        assert_eq!(Hand::new(input), expected_result);

        let input = "A23A4 124";
        let expected_result = Hand {
            real_cards: String::from("A23A4"),
            cards: String::from("A23A4"),
            bid: 124,
            kind: HandType::OnePair,
        };

        assert_eq!(Hand::new(input), expected_result);

        assert_eq!(Hand::new(input), expected_result);

        let input = "23456 121";
        let expected_result = Hand {
            real_cards: String::from("23456"),
            cards: String::from("23456"),
            bid: 121,
            kind: HandType::HighCard,
        };

        assert_eq!(Hand::new(input), expected_result);
    }

    #[test]
    fn test_card_genefation_joker() {
        let input = "T55J5 19";
        let expeted_result = Hand {
            real_cards: "T55J5".to_string(),
            cards: "T5555".to_string(),
            bid: 19,
            kind: HandType::FourOfAkind,
        };

        let input = "TTJ22 19";
        let expeted_result = Hand {
            real_cards: "TTJ22".to_string(),
            cards: "TTT22".to_string(),
            bid: 19,
            kind: HandType::FullHouse,
        };
        assert_eq!(Hand::new(input), expeted_result);
    }
}
