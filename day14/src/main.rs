#![feature(let_chains)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::Report;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut platform: Platform = include_str!("input.txt").parse()?;
    platform.tilt_north();
    println!("Day 14 part 1: {}", platform.get_load());
    let mut platform: Platform = include_str!("input.txt").parse()?;
    platform.run_cycles(1_000_000_000);
    println!("Day 14 part 1: {}", platform.get_load());
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Block {
    Round,
    Cube,
    Empty,
}

impl TryFrom<char> for Block {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Round),
            '#' => Ok(Self::Cube),
            '.' => Ok(Self::Empty),
            _ => Err(eyre!("Not a block: {value}")),
        }
    }
}

#[derive(Clone, Default, Debug)]
struct Platform {
    grid: Vec<Block>,
    width: isize,
    height: isize,
}

impl Platform {
    #[inline]
    const fn c2i(&self, x: isize, y: isize) -> usize {
        (y * self.width + x) as usize
    }

    #[inline]
    fn get(&self, x: isize, y: isize) -> Block {
        if x < 0 || y < 0 {
            Block::Cube
        } else {
            self.grid[self.c2i(x, y)]
        }
    }

    fn tilt_north(&mut self) {
        for y in 1..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Block::Round {
                    self.move_rock(x, y, 0, -1);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..(self.height - 1)).rev() {
            for x in 0..self.width {
                if self.get(x, y) == Block::Round {
                    self.move_rock(x, y, 0, 1);
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for x in (0..(self.width - 1)).rev() {
            for y in 0..self.height {
                if self.get(x, y) == Block::Round {
                    self.move_rock(x, y, 1, 0);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for x in 1..self.width {
            for y in 0..self.height {
                if self.get(x, y) == Block::Round {
                    self.move_rock(x, y, -1, 0);
                }
            }
        }
    }

    fn set(&mut self, x: isize, y: isize, block: Block) {
        let i = self.c2i(x, y);
        self.grid[i] = block;
    }

    fn move_rock(&mut self, x: isize, y: isize, x_offset: isize, y_offset: isize) {
        let mut new_y = y + y_offset;
        let mut new_x = x + x_offset;
        while new_y >= 0 && new_x >= 0 && new_y < self.height && new_x < self.width && self.get(new_x, new_y) == Block::Empty {
            new_y += y_offset;
            new_x += x_offset;
        }
        new_y -= y_offset;
        new_x -= x_offset;

        self.set(x, y, Block::Empty);
        self.set(new_x, new_y, Block::Round);
    }

    fn get_load(&self) -> isize {
        self.grid.iter().enumerate().filter_map(|(i, b)| {
            if *b == Block::Round {
                Some(self.height - (i as isize / self.width))
            } else {
                None
            }
        }).sum()
    }

    fn run_cycles(&mut self, cycles: usize) {
        let mut visited: HashMap<Vec<Block>, usize> = HashMap::new();
        let mut i = 1;
        let mut cycle_found = false;
        while i <= cycles {
            self.next();
            if let Some(last_seen) = visited.insert(self.grid.clone(), i) && !cycle_found {
                let cycle_length = i - last_seen;
                while i + cycle_length < cycles {
                    i += cycle_length;
                }
                cycle_found = true;
            }
            i += 1;
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Block::Round => write!(f, "O")?,
                    Block::Cube => write!(f, "#")?,
                    Block::Empty => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Iterator for Platform {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
        Some(())
    }
}

impl FromStr for Platform {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = isize::try_from(s.lines().count())?;
        let width = isize::try_from(s.lines().next().ok_or_else(|| eyre!("No lines in input"))?.len())?;
        let grid: Vec<Block> = s.chars().filter_map(|c| {
            if c.is_whitespace() {
                None
            } else {
                Block::try_from(c).ok()
            }
        }).collect();
        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Platform;

    const EXAMPLE1: &str = include_str!("example.txt");

    #[test]
    fn it_parses_and_displays() {
        let platform: Platform = EXAMPLE1.parse().unwrap();
        assert_eq!(platform.to_string(), EXAMPLE1);
    }

    #[test]
    fn it_tilts_north() {
        let mut platform: Platform = EXAMPLE1.parse().unwrap();
        platform.tilt_north();
        assert_eq!(platform.to_string(), include_str!("example1_expected.txt"));
    }

    #[test]
    fn it_gets_load() {
        let mut platform: Platform = EXAMPLE1.parse().unwrap();
        platform.tilt_north();
        assert_eq!(platform.get_load(), 136);
    }

    #[test]
    fn it_cycles() {
        let mut platform: Platform = EXAMPLE1.parse().unwrap();
        platform.next();
        assert_eq!(platform.to_string(), include_str!("example1_cycled1.txt"));
    }

    #[test]
    fn it_cycles_a_lot() {
        let mut platform: Platform = EXAMPLE1.parse().unwrap();
        platform.run_cycles(1_000_000_000);
        assert_eq!(platform.get_load(), 64);
    }
}