use std::{collections::HashMap, str::FromStr, u64};

#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
enum Item {
    Round,
    Rock,
    Empty,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Item {
    fn from_char(c: &char) -> Item {
        match c {
            'O' => Item::Round,
            '#' => Item::Rock,
            '.' => Item::Empty,
            _ => unimplemented!("Not possible"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Field {
    field: Vec<Vec<Item>>,
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field: Vec<Vec<Item>> = s
            .lines()
            .map(|l| l.chars().map(|c| Item::from_char(&c)).collect())
            .collect();
        Ok(Field { field })
    }
}

impl Field {
    fn transpose(&self) -> Vec<Vec<Item>> {
        transpose(&self.field)
    }

    fn push(&self, dir: Direction) -> Field {
        Field {
            field: match dir {
                Direction::Up => transpose(&self.transpose().iter().map(push_row_left).collect()),
                Direction::Right => self.field.iter().map(push_row_right).collect(),
                Direction::Down => {
                    transpose(&self.transpose().iter().map(push_row_right).collect())
                }
                Direction::Left => self.field.iter().map(push_row_left).collect(),
            },
        }
    }

    fn cycle(&self) -> Field {
        self.push(Direction::Up)
            .push(Direction::Left)
            .push(Direction::Down)
            .push(Direction::Right)
    }
    fn count(&self) -> u64 {
        self.field
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|item| item == &&Item::Round).count() * (i + 1))
            .sum::<usize>() as u64
    }
}

fn push_row_left(row: &Vec<Item>) -> Vec<Item> {
    let mut new_row = vec![Item::Empty; row.len()];
    let mut last_free_space = 0;
    row.iter().enumerate().for_each(|(i, item)| match item {
        Item::Empty => (),
        Item::Rock => {
            new_row[i] = Item::Rock;
            last_free_space = i + 1;
        }
        Item::Round => {
            new_row[last_free_space] = Item::Round;
            last_free_space += 1;
        }
    });
    new_row
}

fn push_row_right(row: &Vec<Item>) -> Vec<Item> {
    let mut new_row = vec![Item::Empty; row.len()];
    let mut last_free_space = new_row.len() - 1;
    row.iter()
        .enumerate()
        .rev()
        .for_each(|(i, item)| match item {
            Item::Empty => (),
            Item::Rock => {
                new_row[i] = Item::Rock;
                last_free_space = i.saturating_sub(1);
            }
            Item::Round => {
                new_row[last_free_space] = Item::Round;
                last_free_space -= 1;
            }
        });
    new_row
}

fn transpose(field: &Vec<Vec<Item>>) -> Vec<Vec<Item>> {
    (0..field.first().unwrap().len())
        .map(|col| (0..field.len()).map(|row| field[row][col]).collect())
        .collect()
}

pub fn process_part1(input: &str) -> u64 {
    Field::from_str(input).unwrap().push(Direction::Up).count()
}

pub fn process_part2(input: &str) -> u64 {
    const NUM_ROTATIONS: i64 = 1_000_000_000;
    let mut memory: HashMap<Field, i64> = HashMap::new();
    let mut field = Field::from_str(input).unwrap();
    let (index, i) = (0..NUM_ROTATIONS)
        .find_map(|i| {
            if let Some(index) = memory.insert(field.clone(), i) {
                return Some((index, i));
            }
            field = field.cycle();
            None
        })
        .unwrap();
    memory
        .iter()
        .find(|(_, v)| **v == (NUM_ROTATIONS - index - 1) % (i - index) + index + 1)
        .unwrap()
        .0
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(136_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(64_u64, process_part2(input));
    }

    #[test]
    fn test_transpose_equal() {
        let input = ".#O
.#O
.#O";
        let field = Field::from_str(input).unwrap();
        let res = vec![
            vec![Item::Empty, Item::Empty, Item::Empty],
            vec![Item::Rock, Item::Rock, Item::Rock],
            vec![Item::Round, Item::Round, Item::Round],
        ];
        assert_eq!(field.transpose(), res);
    }

    #[test]
    fn test_cycle() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let field = Field::from_str(input).unwrap();
        let input_after_1_cycle = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let field_1_rot = Field::from_str(input_after_1_cycle).unwrap();
        assert_eq!(field.cycle().field, field_1_rot.field);
        let input = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        assert_eq!(
            field.cycle().cycle().field,
            Field::from_str(input).unwrap().field
        );
    }

    #[test]
    fn test_push() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let input_up = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        let input_right = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";
        let origin = Field::from_str(input).unwrap();
        let up_push = Field::from_str(input_up).unwrap();
        let right_push = Field::from_str(input_right).unwrap();
        assert_eq!(origin.push(Direction::Up).field, up_push.field);
        assert_eq!(origin.push(Direction::Right).field, right_push.field);
    }

    #[test]
    fn test_push_right() {
        let v = vec![
            Item::Empty,
            Item::Round,
            Item::Empty,
            Item::Rock,
            Item::Empty,
        ];
        let right = vec![
            Item::Empty,
            Item::Empty,
            Item::Round,
            Item::Rock,
            Item::Empty,
        ];
        assert_eq!(right, push_row_right(&v));
    }

    #[test]
    fn test_some() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let field = Field::from_str(input).unwrap();
        let field2 = Field::from_str(input).unwrap();
        assert_eq!(field.cycle(), field2.cycle());
    }
}
