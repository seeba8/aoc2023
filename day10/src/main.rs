#![feature(let_chains)]

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::Report;


const INPUT: &str = include_str!("input.txt");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let grid: Grid = INPUT.parse()?;
    println!("Day 10 part 1: {}", grid.get_furthest_distance_on_loop().ok_or_else(|| eyre!("Cannot get loop"))?);
    println!("Day 10 part 2: {}", grid.inside_tiles().ok_or_else(|| eyre!("Cannot count tiles inside loop"))?);
    Ok(())
}

type Coordinate = (isize, isize);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Tile {
    Ground,
    Start,
    Pipe(Direction, Direction),
}

impl Tile {
    fn connects(self, direction: Direction) -> bool {
        match self {
            Self::Ground => false,
            Self::Start => true,
            Self::Pipe(a, b) => a == direction || b == direction
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Pipe(Direction::North, Direction::South)),
            '-' => Ok(Self::Pipe(Direction::West, Direction::East)),
            'L' => Ok(Self::Pipe(Direction::North, Direction::East)),
            'J' => Ok(Self::Pipe(Direction::North, Direction::West)),
            '7' => Ok(Self::Pipe(Direction::South, Direction::West)),
            'F' => Ok(Self::Pipe(Direction::South, Direction::East)),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(eyre!("Illegal tile: {value}"))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
            Self::Pipe(a, b) => {
                match (a, b) {
                    (Direction::North, Direction::South) => write!(f, "|"),
                    (Direction::West, Direction::East) => write!(f, "-"),
                    (Direction::North, Direction::East) => write!(f, "L"),
                    (Direction::North, Direction::West) => write!(f, "J"),
                    (Direction::South, Direction::West) => write!(f, "7"),
                    (Direction::South, Direction::East) => write!(f, "F"),
                    _ => write!(f, "?")
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    start: Coordinate,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<Tile>> = vec![];
        let mut start = (0isize, 0isize);
        for (y, line) in s.trim().lines().enumerate() {
            let mut row: Vec<Tile> = vec![];
            for (x, c) in line.trim().chars().enumerate() {
                let t: Tile = c.try_into()?;
                if t == Tile::Start {
                    start = (isize::try_from(x)?, isize::try_from(y)?);
                }
                row.push(t);
            }
            grid.push(row);
        }
        Ok(Self { grid, start })
    }
}

impl Grid {
    fn neighbours(&self, coordinate: &Coordinate) -> Option<[Coordinate; 2]> {
        let mut neighbours: Vec<Coordinate> = vec![];
        let current = self.get(coordinate)?;
        if current.connects(Direction::North) && self.get(&(coordinate.0, coordinate.1 - 1)).or(Some(Tile::Ground))?.connects(Direction::South) {
            neighbours.push((coordinate.0, coordinate.1 - 1));
        }
        if current.connects(Direction::East) && self.get(&(coordinate.0 + 1, coordinate.1)).or(Some(Tile::Ground))?.connects(Direction::West) {
            neighbours.push((coordinate.0 + 1, coordinate.1));
        }
        if current.connects(Direction::South) && self.get(&(coordinate.0, coordinate.1 + 1)).or(Some(Tile::Ground))?.connects(Direction::North) {
            neighbours.push((coordinate.0, coordinate.1 + 1));
        }
        if current.connects(Direction::West) && self.get(&(coordinate.0 - 1, coordinate.1)).or(Some(Tile::Ground))?.connects(Direction::East) {
            neighbours.push((coordinate.0 - 1, coordinate.1));
        }
        neighbours.try_into().ok()
    }
    #[inline]
    fn get(&self, coordinate: &Coordinate) -> Option<Tile> {
        Some(self.grid[usize::try_from(coordinate.1).ok()?][usize::try_from(coordinate.0).ok()?])
    }
    fn get_loop(&self) -> Option<Vec<Coordinate>> {
        let mut current = self.start;
        let mut loop_tiles = vec![];
        let mut previous = self.start;
        let mut first_loop = true;
        while first_loop || current != self.start {
            let neighbours = self.neighbours(&current)?;
            loop_tiles.push(current);
            if neighbours[0] == previous || first_loop {
                previous = current;
                current = neighbours[1];
            } else {
                previous = current;
                current = neighbours[0];
            };
            if first_loop { first_loop = false };
        }
        Some(loop_tiles)
    }

    fn get_furthest_distance_on_loop(&self) -> Option<usize> {
        self.get_loop().map(|l| l.len() / 2)
    }

    fn inside_tiles(&self) -> Option<usize> {
        let loop_tiles: HashSet<Coordinate> = HashSet::from_iter(self.get_loop()?);
        let height = isize::try_from(self.grid.len()).ok()?;
        let width = isize::try_from(self.grid[0].len()).ok()?;
        let mut count = 0;
        for y in 0..height {
            for x in 0..width {
                if loop_tiles.contains(&(x, y)) {
                    continue;
                }
                if self.is_point_in_path((x, y)).unwrap_or(false) {
                    count += 1;
                }
            }
        }
        Some(count)
    }

    /// Implementation of the Even-Odd rule.
    ///
    /// Source: <a href="https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule">Wikipedia</a>
    /// I don't understand it...
    fn is_point_in_path(&self, point: Coordinate) -> Option<bool> {
        let loop_tiles = self.get_loop()?;
        if loop_tiles.contains(&point) {
            // point is part of loop
            return Some(false);
        }
        let mut c = false;
        let mut j = loop_tiles.len() - 1;
        for i in 0..loop_tiles.len() {
            if (loop_tiles[i].1 > point.1) != (loop_tiles[j].1 > point.1) {
                let slope = (point.0 - loop_tiles[i].0) * (loop_tiles[j].1 - loop_tiles[i].1)
                    - (loop_tiles[j].0 - loop_tiles[i].0) * (point.1 - loop_tiles[i].1);
                // this seems to not happen in our scenario.
                // Wikipedia's description is:
                // > point is on boundary
                // I am not sure what that means.
                // if slope == 0 {
                //     return Some(false);
                // }
                if (slope < 0) != (loop_tiles[j].1 < loop_tiles[i].1) {
                    c = !c;
                }
            }
            j = i;
        }
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_and_displays() {
        let input = include_str!("example1.txt").replace("\r\n", "\n");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.to_string().trim(), input);
        let input = include_str!("example2.txt").replace("\r\n", "\n");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.to_string().trim(), input);
        let input = include_str!("example3.txt").replace("\r\n", "\n");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.to_string().trim(), input);
    }

    #[test]
    fn it_gets_simple_loop() {
        let input = include_str!("example2.txt");
        let grid: Grid = input.parse().unwrap();
        let mut l = grid.get_loop().unwrap();
        l.sort_unstable();
        let mut expected = [(1isize, 1isize), (1, 2), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (2, 1)];
        expected.sort_unstable();
        assert_eq!(l, expected);
    }

    #[test]
    fn it_gets_furthest_point() {
        let input = include_str!("example2.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.get_furthest_distance_on_loop(), Some(4));
        let input = include_str!("example3.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.get_furthest_distance_on_loop(), Some(4));
        let input = include_str!("example4.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.get_furthest_distance_on_loop(), Some(8));
    }

    #[test]
    fn it_counts_inside_tiles1() {
        let input = include_str!("example5.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.inside_tiles(), Some(4));
    }

    #[test]
    fn it_counts_inside_tiles2() {
        let input = include_str!("example6.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.inside_tiles(), Some(4));
    }

    #[test]
    fn it_counts_inside_tiles3() {
        let input = include_str!("example7.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.inside_tiles(), Some(8));
    }

    #[test]
    fn it_counts_inside_tiles4() {
        let input = include_str!("example8.txt");
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.inside_tiles(), Some(10));
    }
}