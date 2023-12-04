use std::str::FromStr;

struct Card {
    id: usize,
    winning: Vec<u16>,
    you_have: Vec<u16>,
}

struct Pile {
    cards: Vec<Card>,
}

impl FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_n, numbers) = s.split_once(": ").unwrap();
        let card_numbers: Vec<Vec<u16>> = numbers
            .split(" | ")
            .map(|nums| {
                nums.split(' ')
                    .flat_map(|ns| ns.parse::<u16>())
                    .collect::<Vec<u16>>()
            })
            .collect();
        let id = card_n
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        Ok(Card {
            id,
            winning: card_numbers.get(0).unwrap().to_vec(),
            you_have: card_numbers.get(1).unwrap().to_vec(),
        })
    }
}

impl FromStr for Pile {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pile {
            cards: s.lines().map(|l| Card::from_str(l).unwrap()).collect(),
        })
    }
}

impl Card {
    fn value(&self) -> u64 {
        let n_winning = self.number_winnings();
        if n_winning == 0 {
            0_u64
        } else {
            2_u64.pow(n_winning as u32 - 1)
        }
    }

    fn number_winnings(&self) -> usize {
        self.you_have
            .iter()
            .filter(|yh| self.winning.contains(yh))
            .count()
    }
}

pub fn process_part1(input: &str) -> u64 {
    Pile::from_str(input)
        .unwrap()
        .cards
        .iter()
        .map(|card| card.value())
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let pile = Pile::from_str(input).unwrap();
    let mut card_count = vec![0_usize; pile.cards.len()];
    pile.cards.iter().for_each(|c| {
        let n = c.number_winnings();
        let inc_c = card_count.get(c.id - 1).unwrap() + 1;
        let _ = std::mem::replace(&mut card_count[&c.id - 1], inc_c);
        (c.id..pile.cards.len().min(c.id + n)).for_each(|i| {
            let inc = card_count.get(i).unwrap() + inc_c;
            let _ = std::mem::replace(&mut card_count[i], inc);
        });
    });
    card_count.iter().sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30_u64, process_part2(input));
    }
}
