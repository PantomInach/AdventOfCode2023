use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
pub(crate) enum Card {
    Num(u8),
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
pub(crate) struct Hand {
    cards: Vec<Card>,
    pub bid: u64,
}

impl Card {
    fn from(c: &char) -> Result<Self, String> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::Num(10)),
            '2' => Ok(Card::Num(2)),
            '3' => Ok(Card::Num(3)),
            '4' => Ok(Card::Num(4)),
            '5' => Ok(Card::Num(5)),
            '6' => Ok(Card::Num(6)),
            '7' => Ok(Card::Num(7)),
            '8' => Ok(Card::Num(8)),
            '9' => Ok(Card::Num(9)),
            _ => unimplemented!("{}", format!("The char {} should not be in the input", c)),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').unwrap();
        let cards: Vec<Card> = cards_str.chars().flat_map(|c| Card::from(&c)).collect();
        let bid: u64 = bid_str.parse().unwrap();
        Ok(Hand { cards, bid })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_type().cmp(&other.get_type()) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => {
                let res = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|(c1, c2)| c1 != c2);
                if let Some((c1, c2)) = res {
                    Some(c2.cmp(c1))
                } else {
                    Some(std::cmp::Ordering::Equal)
                }
            }
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl Hand {
    fn frequency(&self) -> Vec<(&Card, usize)> {
        self.cards
            .iter()
            .clone()
            .unique()
            .map(|card| (card, self.cards.iter().filter(|c| c == &card).count()))
            .collect()
    }

    fn get_type(&self) -> Type {
        match 1 {
            _ if !self.n_of_a_kind(5).is_empty() => Type::FiveOfAKind,
            _ if !self.n_of_a_kind(4).is_empty() => Type::FourOfAKind,
            _ if !self.n_of_a_kind(3).is_empty() => {
                if self.n_of_a_kind(2).len() == 2 {
                    Type::FullHouse
                } else {
                    Type::ThreeOfAKind
                }
            }
            _ if self.n_of_a_kind(2).len() == 2 => Type::TwoPair,
            _ if !self.n_of_a_kind(2).is_empty() => Type::OnePair,
            _ => Type::HighCard,
        }
    }

    fn n_of_a_kind(&self, n: usize) -> Vec<&Card> {
        self.frequency()
            .iter()
            .filter(|(_, count)| count >= &n)
            .map(|(card, _)| *card)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_ordering() {
        assert_eq!(Card::A.cmp(&Card::Num(9)), std::cmp::Ordering::Greater);
        assert_eq!(
            Card::Num(10).cmp(&Card::Num(9)),
            std::cmp::Ordering::Greater
        );
    }
}
