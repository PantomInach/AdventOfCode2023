use std::str::FromStr;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Item {
    Ash,
    Rock,
}

#[derive(Debug)]
struct Field {
    pattern: Vec<Vec<Item>>,
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Item::Ash,
                        '#' => Item::Rock,
                        _ => unimplemented!("Cant happen"),
                    })
                    .collect()
            })
            .collect();
        Ok(Field { pattern })
    }
}

impl Field {
    fn find_hori_mirror(&self) -> Option<usize> {
        (0..self.pattern.len() - 1)
            .find(|i| {
                let space_to_right = self.pattern.len() - i - 2;
                let check_range: &usize = i.min(&space_to_right);
                (0..=*check_range).all(|j| {
                    self.pattern.get(i - j).unwrap() == self.pattern.get(i + j + 1).unwrap()
                })
            })
            .map(|u| u + 1)
    }

    fn find_near_miss_hor_mirror(&self) -> Option<usize> {
        (0..self.pattern.len() - 1)
            .find(|i| {
                let space_to_right = self.pattern.len() - i - 2;
                let check_range: &usize = i.min(&space_to_right);
                let s = (0..=*check_range)
                    .map(|j| {
                        count_not_same(
                            self.pattern.get(i - j).unwrap(),
                            self.pattern.get(i + j + 1).unwrap(),
                        )
                    })
                    .sum::<usize>();
                s == 1
            })
            .map(|u| u + 1)
    }

    fn transpose(&self) -> Field {
        let pattern = (0..self.pattern.first().unwrap().len())
            .map(|col| {
                (0..self.pattern.len())
                    .map(|row| self.pattern[row][col])
                    .collect()
            })
            .collect();
        Field { pattern }
    }

    fn get_value(&self) -> u64 {
        if let Some(hor) = self.find_hori_mirror() {
            (hor * 100) as u64
        } else {
            self.transpose().find_hori_mirror().unwrap_or(0) as u64
        }
    }

    fn get_value_near_miss(&self) -> u64 {
        if let Some(hor) = self.find_near_miss_hor_mirror() {
            (hor * 100) as u64
        } else {
            self.transpose().find_near_miss_hor_mirror().unwrap() as u64
        }
    }
}

fn count_not_same<T>(v1: &Vec<T>, v2: &Vec<T>) -> usize
where
    T: PartialEq,
{
    v1.iter().zip(v2.iter()).filter(|(i1, i2)| i1 != i2).count()
}

pub fn process_part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .flat_map(Field::from_str)
        .map(|f| f.get_value())
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    input
        .split("\n\n")
        .flat_map(Field::from_str)
        .map(|f| f.get_value_near_miss())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(405_u64, process_part1(input));
    }

    #[test]
    fn transpose() {
        let f = Field::from_str(".#\n..").unwrap();
        let ft = Field::from_str("..\n#.").unwrap();
        assert_eq!(f.transpose().pattern, ft.pattern);
    }

    #[test]
    fn test_process_part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(400_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(300_u64, process_part2(input));
    }
    #[test]
    fn test_process_part2_v3() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(100_u64, process_part2(input));
    }
}
