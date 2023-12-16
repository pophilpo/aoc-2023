use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Race {
    duration: f32,
    record_distance: f32,
}

impl Race {
    fn new(duration: f32, record_distance: f32) -> Self {
        Self {
            duration,
            record_distance,
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<Race>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;

    let mut lines = io::BufReader::new(file).lines();

    let race_durations: Vec<f32> = lines
        .next()
        .expect("Input is always valid")?
        .split_whitespace()
        .filter_map(|value| value.parse::<f32>().ok())
        .collect();

    let race_records: Vec<f32> = lines
        .next()
        .expect("Input is always valid")?
        .split_whitespace()
        .filter_map(|value| value.parse::<f32>().ok())
        .collect();

    let races: Vec<Race> = race_durations
        .into_iter()
        .zip(race_records.into_iter())
        .map(|(duration, record_distance)| Race::new(duration, record_distance))
        .collect();

    Ok(races)
}

fn solve_part_one(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    // Basically solve {time=(time-x)*x - record} and get int's between the answers
    // -x^2 +time*x - record = 0
    // d = (time*time) - 4 *(record*x)/

    let races = read_input(filename)?;
    let mut answer = 1;

    for race in races {
        let mut tmp = 0;
        let d = ((race.duration * race.duration) - 4.0 * race.record_distance).sqrt();
        let mut x_1 = (race.duration - d) / 2.0;
        let mut x_2 = (race.duration + d) / 2.0;

        // If the boundaries are whole, we skip them since they give the exact record time, not faster
        if x_1.fract() == 0.0 {
            x_1 = x_1 + 1.0;
        } else {
            x_1 = x_1.ceil();
        }

        if x_2.fract() == 0.0 {
            x_2 = x_2;
        } else {
            x_2 = x_2.ceil();
        }

        for _ in x_1 as u32..x_2 as u32 {
            tmp += 1;
        }
        answer *= tmp;
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
    fn test_solve_part_one() {
        let filename = "test.txt";
        assert_eq!(solve_part_one(filename).unwrap(), 288);
    }
}
