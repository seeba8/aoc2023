use std::str::FromStr;
use color_eyre::{Report, Result};
use color_eyre::eyre::eyre;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut sequences = get_sequences_from_input(INPUT)?;

    let s: isize = sequences
        .iter_mut()
        .map(std::iter::Iterator::next)
        .sum::<Option<_>>().ok_or_else(|| eyre!("Cannot get next value"))?;

    println!("Day 09 part 1: {s}");

    let s: isize = sequences
        .iter_mut()
        .map(std::iter::DoubleEndedIterator::next_back)
        .sum::<Option<_>>().ok_or_else(|| eyre!("Cannot get next back value"))?;
    println!("Day 09 part 2: {s}");

    Ok(())
}
fn get_sequences_from_input(input: &str) -> Result<Vec<Sequence>> {
    input.trim()
        .lines()
        .map(Sequence::from_str)
        .collect()
}

#[derive(Clone, Debug)]
struct Sequence {
    last: Vec<isize>,
    first: Vec<isize>,
}

impl Sequence {
    fn new(initial_data: &[isize]) -> Self {
        let mut last: Vec<isize> = vec![*initial_data.last().unwrap()];
        let mut first: Vec<isize> = vec![*initial_data.first().unwrap()];
        let mut numbers = initial_data.to_vec();
        while numbers.iter().any(|n| *n != 0) {
            let mut new_numbers: Vec<isize> = vec![];
            for i in 0..(numbers.len() - 1) {
                new_numbers.push(numbers[i + 1] - numbers[i]);
            }
            first.push(*new_numbers.first().unwrap());
            last.push(*new_numbers.last().unwrap());
            numbers = new_numbers;
        }
        last.reverse();
        first.reverse();
        Self { last , first}
    }
}

impl FromStr for Sequence {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iv: Vec<isize> = s
            .trim()
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(&iv))
    }
}

impl Iterator for Sequence {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 1..self.last.len() {
            self.last[i] += self.last[i - 1];
        }
        self.last.last().copied()
    }
}

impl DoubleEndedIterator for Sequence {
    fn next_back(&mut self) -> Option<Self::Item> {
        for i in 1..self.first.len() {
            self.first[i] -= self.first[i - 1];
        }
        self.first.last().copied()
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::eyre;
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn it_calculates_linear_sequence() {
        let mut seq = Sequence::new(&[0, 3, 6, 9]);
        assert_eq!(seq.take(3).collect::<Vec<_>>(), vec![12, 15, 18]);
    }

    #[test]
    fn it_calculates_higher_sequences() {
        let mut seq = Sequence::new(&[1, 3, 6, 10, 15, 21]);
        assert_eq!(seq.next(), Some(28));

        let mut seq = Sequence::new(&[10, 13, 16, 21, 30, 45]);
        assert_eq!(seq.next(), Some(68));
    }

    #[test]
    fn it_sums_iterators() -> Result<()> {
        let mut iterators: Vec<Sequence> = get_sequences_from_input(EXAMPLE)?;

        let s: isize = iterators
            .iter_mut()
            .map(std::iter::Iterator::next)
            .sum::<Option<_>>().ok_or_else(|| eyre!("Cannot get next value"))?;
        assert_eq!(s, 114);
        Ok(())
    }

    #[test]
    fn it_gets_other_end_of_sequence() -> Result<()> {
        let mut iterators: Vec<Sequence> = get_sequences_from_input(EXAMPLE)?;
        let next_backs: Vec<isize> = iterators
            .iter_mut()
            .map(std::iter::DoubleEndedIterator::next_back)
            .collect::<Option<Vec<_>>>().ok_or_else(|| eyre!("Cannot get last next_back"))?;
        assert_eq!(next_backs, vec![-3, 0, 5]);
        Ok(())
    }

    #[test]
    fn it_deals_with_negative_starts() {
        let mut sequence = Sequence::new(&[-6, -7, -8, -9, -10]);
        assert_eq!(sequence.next_back(), Some(-5));
    }
}