use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Start,
    Garden,
    Rock,
    EvenReachable(usize),
    OddReachable(usize),
}

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|delta| Position {
                x: (self.x as i64).saturating_add(delta.0) as usize,
                y: (self.y as i64).saturating_add(delta.1) as usize,
            })
            .collect()
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Garden,
                        '#' => Tile::Rock,
                        'S' => Tile::Start,
                        _ => unreachable!("Can't only happen if faulty input."),
                    })
                    .collect()
            })
            .collect();
        Ok(Map { map })
    }
}

impl Map {
    fn start_point(&self) -> Position {
        self.map
            .iter()
            .enumerate()
            .find_map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .find_map(|(x, t)| (t == &Tile::Start).then_some(x))
                    .map(|x| Position { x, y })
            })
            .unwrap()
    }

    fn take_steps(&mut self, n: usize) -> u64 {
        let mut even_reachable = 1;
        let mut odd_reachable = 0;
        let mut added_last: Vec<Position> = vec![self.start_point()];
        let mut queue: Vec<Position> = Vec::new();
        for i in 1..=n {
            if added_last.is_empty() {
                break;
            }
            std::mem::swap(&mut added_last, &mut queue);
            while let Some(pos) = queue.pop() {
                pos.neighbors().iter().for_each(|neigh| {
                    if let Some(Tile::Garden) = self.get_tile(neigh) {
                        self.map[neigh.y][neigh.x] = if i % 2 == 0 {
                            even_reachable += 1;
                            Tile::EvenReachable(i)
                        } else {
                            odd_reachable += 1;
                            Tile::OddReachable(i)
                        };
                        added_last.push(*neigh);
                    }
                })
            }
        }
        if n % 2 == 0 {
            even_reachable as u64
        } else {
            odd_reachable as u64
        }
    }

    fn get_tile(&self, pos: &Position) -> Option<&Tile> {
        self.map.get(pos.y).and_then(|v| v.get(pos.x))
    }

    fn parallel_worlds(&mut self, steps: usize) -> u64 {
        self.walk_every_thing();
        let hs = self.map.len() / 2;
        let even_corners: usize = self
            .map
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|t| {
                        if let &&Tile::EvenReachable(k) = t {
                            k > hs
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum();
        let odd_corners: usize = self
            .map
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|t| {
                        if let &&Tile::OddReachable(k) = t {
                            k > hs
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum();
        let even_full: usize = self
            .map
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|t| matches!(t, Tile::EvenReachable(_)))
                    .count()
            })
            .sum::<usize>()
            + 1;
        let odd_full: usize = self
            .map
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|t| matches!(t, Tile::OddReachable(_)))
                    .count()
            })
            .sum();

        let n = (steps - hs) / self.map.len();

        ((n + 1) * (n + 1) * odd_full + n * n * even_full - (n + 1) * odd_corners
            + n * even_corners) as u64
    }

    fn walk_every_thing(&mut self) {
        self.take_steps(usize::MAX);
    }
}

pub fn process_part1(input: &str) -> u64 {
    Map::from_str(input).unwrap().take_steps(64)
}

pub fn process_part2(input: &str) -> u64 {
    Map::from_str(input).unwrap().parallel_worlds(26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let mut map = Map::from_str(input).unwrap();
        map.take_steps(16);
        map.map.iter().for_each(|l| println!("{:?}", l));
        assert_eq!(16_u64, Map::from_str(input).unwrap().take_steps(6));
    }
}
