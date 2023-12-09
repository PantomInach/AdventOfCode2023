fn parse_str(s: &str) -> Vec<Vec<i64>> {
    s.lines()
        .map(|l| l.split(' ').map(|n| n.parse::<i64>().unwrap()).collect())
        .collect()
}

fn difference(nums: &[i64]) -> Vec<i64> {
    nums.iter()
        .as_slice()
        .windows(2)
        .map(|w| w.get(1).unwrap() - w.first().unwrap())
        .collect()
}

pub fn process_part1(input: &str) -> i64 {
    let lines = parse_str(input);
    lines
        .iter()
        .map(|l| {
            let mut stack: Vec<i64> = Vec::new();
            let mut current: Vec<i64> = l.to_vec();
            while current.iter().any(|n| *n != 0) {
                stack.push(*current.last().unwrap());
                current = difference(&current);
            }
            stack.iter().sum::<i64>()
        })
        .sum()
}

pub fn process_part2(input: &str) -> i64 {
    let lines = parse_str(input);
    lines
        .iter()
        .map(|l| {
            let mut stack: Vec<i64> = Vec::new();
            let mut current: Vec<i64> = l.to_vec();
            while current.iter().any(|n| *n != 0) {
                stack.push(*current.first().unwrap());
                current = difference(&current);
            }
            stack.iter().rev().fold(0, |acc, n| n - acc)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114_i64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2_i64, process_part2(input));
    }
}
