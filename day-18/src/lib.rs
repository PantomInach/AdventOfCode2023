use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Site {
    Trench,
    UnDig,
    OutSite,
    InSite,
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unimplemented!("No such direction"),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unimplemented!("No such direction"),
        })
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn go(&self, dir: &Direction) -> Position {
        self.go_steps(dir, &1)
    }

    fn go_steps(&self, dir: &Direction, steps: &i64) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - steps,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + steps,
            },
            Direction::Left => Position {
                x: self.x - steps,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + steps,
                y: self.y,
            },
        }
    }
}

struct DigMap {
    map: Vec<Vec<Site>>,
    width: usize,
    height: usize,
}

impl FromStr for DigMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction: Vec<(Direction, usize)> = s
            .lines()
            .map(|l| {
                let mut elements = l.split(' ');
                let dir = Direction::from_str(elements.next().unwrap()).unwrap();
                let steps = usize::from_str_radix(elements.next().unwrap(), 10).unwrap();
                (dir, steps)
            })
            .collect();

        let mut diged_locations: HashMap<Position, Site> = HashMap::new();
        let mut pos = Position { x: 0, y: 0 };

        diged_locations.insert(pos, Site::Trench);

        instruction.iter().for_each(|(dir, steps)| {
            for _ in 0..*steps {
                pos = pos.go(dir);
                diged_locations.insert(pos, Site::Trench);
            }
        });
        let min_y = diged_locations.keys().map(|pos| pos.y).min().unwrap();
        let max_y = diged_locations.keys().map(|pos| pos.y).max().unwrap();
        let min_x = diged_locations.keys().map(|pos| pos.x).min().unwrap();
        let max_x = diged_locations.keys().map(|pos| pos.x).max().unwrap();

        let map: Vec<Vec<Site>> = (min_y..=max_y)
            .map(move |y| {
                (min_x..=max_x)
                    .flat_map(|x| {
                        diged_locations
                            .get(&Position { x, y })
                            .or(Some(&Site::UnDig))
                    })
                    .cloned()
                    .collect()
            })
            .collect();
        Ok(DigMap {
            map,
            width: (min_x.abs() + max_x.abs() + 1) as usize,
            height: (min_y.abs() + max_y.abs() + 1) as usize,
        })
    }
}

impl DigMap {
    fn fill_inside(&mut self) {
        let mut sweep_line = vec![false; self.width];
        (0..self.height).for_each(|y| {
            let mut inside: usize = 0;
            self.map[y] = self.map[y]
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, s)| match s {
                    Site::Trench => {
                        if sweep_line[i] {
                            inside += 1;
                        }
                        sweep_line[i] = true;
                        s
                    }
                    Site::UnDig => {
                        sweep_line[i] = false;
                        if inside % 2 == 1 {
                            Site::InSite
                        } else {
                            Site::OutSite
                        }
                    }
                    _ => s,
                })
                .collect();
        })
    }

    fn count_volume(&self) -> u64 {
        self.map
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|s| match s {
                        Site::Trench => true,
                        Site::UnDig => false,
                        Site::OutSite => false,
                        Site::InSite => true,
                    })
                    .count()
            })
            .sum::<usize>() as u64
    }
}

struct Shoelace {
    points: Vec<Position>,
    boarder_points: i64,
}

impl FromStr for Shoelace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<(Direction, i64)> = s
            .lines()
            .map(|v| {
                let num = i64::from_str_radix(&v.split(' ').last().unwrap()[2..7], 16).unwrap();
                let dir: Direction = v.get(v.len() - 2..).unwrap().chars().nth(0).unwrap().into();
                (dir, num)
            })
            .collect();

        let mut boarder_points = 1;
        let mut pos = Position { x: 0, y: 0 };
        let mut corners: Vec<Position> = vec![pos];
        // let mut corners: Vec<Position> = vec![];

        instructions.iter().for_each(|(dir, steps)| {
            pos = pos.go_steps(dir, steps);
            corners.push(pos);
            boarder_points += steps;
        });
        corners.push(Position { x: 0, y: 0 });

        Ok(Shoelace {
            points: corners,
            boarder_points,
        })
    }
}

impl Shoelace {
    fn interiour_area(&self) -> i64 {
        self.points
            .windows(2)
            .fold(0_i64, |acc, x| {
                let pos1 = x.first().unwrap();
                let pos2 = x.get(1).unwrap();
                acc + (pos1.y + pos2.y)
                    .checked_mul(pos1.x - pos2.x)
                    .expect("Overflow while multiplying")
            })
            .abs()
            / 2
    }

    fn area(&self) -> i64 {
        self.interiour_area() + self.boarder_points / 2 + 1
    }
}

pub fn process_part1(input: &str) -> u64 {
    // Solved by building the dig site and using a sweep line algorithm to determine the inside and
    // outside points.
    let mut dig_map = DigMap::from_str(input).unwrap();
    dig_map.fill_inside();
    dig_map.count_volume()
}

pub fn process_part2(input: &str) -> u64 {
    // Solved using using the shoelace formular (https://en.wikipedia.org/wiki/Shoelace_formula)
    Shoelace::from_str(input).unwrap().area() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(62_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(952408144115_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_instructions() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)";
        let sl = Shoelace::from_str(input).unwrap();
        let res = vec![
            Position { x: 0, y: 0 },
            Position { x: 461937, y: 0 },
            Position {
                x: 461937,
                y: 56407,
            },
            Position { x: 0, y: 0 },
        ];
        assert_eq!(res, sl.points);
    }
}
