use regex::Regex;
use std::collections::HashMap;

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn process_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let front = l.chars().find(|x| x.is_digit(10));
            let back = l.chars().rev().find(|x| x.is_digit(10));
            10 * (front.unwrap() as u64 - '0' as u64) + back.unwrap() as u64 - '0' as u64
        })
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let value_map: HashMap<String, u64> = vec![
        ("0", 0_u64),
        ("zero", 0_u64),
        ("1", 1_u64),
        ("one", 1_u64),
        ("2", 2_u64),
        ("two", 2_u64),
        ("3", 3_u64),
        ("three", 3_u64),
        ("4", 4_u64),
        ("four", 4_u64),
        ("5", 5_u64),
        ("five", 5_u64),
        ("6", 6_u64),
        ("six", 6_u64),
        ("7", 7_u64),
        ("seven", 7_u64),
        ("8", 8_u64),
        ("eight", 8_u64),
        ("9", 9_u64),
        ("nine", 9_u64),
    ]
    .into_iter()
    .map(|(s, n)| (s.to_owned(), n))
    .collect();

    let re_phrase: String = value_map
        .keys()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join("|");
    let re = Regex::new(&re_phrase).unwrap();
    let re_rev = Regex::new(&rev(&re_phrase)).unwrap();

    input
        .lines()
        .map(|l| {
            let l_rev = rev(l);
            let front = &re.captures(l).unwrap()[0];
            let back = &rev(&re_rev.captures(&l_rev).unwrap()[0]);
            10 * value_map.get(front).unwrap() + value_map.get(back).unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = process_part1(input);
    // let output = process_part2(input);
    println!("Output: {:?}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(142_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(281_u64, process_part2(input));
    }
}
