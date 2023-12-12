use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, PartialEq, Debug)]
enum State {
    Good,
    Broken,
    Unknown,
}

struct Line {
    springs: Vec<State>,
    backup: Vec<usize>,
    unknowns: usize,
    unknown_broken: usize,
}

impl State {
    fn from(c: &char) -> State {
        match c {
            '.' => State::Good,
            '#' => State::Broken,
            '?' => State::Unknown,
            _ => unimplemented!("Cant happen"),
        }
    }
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs_str, backup_str) = s.split_once(' ').unwrap();
        let springs: Vec<State> = springs_str.chars().map(|c| State::from(&c)).collect();
        let backup: Vec<usize> = backup_str
            .split(',')
            .flat_map(|n| n.parse::<usize>())
            .collect();
        let unknowns: usize = springs.iter().filter(|s| s == &&State::Unknown).count();
        let unknown_broken: usize =
            backup.iter().sum::<usize>() - springs.iter().filter(|s| s == &&State::Broken).count();
        Ok(Line {
            springs,
            backup,
            unknowns,
            unknown_broken,
        })
    }
}

impl Line {
    fn valid(&self, resolve: &Vec<&State>) -> bool {
        if resolve.len() != self.unknowns {
            return false;
        }
        if resolve.iter().filter(|s| s == &&&State::Broken).count() != self.unknown_broken {
            return false;
        }
        let mut temp = self.springs.clone();
        self.springs
            .iter()
            .enumerate()
            .filter(|(_, s)| s == &&State::Unknown)
            .zip(resolve)
            .for_each(|((i, _), s)| temp[i] = s.clone().clone());
        let mut length: usize = 1;
        let mut groups: Vec<usize> = temp.iter().fold(vec![0], |mut acc, x| {
            match x {
                State::Good => {
                    if acc.last().unwrap() != &0_usize {
                        acc.push(0_usize);
                        length += 1;
                    }
                }
                State::Broken => acc[length - 1] += 1,
                State::Unknown => unimplemented!("Cant happen"),
            };
            acc
        });
        if groups.last().unwrap() == &0 {
            groups.pop();
        }
        groups == self.backup
    }

    fn brute_force(&self) -> u64 {
        (0..self.unknowns)
            .map(|_| vec![&State::Good, &State::Broken])
            .multi_cartesian_product()
            .filter(|v| self.valid(v))
            .count() as u64
    }
}

pub fn process_part1(input: &str) -> u64 {
    let lines: Vec<Line> = input.lines().flat_map(|l| Line::from_str(l)).collect();
    lines.iter().map(|l| l.brute_force()).sum()
}

pub fn process_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21_u64, process_part1(input));
    }

    #[test]
    fn test_brute_force() {
        let input = "?###???????? 3,2,1";
        assert_eq!(10_u64, Line::from_str(input).unwrap().brute_force());
    }

    #[test]
    fn test_valid() {
        let input = "..##..## 2,2";
        assert!(Line::from_str(input).unwrap().valid(&vec![]));
        let input = "..##..?# 2,2";
        assert!(Line::from_str(input).unwrap().valid(&vec![&State::Broken]));
        assert!(!Line::from_str(input).unwrap().valid(&vec![&State::Good]));

        let input = "?###???????? 3,2,1";
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Good,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Good,
            &State::Good,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Good,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Broken,
            &State::Good,
            &State::Good,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Good,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Broken,
            &State::Good,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Good,
            &State::Broken,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
        let resolve = vec![
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Good,
            &State::Broken,
            &State::Broken,
            &State::Good,
            &State::Broken,
        ];
        assert!(Line::from_str(input).unwrap().valid(&resolve));
    }

    #[test]
    fn test_process_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(525152_u64, process_part1(input));
    }
}
