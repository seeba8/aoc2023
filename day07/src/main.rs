use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Shl;
use std::str::FromStr;
use color_eyre::{Report, Result};
use color_eyre::eyre::eyre;
use Card::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};

const INPUT: &str = include_str!("input.txt");
fn main() -> Result<()> {
    color_eyre::install()?;
    let hands: Vec<Hand> = INPUT.trim().lines().map(Hand::from_str).collect::<Result<Vec<_>>>()?;
    println!("Day 07 part 1: {}", get_total_winnings(&hands));
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            v => Err(color_eyre::eyre::eyre!("Cannot parse card: {v}")),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    #[allow(clippy::struct_field_names)]
    hand_type: HandType,
    sort_order: u32,
    bid: usize,
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let mut s = Self {
            cards,
            hand_type: HandType::FiveOfAKind,
            sort_order: 0,
            bid,
        };
        s.set_hand_type();
        s.set_sort_order();
        s
    }

    fn set_sort_order(&mut self) {
        let mut first_byte = u32::from((0b1111 - self.hand_type as u8) << 4);
        first_byte |= u32::from(0b1111 - self.cards[0] as u8);
        let second_byte = u32::from(((0b1111 - self.cards[1] as u8) << 4) | (0b1111 - self.cards[2] as u8));
        let third_byte = u32::from(((0b1111 - self.cards[3] as u8) << 4) | (0b1111 - self.cards[4] as u8));
        self.sort_order = first_byte.shl(24) | second_byte.shl(16) | third_byte.shl(8);
    }
    fn set_hand_type(&mut self) {
        let mut buckets: HashMap<Card, u8> = HashMap::new();
        for card in self.cards {
            buckets.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }
        self.hand_type = match buckets.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if buckets.iter().any(|(_, num)| *num == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if buckets.iter().any(|(_, num)| *num == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.sort_order.cmp(&other.sort_order))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order.cmp(&other.sort_order)
    }
}

impl FromStr for Hand {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').ok_or_else(|| eyre!("Cannot parse line: {s}"))?;
        if cards.len() != 5 {
            return Err(eyre!("{cards}.len() != 5"));
        }
        Ok(Self::new(
            cards
                .chars()
                .map(Card::try_from)
                .collect::<Result<Vec<Card>>>()?
                .try_into()
                .map_err(|v| eyre!("cannot construct cards array from {v:#?}"))?,
            bid.parse()?))
    }
}

fn get_total_winnings(hands: &[Hand]) -> usize {
    let mut hands: Vec<Hand> = hands.to_vec();
    hands.sort_unstable();
    hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn it_compares_hands() {
        let a = Hand::new([Three, Three, Three, Three, Two], 0);
        let b = Hand::new([Two, Ace, Ace, Ace, Ace], 0);
        assert!(a > b);
        let a = Hand::new([Seven, Seven, Eight, Eight, Eight], 0);
        let b = Hand::new([Seven, Seven, Seven, Eight, Eight], 0);
        assert!(a > b);
    }

    #[test]
    fn it_parses_hand() {
        let input = "32T3K 765";
        let hand: Hand = input.parse().unwrap();
        assert_eq!(hand.bid, 765);
        assert_eq!(hand.hand_type, HandType::OnePair);
    }

    #[test]
    fn it_gets_total_winnings() -> Result<()>{
        let hands: Vec<Hand> = EXAMPLE.trim().lines().map(Hand::from_str).collect::<Result<Vec<_>>>()?;
        assert_eq!(get_total_winnings(&hands), 6440);
        Ok(())
    }
}