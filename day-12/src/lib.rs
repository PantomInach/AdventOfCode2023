use std::{collections::HashMap, str::FromStr};

#[derive(Clone, PartialEq, Debug, Copy)]
enum State {
    Good,
    Broken,
    Unknown,
}

struct Line {
    springs: Vec<State>,
    backup: Vec<usize>,
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
        Ok(Line { springs, backup })
    }
}

impl Line {
    fn dp(&self, pos: usize, at_group: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
        if let Some(res) = cache.get(&(pos, at_group)) {
            return *res;
        }
        let spring_op = self.springs.get(pos);
        if let Some(spring) = spring_op {
            let mut res = 0;
            if spring == &State::Good || spring == &State::Unknown {
                // Let spring be good
                res += self.dp(pos + 1, at_group, cache);
            }
            if (spring == &State::Unknown || spring == &State::Broken)
                && at_group < self.backup.len()
            {
                let check_group = if let Some(group) = self
                    .springs
                    .get(pos..pos + self.backup.get(at_group).unwrap())
                {
                    // Can take group
                    !group.contains(&State::Good)
                } else {
                    false
                };
                let check_group_end = if let Some(next_spring) =
                    self.springs.get(pos + self.backup.get(at_group).unwrap())
                {
                    next_spring != &State::Broken
                } else {
                    true
                };
                if check_group && check_group_end {
                    res += self.dp(
                        pos + self.backup.get(at_group).unwrap() + 1,
                        at_group + 1,
                        cache,
                    );
                }
            }
            cache.insert((pos, at_group), res);
            res
        } else {
            // End of springs => check if all groups were taken
            if at_group == self.backup.len() {
                cache.insert((pos, at_group), 1);
                1
            } else {
                cache.insert((pos, at_group), 0);
                0
            }
        }
    }

    fn expand(&self) -> Line {
        let mut springs = self.springs.clone();
        springs.push(State::Unknown);
        springs = springs.repeat(5);
        springs.pop();
        let backup = self.backup.repeat(5);
        Line { springs, backup }
    }
}

pub fn process_part1(input: &str) -> u64 {
    let lines: Vec<Line> = input.lines().flat_map(Line::from_str).collect();
    lines.iter().map(|l| l.dp(0, 0, &mut HashMap::new())).sum()
}

pub fn process_part2(input: &str) -> u64 {
    let lines: Vec<Line> = input
        .lines()
        .flat_map(Line::from_str)
        .map(|l| l.expand())
        .collect();
    lines.iter().map(|l| l.dp(0, 0, &mut HashMap::new())).sum()
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
    fn test_dp() {
        let input = "?###???????? 3,2,1";
        println!("{:?}", Line::from_str(input).unwrap().springs);
        println!("{:?}", Line::from_str(input).unwrap().backup);
        assert_eq!(
            10_u64,
            Line::from_str(input).unwrap().dp(0, 0, &mut HashMap::new())
        );
    }

    #[test]
    fn test_process_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(525152_u64, process_part2(input));
    }
}
