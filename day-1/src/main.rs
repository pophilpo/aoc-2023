use core::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

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
    println!("Working on line: [{}]", line);
    let mut trie = Node::new();
    trie.insert("one", 1);
    trie.insert("two", 2);
    trie.insert("three", 3);
    trie.insert("four", 4);
    trie.insert("five", 5);
    trie.insert("six", 6);
    trie.insert("seven", 7);
    trie.insert("eight", 8);
    trie.insert("nine", 9);

    let mut digits: Vec<i32> = Vec::new();

    // Case like threight doesn't work.
    // t (hree) -> th (ree) -> thr -> thre (e) -> threi (No nodes found from the leaf E, go to top, no Nodes found with I)
    // Need to add some sort of backtracking?
    let mut node = &trie;

    println!("{}", node);

    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        println!("Current char: {}", c);
        match node.children.get(&c) {
            Some(n) => {
                if let Some(digit) = n.value {
                    digits.push(digit);

                    println!("Found a value {}! ", digit);

                    node = &trie;
                    println!("Made a full reset!");
                    println!("{}", node);
                    match node.children.get(&c) {
                        Some(node_next) => {
                            println!("Found a value for char {}!", c);
                            node = node_next;
                        }
                        None => {
                            println!("No children after finding a value and making a reset!");
                            println!("{}", node);
                        }
                    }
                } else {
                    println!("Not a LEAF!");

                    node = n;

                    println!("Current char {}", c);
                    println!("{}", node);

                    let next_char = chars.peek();

                    match next_char {
                        Some(next_char) => {
                            if let Some(_) = node.children.get(&next_char) {
                                continue;
                            } else {
                                node = &trie;
                                match node.children.get(&c) {
                                    Some(new_node) => node = new_node,
                                    None => node = &trie,
                                }
                            }
                        }
                        None => continue,
                    }
                }
            }
            None => {
                println!("No child was found for current char! {}", c);
                if c.is_digit(10) {
                    let digit = c.to_digit(10).unwrap();
                    digits.push(digit as i32);
                }

                println!("Made a full reset!");
                node = &trie;
                println!("{}", node);
                match node.children.get(&c) {
                    Some(n) => {
                        node = n;
                        println!("Found a child after reset for char {}!", c);
                        println!("{}", node);
                    }
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
        trie.insert("one", 1);
        trie.insert("two", 2);
        trie.insert("three", 3);
        trie.insert("four", 4);
        trie.insert("five", 5);
        trie.insert("six", 6);
        trie.insert("seven", 7);
        trie.insert("eight", 8);
        trie.insert("nine", 9);

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
