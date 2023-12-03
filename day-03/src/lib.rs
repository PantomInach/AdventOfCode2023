use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum FieldType {
    Empty,
    Part(Part),
    Symbol(char),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    id: u64,
    number: u64,
}

impl Part {
    fn new(id: u64, number: u64) -> Part {
        Part { id, number }
    }
}

#[derive(Debug)]
struct Field {
    field: Vec<Vec<FieldType>>,
}

impl Field {
    fn get(&self, x: usize, y: usize) -> Option<&FieldType> {
        self.field.get(x)?.get(y)
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&FieldType> {
        let fields: Vec<Option<&FieldType>> = vec![
            self.get(x - 1, y + 1),
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
            self.get(x - 1, y),
            self.get(x - 1, y),
            self.get(x - 1, y - 1),
            self.get(x, y - 1),
            self.get(x + 1, y - 1),
        ];
        fields.iter().flatten().copied().collect()
    }

    fn get_symbol_positions(&self) -> Vec<(usize, usize, &FieldType)> {
        self.field
            .iter()
            .enumerate()
            .flat_map(|(x, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, f)| matches!(f, FieldType::Symbol(_)))
                    .map(move |(y, f)| (x, y, f))
            })
            .collect()
    }

    fn get_gears(&self) -> Vec<(usize, usize)> {
        self.field
            .iter()
            .enumerate()
            .flat_map(|(x, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, f)| matches!(f, FieldType::Symbol('*')))
                    .map(move |(y, _)| (x, y))
            })
            .collect()
    }

    fn get_part_neighbors(&self, x: usize, y: usize) -> Vec<&Part> {
        self.get_neighbors(x, y)
            .iter()
            .filter_map(|f| match f {
                FieldType::Part(x) => Some(x),
                _ => None,
            })
            .unique()
            .collect()
    }
}

impl FromStr for Field {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let field: Vec<Vec<FieldType>> = s
            .lines()
            .map(|l| {
                let mut line: Vec<FieldType> = Vec::new();
                let mut iter = l.chars().enumerate().peekable();
                let mut part_number: u64 = 0;
                let mut part_number_start: Option<usize> = None;
                while let Some((i, c)) = iter.next() {
                    match c {
                        '.' => line.push(FieldType::Empty),
                        _ if c.is_ascii_digit() => {
                            part_number = part_number * 10 + c as u64 - '0' as u64;
                            if part_number_start.is_none() {
                                part_number_start = Some(i);
                            }
                            if !iter.peek().is_some_and(|(_, cc)| cc.is_ascii_digit()) {
                                let new_part = Part::new(id, part_number);
                                (part_number_start.unwrap()..=i).for_each(|_| {
                                    line.push(FieldType::Part(new_part.clone()));
                                });
                                id += 1;
                                part_number_start = None;
                                part_number = 0;
                            }
                        }
                        _ => line.push(FieldType::Symbol(c)),
                    }
                }
                line
            })
            .collect();
        Ok(Field { field })
    }
}

pub fn process_part1(input: &str) -> u64 {
    let field = Field::from_str(input).unwrap();
    field
        .get_symbol_positions()
        .iter()
        .flat_map(|(x, y, _)| field.get_neighbors(*x, *y))
        .flat_map(|f| match f {
            FieldType::Part(x) => Some(x),
            _ => None,
        })
        .sorted()
        .dedup_by(|x, y| x.id == y.id)
        .map(|p| p.number)
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let field = Field::from_str(input).unwrap();
    let binding = field.get_gears();
    binding
        .iter()
        .map(|(x, y)| field.get_part_neighbors(*x, *y))
        .filter(|neig| neig.len() == 2)
        .map(|ps| ps.iter().map(|p| p.number).product::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835_u64, process_part2(input));
    }
}
