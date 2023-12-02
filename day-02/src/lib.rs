use std::str::FromStr;

const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

#[derive(Debug)]
struct Colors {
    r: u64,
    g: u64,
    b: u64,
}

struct Game {
    number: u64,
    colors: Vec<Colors>,
}

impl FromStr for Colors {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        s.trim().split(", ").for_each(|nc| {
            if nc.contains("red") {
                r += nc[..nc.len() - 4].parse::<u64>().unwrap();
            } else if nc.contains("green") {
                g += nc[..nc.len() - 6].parse::<u64>().unwrap();
            } else if nc.contains("blue") {
                b += nc[..nc.len() - 5].parse::<u64>().unwrap();
            }
        });
        Ok(Colors { r, g, b })
    }
}

impl Colors {
    fn valid(&self) -> bool {
        self.r <= MAX_RED && self.g <= MAX_GREEN && self.b <= MAX_BLUE
    }
}

impl Game {
    fn valid(&self) -> bool {
        self.colors.iter().all(|c| c.valid())
    }

    fn min_colors(&self) -> Colors {
        let mut res = Colors { r: 0, g: 0, b: 0 };
        self.colors.iter().for_each(|c| {
            res.r = res.r.max(c.r);
            res.g = res.g.max(c.g);
            res.b = res.b.max(c.b);
        });
        res
    }
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":").into_iter();
        let number = parts.next().unwrap()[5..].parse::<u64>().unwrap();
        let colors = parts
            .next()
            .unwrap()
            .split(";")
            .map(|s| Colors::from_str(s).unwrap())
            .collect();
        Ok(Game { number, colors })
    }
}

pub fn process_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| Game::from_str(l))
        .filter(|g_res| g_res.as_ref().is_ok_and(|g| g.valid()))
        .map(|g_res| g_res.unwrap().number)
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| Game::from_str(l).unwrap().min_colors())
        .map(|c| c.r * c.g * c.b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286_u64, process_part2(input));
    }
}
