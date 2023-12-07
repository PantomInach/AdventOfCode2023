pub mod part1hand;
mod part2hand;

use itertools::Itertools;
use std::str::FromStr;

pub fn process_part1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(part1hand::Hand::from_str)
        .sorted()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    input
        .lines()
        .flat_map(part2hand::Hand::from_str)
        .sorted()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_add1() {
        let input = "33322 10
22233 1";
        assert_eq!(21_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_add2() {
        let input = "33333 1
22223 10
22233 100
22234 1000
22334 10000
22345 100000
23456 1000000";
        assert_eq!(1234567_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_add3() {
        let input = "A2222 1
T2222 10
92222 100
";
        assert_eq!(123_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905_u64, process_part2(input));
    }
}
