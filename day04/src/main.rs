use std::str::FromStr;
use nom::{bytes::complete::{tag, tag_no_case}, character::complete::{space1, digit1}, combinator::{all_consuming, map}, Finish, IResult, multi::separated_list1, sequence::{separated_pair, tuple}};
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::preceded;

fn main() {
    let input = include_str!("input.txt");
    let cards: Vec<Card> = input.trim().lines().map(Card::from_str).collect::<Result<Vec<_>, _>>().unwrap();
    let sum: usize = cards.iter().map(Card::get_points).sum();
    println!("Day 04 part 1: {sum}");
    println!("Day 04 part 2: {}", count_total_cards(&cards));
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn get_points(&self) -> usize {
        let matches = u32::try_from(self.count_wins()).unwrap();
        if matches == 0 {
            0
        } else {
            2usize.pow(matches - 1)
        }
    }
    fn count_wins(&self) -> usize {
        self.winning_numbers.iter().filter(|v| self.numbers.contains(*v)).count()
    }
}

fn count_total_cards(cards: &[Card]) -> usize {
    let mut amounts = vec![1usize; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let amount_of_current_card = amounts[index];
        let wins = card.count_wins();
        for amount in amounts.iter_mut().skip(index+1).take(wins) {
            *amount += amount_of_current_card;
        }
    }
    amounts.iter().sum()
}


fn parse_card(input: &str) -> IResult<&str, Card> {
    map(all_consuming(preceded(
        tuple((tag_no_case("Card"), space1, digit1::<_, Error<_>>, tag_no_case(":"), space1)),
        separated_pair(
            separated_list1(space1, map_res(digit1, str::parse::<u8>)),
            tuple((space1, tag("|"), space1)),
            separated_list1(space1, map_res(digit1, str::parse::<u8>)),
        ),
    )), |v| Card { numbers: v.1, winning_numbers: v.0 })(input)
}

impl FromStr for Card {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_card(s).finish() {
            Ok((_, c)) => Ok(c),
            Err(Error { input, code }) => Err(color_eyre::eyre::eyre!("Cannot parse input {input}: {code:#?}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = Card {
            winning_numbers: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let actual: Card = input.parse().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_gets_points() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual: Card = input.parse().unwrap();
        assert_eq!(8, actual.get_points());
    }

    #[test]
    fn it_gets_all_points() {
        let input = include_str!("example.txt");
        let cards: Vec<Card> = input.trim().lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(cards.iter().map(Card::get_points).sum::<usize>(), 13usize);
    }

    #[test]
    fn it_counts_total_cards() {
        let input = include_str!("example.txt");
        let cards: Vec<Card> = input.trim().lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(count_total_cards(&cards), 30);
    }
}
