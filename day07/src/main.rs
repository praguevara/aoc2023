mod hand {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub enum Card {
        N(u8),
        T,
        J,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                '2'..='9' => Ok(Card::N(value as u8 - b'0')),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Type {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug)]
    pub struct Hand {
        cards: [Card; 5],
    }

    impl Hand {
        pub fn new(cards: [Card; 5]) -> Self {
            Self { cards }
        }

        fn hand_type(&self) -> Type {
            let mut card_counts: HashMap<Card, usize> = HashMap::new();
            for card in self.cards.iter() {
                *card_counts.entry(*card).or_default() += 1;
            }

            match card_counts.len() {
                1 => Type::FiveOfAKind,
                2 => {
                    if card_counts.values().any(|&count| count == 4) {
                        Type::FourOfAKind
                    } else {
                        Type::FullHouse
                    }
                }
                3 => {
                    if card_counts.values().any(|&count| count == 3) {
                        Type::ThreeOfAKind
                    } else {
                        Type::TwoPair
                    }
                }
                4 => Type::OnePair,
                5 => Type::HighCard,
                _ => unreachable!(),
            }
        }
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .all(|(a, b)| a == b)
        }
    }

    impl Eq for Hand {}

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.hand_type().cmp(&other.hand_type()) {
                std::cmp::Ordering::Equal => self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|&ordering| ordering != std::cmp::Ordering::Equal)
                    .unwrap_or(std::cmp::Ordering::Equal),
                ordering => ordering,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl TryFrom<&str> for Hand {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let cards = value
                .chars()
                .filter_map(|c| c.try_into().ok())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| ())?;
            Ok(Self::new(cards))
        }
    }
}

fn parse_input(input: &str) -> Vec<(hand::Hand, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap();
            let bet = parts.next().unwrap().parse().unwrap();
            (hand::Hand::try_from(hand).unwrap(), bet)
        })
        .collect()
}

fn total_winnings(mut hands_and_bets: Vec<(hand::Hand, i32)>) -> i32 {
    hands_and_bets.sort_by(|(h_l, _), (h_r, _)| h_l.cmp(h_r));

    let individual_winnings = hands_and_bets
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) as i32 * bet);

    individual_winnings.sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let hands_and_bets = parse_input(input);
    let winnings = total_winnings(hands_and_bets);
    println!("{}", winnings);
}

#[cfg(test)]
mod tests {
    use super::hand::*;
    use super::*;

    fn sample_input() -> &'static str {
        include_str!("../sample.txt")
    }

    #[test]
    fn test_ordering() {
        assert!(Hand::try_from("AAAAA").unwrap() > Hand::try_from("AA8AA").unwrap());
        assert!(Hand::try_from("AA8AA").unwrap() > Hand::try_from("23332").unwrap());
        assert!(Hand::try_from("23332").unwrap() > Hand::try_from("TTT98").unwrap());
        assert!(Hand::try_from("TTT98").unwrap() > Hand::try_from("23432").unwrap());
        assert!(Hand::try_from("23432").unwrap() > Hand::try_from("A23A4").unwrap());
        assert!(Hand::try_from("A23A4").unwrap() > Hand::try_from("23456").unwrap());

        assert!(Hand::try_from("33332").unwrap() > Hand::try_from("2AAAA").unwrap());
        assert!(Hand::try_from("77888").unwrap() > Hand::try_from("77788").unwrap());

        assert!(Hand::try_from("QQQJA").unwrap() > Hand::try_from("T55J5").unwrap());
    }

    #[test]
    fn test_parsing() {
        let input = sample_input();
        let parsed = parse_input(input);
    }

    #[test]
    fn test_total_winnings() {
        let input = sample_input();
        let hands_and_bets = parse_input(input);
        let winnings = total_winnings(hands_and_bets);
        assert_eq!(winnings, 6440);
    }
}
