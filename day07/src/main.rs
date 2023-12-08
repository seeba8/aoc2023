use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use color_eyre::{Report, Result};
use color_eyre::eyre::eyre;
use Card::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    color_eyre::install()?;
    let hands: Vec<Hand> = INPUT.trim().lines().map(Hand::from_str).collect::<Result<Vec<_>>>()?;
    println!("Day 07 part 1: {}", get_total_winnings(&hands));
    let hands: Vec<Hand> = INPUT.trim().lines().map(|line| Hand::from_str(line).map(Hand::j_is_joker)).collect::<Result<Vec<_>>>()?;
    println!("Day 07 part 2: {}", get_total_winnings(&hands));
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

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Ace => "A",
            King => "K",
            Queen => "Q",
            Jack => "J",
            Ten => "T",
            Nine => "9",
            Eight => "8",
            Seven => "7",
            Six => "6",
            Five => "5",
            Four => "4",
            Three => "3",
            Two => "2",
        })
    }
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
        s.set_hand_type(false);
        s.set_sort_order(false);
        s
    }

    fn j_is_joker(mut self) -> Self {
        self.set_hand_type(true);
        self.set_sort_order(true);
        self
    }

    fn set_sort_order(&mut self, j_is_joker: bool) {
        self.sort_order = u32::from((0b1111 - self.hand_type as u8) << 4);
        for card in &self.cards {
            self.sort_order <<= 4;
            if !j_is_joker || *card != Jack {
                self.sort_order |= u32::from(0b1111 - *card as u8);
            }
        }
    }

    fn set_hand_type(&mut self, j_is_joker: bool) {
        let mut buckets: HashMap<Card, u8> = HashMap::new();
        for card in self.cards {
            buckets.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }
        let num_jokers = if j_is_joker {
            buckets.remove(&Jack).unwrap_or(0)
        } else {
            0
        };
        self.hand_type = match buckets.len() {
            0 | 1 => HandType::FiveOfAKind,
            2 => {
                if buckets.iter().any(|(_, num)| *num + num_jokers == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if buckets.iter().any(|(_, num)| *num + num_jokers == 3) {
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

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in &self.cards {
            write!(f, "{c}")?;
        }
        Ok(())
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
    fn it_gets_total_winnings() -> Result<()> {
        let hands: Vec<Hand> = EXAMPLE.trim().lines().map(Hand::from_str).collect::<Result<Vec<_>>>()?;
        assert_eq!(get_total_winnings(&hands), 6440);
        Ok(())
    }

    #[test]
    fn it_gets_total_winnings_with_joker() -> Result<()> {
        let hands: Vec<Hand> = EXAMPLE.trim().lines().map(|l| Hand::from_str(l).map(Hand::j_is_joker)).collect::<Result<Vec<_>>>()?;
        assert_eq!(get_total_winnings(&hands), 5905);
        Ok(())
    }

    #[test]
    fn it_gets_ranking_with_joker() -> Result<()> {
        let mut hands: Vec<Hand> = EXAMPLE.trim().lines().map(|l| Hand::from_str(l).map(Hand::j_is_joker)).collect::<Result<Vec<_>>>()?;
        hands.sort_unstable();
        assert_eq!(hands[0].to_string(), "32T3K");
        assert_eq!(hands[1].to_string(), "KK677");
        assert_eq!(hands[2].to_string(), "T55J5");
        assert_eq!(hands[3].to_string(), "QQQJA");
        assert_eq!(hands[4].to_string(), "KTJJT");
        Ok(())
    }

    #[test]
    fn it_ranks_joker_correctly() {
        let a: Hand = "J2223 0".parse().unwrap();
        let a = a.j_is_joker();
        let b: Hand = "2KKKK 0".parse().unwrap();
        let b = b.j_is_joker();
        assert!(b > a);
    }
}