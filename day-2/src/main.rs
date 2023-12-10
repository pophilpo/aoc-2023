use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines = read_lines(filename)?;

    for line in lines {
        println!("{}", line?);
    }

    Ok(8)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let filename = "test.txt";
        assert_eq!(solve(filename).unwrap(), 8);
    }
}
