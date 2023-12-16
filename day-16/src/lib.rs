use std::{ops, str::FromStr};

#[derive(Clone, PartialEq, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Mirror {
    Empty = 0,
    Vert = 1,
    Hori = 2,
    DiagDown = 3,
    DiagUp = 4,
}

#[derive(Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
}

struct Contraption {
    field: Vec<Vec<Mirror>>,
    energized: Vec<Vec<[bool; 5]>>,
}

impl FromStr for Contraption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field: Vec<Vec<Mirror>> = s
            .lines()
            .map(|l| l.chars().map(Mirror::from_char).collect())
            .collect();
        let energized = vec![vec![[false; 5]; field.first().unwrap().len()]; field.len()];
        Ok(Contraption { field, energized })
    }
}

impl ops::Add for Position {
    fn add(self, rhs: Self) -> Position {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    type Output = Position;
}

impl Direction {
    fn move_further(&self, pos: &Position) -> (Position, Direction) {
        match self {
            Direction::Up => (Position { x: 0, y: -1 } + pos.clone(), *self),
            Direction::Down => (Position { x: 0, y: 1 } + pos.clone(), *self),
            Direction::Left => (Position { x: -1, y: 0 } + pos.clone(), *self),
            Direction::Right => (Position { x: 1, y: 0 } + pos.clone(), *self),
        }
    }
}

impl Mirror {
    fn from_char(c: char) -> Mirror {
        match c {
            '-' => Mirror::Hori,
            '|' => Mirror::Vert,
            '/' => Mirror::DiagUp,
            '\\' => Mirror::DiagDown,
            '.' => Mirror::Empty,
            _ => unimplemented!("Not a valid carachter"),
        }
    }

    fn new_dirs(&self, dir: &Direction) -> Vec<Direction> {
        match self {
            Mirror::Empty => vec![*dir],
            Mirror::Vert => {
                if dir == &Direction::Up || dir == &Direction::Down {
                    vec![*dir]
                } else {
                    vec![Direction::Up, Direction::Down]
                }
            }
            Mirror::Hori => {
                if dir == &Direction::Left || dir == &Direction::Right {
                    vec![*dir]
                } else {
                    vec![Direction::Left, Direction::Right]
                }
            }
            Mirror::DiagDown => vec![match dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }],
            Mirror::DiagUp => vec![match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }],
        }
    }

    fn redirect(&self, pos: &Position, dir: &Direction) -> Vec<(Position, Direction)> {
        self.new_dirs(dir)
            .iter()
            .map(|d| d.move_further(pos))
            .collect()
    }
}

impl Contraption {
    fn process_beam(&mut self, pos: &Position, dir: &Direction) {
        if pos.x < 0 || pos.y < 0 {
            return;
        }
        if let Some(mirror) = self
            .field
            .get(pos.y as usize)
            .and_then(|v| v.get(pos.x as usize))
        {
            if self.energized[pos.y as usize][pos.x as usize][*dir as usize] {
                return;
            }
            self.energized[pos.y as usize][pos.x as usize][*dir as usize] = true;
            mirror
                .redirect(pos, dir)
                .iter()
                .for_each(|(p, d)| self.process_beam(p, d))
        }
    }

    fn count_energized(&self) -> u64 {
        self.energized
            .iter()
            .map(|v| v.iter().filter(|e| e.iter().any(|b| *b)).count())
            .sum::<usize>() as u64
    }

    fn reset(&mut self) {
        self.energized =
            vec![vec![[false; 5]; self.field.first().unwrap().len()]; self.field.len()];
    }
}

pub fn process_part1(input: &str) -> u64 {
    let mut contraption = Contraption::from_str(input).unwrap();
    contraption.process_beam(&Position { x: 0, y: 0 }, &Direction::Right);
    contraption.count_energized()
}

pub fn process_part2(input: &str) -> u64 {
    let mut contraption = Contraption::from_str(input).unwrap();
    let len_x = contraption.field.len() as i64;
    let len_y = contraption.field.first().unwrap().len() as i64;
    let max_down = (0..len_x)
        .map(|x| {
            contraption.reset();
            contraption.process_beam(&Position { x, y: 0 }, &Direction::Down);
            contraption.count_energized()
        })
        .max()
        .unwrap();
    let max_up = (0..len_x)
        .map(|x| {
            contraption.reset();
            contraption.process_beam(&Position { x, y: len_y - 1 }, &Direction::Up);
            contraption.count_energized()
        })
        .max()
        .unwrap();
    let max_right = (0..len_y)
        .map(|y| {
            contraption.reset();
            contraption.process_beam(&Position { x: 0, y }, &Direction::Right);
            contraption.count_energized()
        })
        .max()
        .unwrap();
    let max_left: u64 = (0..len_y)
        .map(|y| {
            contraption.reset();
            contraption.process_beam(&Position { x: len_x - 1, y }, &Direction::Left);
            contraption.count_energized()
        })
        .max()
        .unwrap();
    max_up.max(max_down).max(max_left).max(max_right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(46_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(51_u64, process_part2(input));
    }
}
