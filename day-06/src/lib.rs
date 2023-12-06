use std::str::FromStr;

struct Races {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl FromStr for Races {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times_str = lines.next().unwrap();
        let distances_str = lines.next().unwrap();

        let times: Vec<u64> = times_str
            .split_once(':')
            .unwrap()
            .1
            .split(' ')
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();
        let distances: Vec<u64> = distances_str
            .split_once(':')
            .unwrap()
            .1
            .split(' ')
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();

        Ok(Races { times, distances })
    }
}

pub fn process_part1(input: &str) -> u64 {
    let races = Races::from_str(input).unwrap();
    races
        .times
        .iter()
        .zip(races.distances.iter())
        .map(|(t, min_d)| {
            (0..=*t)
                .map(|hold_time| (t - hold_time) * hold_time)
                .filter(|d| d > min_d)
                .count()
        })
        .product::<usize>() as u64
}

pub fn process_part2(input: &str) -> u64 {
    let races = Races::from_str(input).unwrap();
    let time = races
        .times
        .iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = races
        .distances
        .iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    (0..=time)
        .map(|hold_time| (time - hold_time) * hold_time)
        .filter(|d| d > &distance)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "Time:      7  15    30
Distance:  9  40  200";
        assert_eq!(71503_u64, process_part2(input));
    }
}
