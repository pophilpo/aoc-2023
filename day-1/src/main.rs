use core::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

const DIGIT_MAPPINGS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[derive(Default, Debug)]
struct Node {
    children: HashMap<char, Node>,
    value: Option<i32>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            value: None,
        }
    }

    fn insert(&mut self, word: &str, value: i32) {
        let mut node = self;

        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(Node::new);
        }
        node.value = Some(value);
    }

    fn _search(&self, word: &str) -> Option<i32> {
        // Don't really need this func, but will use for testing
        let mut node = self;

        for c in word.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return None,
            }
        }
        node.value
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: Vec<char> = Vec::new();

        for (child, _) in &self.children {
            result.push(child.clone());
            result.push(' ');
        }
        let result: String = result.into_iter().collect();

        write!(f, "Current Node Paths: {}", result)
    }
}

fn main() {
    let answer = solve().unwrap();
    println!("{}", answer);
}

fn solve() -> Result<i32, Box<dyn std::error::Error>> {
    let lines = read_lines("./input.txt")?;
    let mut answer = 0;
    for line in lines {
        let input_string = line?;
        let calibration_number = get_calibration_value_part_two(&input_string)?;
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

fn _get_calibration_value_part_one(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

    match (digits.first(), digits.last()) {
        (Some(first_digit), Some(last_digit)) => {
            return Ok(format!("{}{}", first_digit, last_digit).parse::<i32>()?)
        }
        _ => return Ok(0),
    }
}

fn get_calibration_value_part_two(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut trie = Node::new();

    for &(word, digit_value) in &DIGIT_MAPPINGS {
        trie.insert(word, digit_value);
    }

    // Get all found digits into 1 vec
    let mut digits: Vec<i32> = Vec::new();

    let mut node = &trie;
    let mut chars = line.chars().peekable();

    while let Some(current_char) = chars.next() {
        // Check if current char is part of the word
        match node.children.get(&current_char) {
            Some(matched_node) => {
                // If matched node has a value, it's the last node
                if let Some(digit) = matched_node.value {
                    // Push the digit of the last value to all found digits
                    digits.push(digit);

                    // Reset the node to the first one, since we reached the end
                    node = &trie;

                    // Check if the current char might start a new node
                    // E.g. for cases like twone where the correct result would be 2, 1
                    match node.children.get(&current_char) {
                        // If the current char is a start of a new word
                        // Just continue traversal in that direction
                        Some(matched_node) => {
                            node = matched_node;
                        }
                        // Do nothing if the current char doesn't start a word
                        // Meaning we are still at the top most node
                        None => {
                            continue;
                        }
                    }
                } else {
                    // Case where we found a new node, but it's not the last one
                    // Meaning this node is not a Leaf and has children

                    // Set the current node to the new matched node
                    node = matched_node;

                    // In order not get a dead end, we need to check that if next char
                    // Is one of the childs of the current node
                    // This part is designed for cases like "threight"
                    // The traversal wihout this part would look like this: (word_part [current_node_children])
                    // None [o, t, f, s, e, n] -> t [h, w] -> th [r] -> thr -> [e] -> thre [e] -> threi [nowhere to go, reset] -> i [nowhere to go, next char]
                    // So when we reach a node with 1 leaf child, but the char value of this node could be another word, we will hit a dead end because the only child
                    // Is a leaf

                    // Get the next char in line without consuming it
                    let next_char = chars.peek();

                    match next_char {
                        Some(next_char) => {
                            // If the next char leads somewhere from the current node
                            // Just leave everything as is, it will continue traversal in the right direction
                            if let Some(_) = node.children.get(&next_char) {
                                continue;
                            }
                            // If the next char hit's a dead end, check if current char
                            // Might start a new word
                            else {
                                node = &trie;
                                match node.children.get(&current_char) {
                                    // If current char can start a new word -> go in that direction
                                    Some(new_node) => node = new_node,

                                    // If current char is not a start of a new word, start from the top
                                    None => continue,
                                }
                            }
                        }

                        // If there is no next char just do nothing
                        None => continue,
                    }
                }
            }

            // Case where a char doesn't math any node
            None => {
                // It might be a digit
                if current_char.is_digit(10) {
                    let digit = current_char.to_digit(10).unwrap();
                    digits.push(digit as i32);
                }

                // If it's not a digit, start from the top
                node = &trie;

                // Check if the char might begin a new word
                match node.children.get(&current_char) {
                    // If it does, go in that direction
                    Some(n) => {
                        node = n;
                    }
                    // If it doesn't, just stay at the top and take next char
                    None => continue,
                }
            }
        }
    }

    match (digits.first(), digits.last()) {
        (Some(first_digit), Some(last_digit)) => {
            return Ok(format!("{}{}", first_digit, last_digit).parse::<i32>()?)
        }
        _ => return Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_calibration_value_part_one() {
        assert_eq!(_get_calibration_value_part_one("1abc2").unwrap(), 12);
        assert_eq!(_get_calibration_value_part_one("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(_get_calibration_value_part_one("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(_get_calibration_value_part_one("treb7uchet").unwrap(), 77);
        assert_eq!(_get_calibration_value_part_one("abc").unwrap(), 0);
    }

    #[test]
    fn test_trie() {
        let mut trie = Node::new();

        for &(word, digit_value) in &DIGIT_MAPPINGS {
            trie.insert(word, digit_value);
        }

        assert_eq!(trie._search("one"), Some(1));
        assert_eq!(trie._search("two"), Some(2));
        assert_eq!(trie._search("three"), Some(3));
        assert_eq!(trie._search("four"), Some(4));
        assert_eq!(trie._search("five"), Some(5));
        assert_eq!(trie._search("six"), Some(6));
        assert_eq!(trie._search("seven"), Some(7));
        assert_eq!(trie._search("eight"), Some(8));
        assert_eq!(trie._search("nine"), Some(9));
        assert_eq!(trie._search("nines"), None);
    }

    #[test]
    fn test_get_calibration_value_part_two() {
        assert_eq!(get_calibration_value_part_two("2three5three").unwrap(), 23);
        assert_eq!(get_calibration_value_part_two("abcontwdfdfdf").unwrap(), 0);
        assert_eq!(get_calibration_value_part_two("one2").unwrap(), 12);
        assert_eq!(get_calibration_value_part_two("otwo3").unwrap(), 23);
        assert_eq!(get_calibration_value_part_two("two1nine").unwrap(), 29);
        assert_eq!(get_calibration_value_part_two("eightwothree").unwrap(), 83);
        assert_eq!(
            get_calibration_value_part_two("abcone2threexyz").unwrap(),
            13
        );
        assert_eq!(get_calibration_value_part_two("xtwone3four").unwrap(), 24);
        assert_eq!(
            get_calibration_value_part_two("4nineeightseven2").unwrap(),
            42
        );
        assert_eq!(get_calibration_value_part_two("zoneight234").unwrap(), 14);
        assert_eq!(get_calibration_value_part_two("onetwone").unwrap(), 11);
        assert_eq!(get_calibration_value_part_two("sevennine").unwrap(), 79);
        assert_eq!(get_calibration_value_part_two("7pqrstsixteen").unwrap(), 76);
        assert_eq!(
            get_calibration_value_part_two("9one9pjtnncsqzhcszp5").unwrap(),
            95
        );

        assert_eq!(get_calibration_value_part_two("sevenninenine").unwrap(), 79);
        assert_eq!(get_calibration_value_part_two("sevenninenine").unwrap(), 79);

        // Found the culprit. Doesn't pass
        // Fixed with a peekable lookup of the next char and making a step back
        assert_eq!(get_calibration_value_part_two("threight").unwrap(), 88);
    }
}
