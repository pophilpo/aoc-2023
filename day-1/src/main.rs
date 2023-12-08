use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    solve().unwrap();
}

fn solve() -> Result<i32, Box<dyn std::error::Error>> {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            println!("{}", line.unwrap());
        }
    }
    Ok(12)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
