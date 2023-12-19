use std::fs::File;
use std::io::{self, BufRead};

struct Node {
    value: String,
    left: usize,
    right: usize,
}

impl Node {
    fn new(line: &str, tmp: &Vec<String>) -> Self {
        let mut line = line.replace("= ", "");
        line = line.replace(",", "");
        line = line.replace("(", "");
        line = line.replace(")", "");

        let parts: Vec<&str> = line.split_whitespace().collect();

        let value = parts[0].to_string();
        let left = parts[1];
        let right = parts[2];

        let index_left = tmp
            .iter()
            .position(|value| value == left)
            .expect("The input is always valid");
        let index_right = tmp
            .iter()
            .position(|value| value == right)
            .expect("The input is always valid");

        Self {
            value,
            left: index_left,
            right: index_right,
        }
    }
}

fn parse_input(filename: &str) -> Result<(String, Vec<Node>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;

    let mut lines = io::BufReader::new(file).lines();
    let instructions = lines.next().expect("The input is always valid")?;
    lines.next();

    let mut nodes: Vec<Node> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();

    for line in lines {
        let line = line?;

        let name = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .first()
            .expect("The input is always valid")
            .to_string();

        tmp.push(name);
    }

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();
    lines.next();
    lines.next();

    for line in lines {
        let line = line?;
        let node = Node::new(&line, &tmp);
        nodes.push(node);
    }

    Ok((instructions, nodes))
}

fn solve_part_one(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let (instructions, nodes) = parse_input(filename)?;
    let mut answer = 0;

    let mut current_node = nodes
        .iter()
        .find(|&node| node.value == "AAA")
        .expect("The input is always valid");

    let mut current_instruction_index = 0;
    while current_node.value != "ZZZ" {
        if current_instruction_index >= instructions.len() {
            current_instruction_index = 0;
        }

        let direction = instructions.chars().nth(current_instruction_index).unwrap();
        match direction {
            'L' => current_node = &nodes[current_node.left],
            'R' => current_node = &nodes[current_node.right],
            _ => unreachable!(),
        }

        current_instruction_index += 1;
        answer += 1;
        println!("{}", answer);
    }

    Ok(answer)
}

fn main() {
    let filename = "input.txt";
    let answer = solve_part_one(filename).unwrap();
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use crate::solve_part_one;

    #[test]
    fn test_solve_part_one() {
        let filename = "test.txt";
        assert_eq!(solve_part_one(filename).unwrap(), 6);
    }
}
