use std::cmp::Ordering;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::{separated_list1},
    sequence::{separated_pair, tuple},
    Finish, IResult,
    character::complete::digit1,
};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map_res;
use nom::sequence::{delimited};

const INPUT: &str = include_str!("input.txt");
const PART1_BAG: Draw = Draw {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn parse(input: &str) -> Self {
        all_consuming(
            map(tuple((
                delimited(tag_no_case("Game "), map_res(digit1, str::parse), tag(": ")),
                separated_list1(tag("; "), Draw::parse))
            ), |(id, draws)| Self { id, draws: draws.into_iter().map(Draw::from).collect() })
        )(input).finish().unwrap().1
    }

    fn get_power_of_minimum_set(&self) -> usize {
        self.draws.iter().fold(Draw::default(), |acc, draw| Draw {
            red: acc.red.max(draw.red),
            green: acc.green.max(draw.green),
            blue: acc.blue.max(draw.blue),
        }).power()
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}


impl Draw {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(
            tag(", "),
            separated_pair(map_res(digit1, str::parse),
                           multispace0,
                           alpha1)), |x: Vec<(usize, &str)>| {
            let mut s = Self::default();
            for (amount, colour) in x {
                match colour {
                    "red" => s.red += amount,
                    "green" => s.green += amount,
                    "blue" => s.blue += amount,
                    _ => ()
                }
            }
            s
        })(input)
    }

    const fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.red > other.red || self.green > other.green || self.blue > other.blue {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

fn main() {
    let games: Vec<Game> = INPUT.lines().map(Game::parse).collect();

    let sum: usize = games
        .iter()
        .filter(|g| g.draws.iter().all(|draw| draw < &PART1_BAG))
        .map(|game| game.id)
        .sum();
    println!("Day 02 part 1: {sum}");
    println!("Day 02 part 2: {}", games.iter().map(Game::get_power_of_minimum_set).sum::<usize>());
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example.txt");

    #[test]
    fn it_parses_draw() {
        let expected = Draw {
            red: 2,
            green: 12,
            blue: 1,
        };
        assert_eq!(expected, Draw::parse("1 blue, 12 green, 2 red").unwrap().1);
    }

    #[test]
    fn it_parses_game() {
        let game = Game {
            id: 5,
            draws: vec![Draw {
                red: 6,
                green: 3,
                blue: 1,
            }, Draw {
                red: 1,
                green: 2,
                blue: 2,
            }],
        };
        assert_eq!(game, Game::parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"));
    }


    #[test]
    fn it_solves_example_part1() {
        let games: Vec<Game> = EXAMPLE1.lines().map(Game::parse).collect();

        let sum: usize = games
            .iter()
            .filter(|g| g.draws.iter().all(|draw| draw < &PART1_BAG))
            .map(|game| game.id)
            .sum();
        assert_eq!(8, sum);
    }

    #[test]
    fn it_solves_example_part2() {
        let games: Vec<Game> = EXAMPLE1.lines().map(Game::parse).collect();
        assert_eq!(2286_usize, games.iter().map(Game::get_power_of_minimum_set).sum());
    }
}