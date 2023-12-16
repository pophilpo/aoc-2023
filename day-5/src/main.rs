use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum MapVariants {
    None,
    SeedSoil,
    SoilFert,
    FertWater,
    WaterLight,
    LightTemp,
    TempHum,
    HumLoc,
}

#[derive(Debug, PartialEq)]
struct Map {
    source: i64,
    destination: i64,
    step: i64,
    diff: i64,
}

impl Map {
    fn new(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let values: Vec<i64> = line
            .split_whitespace()
            .filter_map(|value| value.parse::<i64>().ok())
            .collect();

        Ok(Map {
            source: values[1],
            destination: values[0],
            step: values[2],
            diff: (values[1] - values[0]).abs(),
        })
    }

    fn get_mapping(&self, input: i64) -> Option<i64> {
        if input < self.source || input > self.source + self.step {
            return None;
        } else {
            if self.source > self.destination {
                return Some(input - self.diff);
            } else {
                return Some(input + self.diff);
            }
        }
    }
}

fn solve_part_one(filename: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut answer = i64::MAX;

    let file = File::open(filename)?;

    let mut lines = io::BufReader::new(file).lines();
    let seeds_line = lines.next().expect("Input is always valid")?;

    let seeds: Vec<i64> = seeds_line
        .split_whitespace()
        .filter_map(|value| value.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let mut seed_to_soil_maps: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer_maps: Vec<Map> = Vec::new();
    let mut fertilizer_to_water_maps: Vec<Map> = Vec::new();
    let mut water_to_light_maps: Vec<Map> = Vec::new();
    let mut light_to_temp_maps: Vec<Map> = Vec::new();
    let mut temp_to_hum_maps: Vec<Map> = Vec::new();
    let mut hum_to_location_maps: Vec<Map> = Vec::new();

    let mut current_variant = MapVariants::None;

    for line in lines {
        let line = line?;

        match line.as_str() {
            "" => continue,
            "seed-to-soil map:" => current_variant = MapVariants::SeedSoil,
            "soil-to-fertilizer map:" => current_variant = MapVariants::SoilFert,
            "fertilizer-to-water map:" => current_variant = MapVariants::FertWater,
            "water-to-light map:" => current_variant = MapVariants::WaterLight,
            "light-to-temperature map:" => current_variant = MapVariants::LightTemp,
            "temperature-to-humidity map:" => current_variant = MapVariants::TempHum,
            "humidity-to-location map:" => current_variant = MapVariants::HumLoc,
            _ => {
                let map = Map::new(&line)?;
                match current_variant {
                    MapVariants::SeedSoil => seed_to_soil_maps.push(map),
                    MapVariants::SoilFert => soil_to_fertilizer_maps.push(map),
                    MapVariants::FertWater => fertilizer_to_water_maps.push(map),
                    MapVariants::WaterLight => water_to_light_maps.push(map),
                    MapVariants::LightTemp => light_to_temp_maps.push(map),
                    MapVariants::TempHum => temp_to_hum_maps.push(map),
                    MapVariants::HumLoc => hum_to_location_maps.push(map),
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut map_collection: Vec<Vec<Map>> = Vec::with_capacity(7);
    map_collection.push(seed_to_soil_maps);
    map_collection.push(soil_to_fertilizer_maps);
    map_collection.push(fertilizer_to_water_maps);
    map_collection.push(water_to_light_maps);
    map_collection.push(light_to_temp_maps);
    map_collection.push(temp_to_hum_maps);
    map_collection.push(hum_to_location_maps);

    for seed in seeds {
        let mut current_answer = seed;

        for maps in &map_collection {
            for map in maps {
                match map.get_mapping(current_answer) {
                    Some(new_answer) => {
                        current_answer = new_answer;
                        break;
                    }
                    None => continue,
                }
            }
        }

        if current_answer < answer {
            answer = current_answer;
        }
    }

    return Ok(answer);
}

fn main() {
    let answer = solve_part_one("input.txt").unwrap();
    println!("{}", answer);
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
            diff: 48,
        };

        assert_eq!(Map::new(input).unwrap(), expected_map);
    }

    #[test]
    fn test_get_mapping() {
        let input = "50 98 2";
        let map = Map::new(input).unwrap();

        assert_eq!(map.get_mapping(98), Some(50));
        assert_eq!(map.get_mapping(99), Some(51));
        assert_eq!(map.get_mapping(10), None);

        let input = "52 50 48";
        let map = Map::new(input).unwrap();
        assert_eq!(map.get_mapping(79), Some(81));
        assert_eq!(map.get_mapping(55), Some(57));

        let input = "20 10 15";
        let map = Map::new(input).unwrap();

        assert_eq!(map.get_mapping(11), Some(21));
        assert_eq!(map.get_mapping(25), Some(35));
        assert_eq!(map.get_mapping(10), Some(20));
        assert_eq!(map.get_mapping(9), None);
    }

    #[test]
    fn test_solve_part_one() {
        let filename = "test.txt";
        assert_eq!(solve_part_one(filename).unwrap(), 35);
    }
}
