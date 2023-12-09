use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

#[derive(Default)]
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

    fn search(&self, word: &str) -> Option<i32> {
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

fn main() {
    let answer = solve().unwrap();
    println!("{}", answer);
}

fn solve() -> Result<i32, Box<dyn std::error::Error>> {
    let lines = read_lines("./input.txt")?;
    let mut answer = 0;
    for line in lines {
        let input_string = line?;
        let calibration_number = get_calibration_value_part_one(&input_string)?;
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

fn get_calibration_value_part_one(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
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

fn get_calibration_value_part_two(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_calibration_value_part_one() {
        assert_eq!(get_calibration_value_part_one("1abc2").unwrap(), 12);
        assert_eq!(get_calibration_value_part_one("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(get_calibration_value_part_one("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(get_calibration_value_part_one("treb7uchet").unwrap(), 77);
        assert_eq!(get_calibration_value_part_one("abc").unwrap(), 0);
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

        assert_eq!(trie.search("one"), Some(1));
        assert_eq!(trie.search("two"), Some(2));
        assert_eq!(trie.search("three"), Some(3));
        assert_eq!(trie.search("four"), Some(4));
        assert_eq!(trie.search("five"), Some(5));
        assert_eq!(trie.search("six"), Some(6));
        assert_eq!(trie.search("seven"), Some(7));
        assert_eq!(trie.search("eight"), Some(8));
        assert_eq!(trie.search("nine"), Some(9));
        assert_eq!(trie.search("nines"), None);
    }
}
