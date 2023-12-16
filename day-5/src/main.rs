#[derive(Debug, PartialEq)]
struct Map {
    source: u32,
    destination: u32,
    step: u32,
}

impl Map {
    fn new(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let values: Vec<u32> = line
            .split_whitespace()
            .filter_map(|value| value.parse::<u32>().ok())
            .collect();

        Ok(Map {
            source: values[1],
            destination: values[0],
            step: values[2],
        })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let input = "50 98 2";

        let expected_map = Map {
            source: 98,
            destination: 50,
            step: 2,
        };

        assert_eq!(Map::new(input).unwrap(), expected_map);
    }
}
