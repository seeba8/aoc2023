use std::num::ParseIntError;
use std::str::FromStr;
use color_eyre::{Result};
use color_eyre::eyre::{eyre, WrapErr};
const INPUT: &str = include_str!("input.txt");
fn main() -> Result<()> {
    color_eyre::install()?;
    let races = parse(INPUT)?;
    let margin: usize = races.iter().map(Race::get_number_of_winning_options).product();
    println!("Day 06 part 1: {margin}");
    let race: Race = INPUT.parse()?;
    println!("Day 06 part 2: {}", race.get_number_of_winning_options());
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl FromStr for Race {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<usize> = s
            .trim()
            .lines()
            .map(|line| line
                .split_once(':')
                .ok_or_else(|| eyre!("Cannot parse time")).and_then(|(_, b)| b.replace(' ', "").parse().map_err(|e: ParseIntError| eyre!(e))))
                .collect::<Result<Vec<usize>>>()?;
        Ok(Self {time: lines[0], record_distance: lines[1]})
    }
}

impl Race {
    #[inline]
    const fn get_distance(&self, button_duration: usize) -> usize {
        (self.time - button_duration) * button_duration
    }

    #[inline]
    const fn wins(&self, button_duration: usize) -> bool {
        self.get_distance(button_duration) > self.record_distance
    }

    fn get_number_of_winning_options(&self) -> usize {
        (1..self.time).filter(|v| self.wins(*v)).count()
    }
}

fn parse(input: &str) -> Result<Vec<Race>> {
    let lines: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| line
            .trim()
            .split_ascii_whitespace()
            .skip(1)
            .map(|v| v.parse().wrap_err("Cannot parse number"))
            .collect::<Result<Vec<usize>>>())
        .collect::<Result<Vec<_>>>()?;
    let mut races: Vec<Race> = Vec::with_capacity(lines[0].len());
    for i in 0..lines[0].len() {
        races.push(Race{ time: lines[0][i], record_distance: lines[1][i] });
    }
    Ok(races)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn it_parses_input() {
        let races = parse(EXAMPLE).unwrap();
        assert_eq!(races, vec![Race{time: 7, record_distance: 9}, Race{time: 15, record_distance: 40}, Race{time: 30, record_distance: 200}]);
    }
    #[test]
    fn it_calculates_wins() {
        let races = parse(EXAMPLE).unwrap();
        assert_eq!(races[0].get_number_of_winning_options(), 4);
        assert_eq!(races[1].get_number_of_winning_options(), 8);
        assert_eq!(races[2].get_number_of_winning_options(), 9);
    }

    #[test]
    fn it_parses_part2() {
        let race: Race = EXAMPLE.parse().unwrap();
        assert_eq!(race, Race{time: 71_530, record_distance: 940_200});
    }
    #[test]
    fn it_solves_example_part2() {
        let race: Race = EXAMPLE.parse().unwrap();
        assert_eq!(race.get_number_of_winning_options(), 71_503);
    }

}