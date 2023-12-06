use std::str::FromStr;

#[derive(Debug)]
struct Mapping {
    source_start: u64,
    source_end: u64,
    dest_start: u64,
}

struct Mappings {
    maps: Vec<Mapping>,
}

struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Mappings {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mappings {
            maps: s.lines().flat_map(|l| Mapping::from_str(l).ok()).collect(),
        })
    }
}

impl Mappings {
    fn get(&self, index: &u64) -> u64 {
        self.maps
            .iter()
            .find_map(|m| m.get(index))
            .unwrap_or(*index)
    }
}

impl FromStr for Mapping {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Result<Vec<u64>, Self::Err> = s.split(' ').map(|n| n.parse::<u64>()).collect();
        if let Ok(mut nums) = numbers {
            let range = nums.pop().unwrap();
            let start_source = nums.pop().unwrap();
            let start_dest = nums.pop().unwrap();
            Ok(Mapping {
                source_start: start_source,
                source_end: start_source + range,
                dest_start: start_dest,
            })
        } else {
            Err(numbers.unwrap_err())
        }
    }
}

impl Mapping {
    fn get(&self, index: &u64) -> Option<u64> {
        if self.source_start <= *index && *index <= self.source_end {
            Some(index - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

fn parse_seeds(s: &str) -> Vec<u64> {
    s.split_once(':')
        .unwrap()
        .1
        .split(' ')
        .flat_map(|n| n.parse::<u64>().ok())
        .collect()
}

pub fn process_part1(input: &str) -> u64 {
    let blocks: Vec<&str> = input
        .split("\n\n")
        // .for_each(|block| println!("Start of block---- {}", block));
        .collect();
    let seeds = parse_seeds(blocks.first().unwrap());
    let seed_to_soil: Mappings = Mappings::from_str(blocks.get(1).unwrap()).unwrap();
    let soil_to_fertilizer: Mappings = Mappings::from_str(blocks.get(2).unwrap()).unwrap();
    let fertilizer_to_water: Mappings = Mappings::from_str(blocks.get(3).unwrap()).unwrap();
    let water_to_light: Mappings = Mappings::from_str(blocks.get(4).unwrap()).unwrap();
    let light_to_temperature: Mappings = Mappings::from_str(blocks.get(5).unwrap()).unwrap();
    let temperature_to_humidity: Mappings = Mappings::from_str(blocks.get(6).unwrap()).unwrap();
    let humidity_to_location: Mappings = Mappings::from_str(blocks.get(7).unwrap()).unwrap();
    seeds
        .iter()
        .map(|s| {
            humidity_to_location.get(&temperature_to_humidity.get(
                &light_to_temperature.get(
                    &water_to_light.get(
                        &fertilizer_to_water.get(&soil_to_fertilizer.get(&seed_to_soil.get(s))),
                    ),
                ),
            ))
        })
        .min()
        .unwrap()
}

pub fn process_part2(input: &str) -> u64 {
    let blocks: Vec<&str> = input
        .split("\n\n")
        // .for_each(|block| println!("Start of block---- {}", block));
        .collect();
    let seeds: Vec<u64> = parse_seeds(blocks.first().unwrap())
        .as_slice()
        .chunks(2)
        .flat_map(|chunk| (chunk[0]..chunk[1] + chunk[0]))
        .collect();
    let seed_to_soil: Mappings = Mappings::from_str(blocks.get(1).unwrap()).unwrap();
    let soil_to_fertilizer: Mappings = Mappings::from_str(blocks.get(2).unwrap()).unwrap();
    let fertilizer_to_water: Mappings = Mappings::from_str(blocks.get(3).unwrap()).unwrap();
    let water_to_light: Mappings = Mappings::from_str(blocks.get(4).unwrap()).unwrap();
    let light_to_temperature: Mappings = Mappings::from_str(blocks.get(5).unwrap()).unwrap();
    let temperature_to_humidity: Mappings = Mappings::from_str(blocks.get(6).unwrap()).unwrap();
    let humidity_to_location: Mappings = Mappings::from_str(blocks.get(7).unwrap()).unwrap();
    seeds
        .iter()
        .map(|s| {
            humidity_to_location.get(&temperature_to_humidity.get(
                &light_to_temperature.get(
                    &water_to_light.get(
                        &fertilizer_to_water.get(&soil_to_fertilizer.get(&seed_to_soil.get(s))),
                    ),
                ),
            ))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46_u64, process_part2(input));
    }
}
