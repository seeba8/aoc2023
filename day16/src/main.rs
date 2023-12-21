use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;
use color_eyre::{Result, Report, eyre::eyre};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;
    let contraption: Contraption = include_str!("input.txt").parse()?;
    println!("Day 16 part 1: {}", contraption.energise(Ray { x: 1, y: 1, direction: Direction::East }).iter().map(|r| (r.x, r.y)).unique().count());
    println!("Day 16 part 2: {}", contraption.best_energisation());
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Block {
    Empty,
    MirrorSlash,
    MirrorBackslash,
    SplitterVertical,
    SplitterHorizontal,
}

impl TryFrom<char> for Block {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            '/' => Ok(Self::MirrorSlash),
            '\\' => Ok(Self::MirrorBackslash),
            _ => Err(eyre!("Cannot parse block: {value}"))
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Ray {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Contraption {
    grid: Vec<Block>,
    width: usize,
    height: usize,
}

impl Contraption {
    fn get(&self, x: usize, y: usize) -> &Block {
        self.grid.get((y - 1) * self.width + (x - 1)).unwrap()
    }

    const fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x <= self.width && y <= self.height
    }

    fn best_energisation(&self) -> usize {
        let mut best = 0;
        for x in 1..=self.width {
            best = best.max(self.energise(Ray { x, y: 1, direction: Direction::South }).iter().map(|r| (r.x, r.y)).unique().count());
            best = best.max(self.energise(Ray { x, y: self.height, direction: Direction::North }).iter().map(|r| (r.x, r.y)).unique().count());
        }

        for y in 1..=self.height {
            best = best.max(self.energise(Ray { x: 1, y, direction: Direction::East }).iter().map(|r| (r.x, r.y)).unique().count());
            best = best.max(self.energise(Ray { x: self.width, y, direction: Direction::West }).iter().map(|r| (r.x, r.y)).unique().count());
        }
        best
    }
    fn energise(&self, ray: Ray) -> HashSet<Ray> {
        let mut existing_rays: HashSet<Ray> = HashSet::new();
        let mut rays = BTreeSet::new();
        rays.insert(ray);
        loop {
            let Some(mut ray) = rays.pop_first() else { return existing_rays; };
            while self.is_in_bounds(ray.x, ray.y) && !existing_rays.contains(&ray) {
                existing_rays.insert(ray);
                match self.get(ray.x, ray.y) {
                    Block::Empty => {}
                    Block::MirrorSlash => {
                        ray.direction = match ray.direction {
                            Direction::North => Direction::East,
                            Direction::South => Direction::West,
                            Direction::East => Direction::North,
                            Direction::West => Direction::South,
                        }
                    }
                    Block::MirrorBackslash => {
                        ray.direction = match ray.direction {
                            Direction::North => Direction::West,
                            Direction::South => Direction::East,
                            Direction::East => Direction::South,
                            Direction::West => Direction::North,
                        }
                    }
                    Block::SplitterVertical => {
                        match ray.direction {
                            Direction::North | Direction::South => {}
                            Direction::East | Direction::West => {
                                ray.direction = Direction::South;
                                let new_ray = Ray { x: ray.x, y: ray.y, direction: Direction::North };
                                rays.insert(new_ray);
                            }
                        }
                    }
                    Block::SplitterHorizontal => {
                        match ray.direction {
                            Direction::East | Direction::West => {}
                            Direction::North | Direction::South => {
                                ray.direction = Direction::East;
                                let new_ray = Ray { x: ray.x, y: ray.y, direction: Direction::West };
                                rays.insert(new_ray);
                            }
                        }
                    }
                }
                match ray.direction {
                    Direction::North => ray.y -= 1,
                    Direction::South => ray.y += 1,
                    Direction::East => ray.x += 1,
                    Direction::West => ray.x -= 1,
                };
            }
        }
    }
}

impl FromStr for Contraption {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().ok_or_else(|| eyre!("Cannot get first line"))?.len();
        let grid: Vec<Block> = s.chars().filter_map(|c| if c.is_whitespace() { None } else { Block::try_from(c).ok() }).collect();
        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");

    #[test]
    fn it_parses_map() {
        let contraption: Contraption = EXAMPLE1.parse().unwrap();
        assert_eq!(contraption.height, 10);
        assert_eq!(contraption.width, 10);
        assert_eq!(contraption.get(2, 1), &Block::SplitterVertical);
    }

    #[test]
    fn it_gets_energisation() {
        let contraption: Contraption = EXAMPLE1.parse().unwrap();
        assert_eq!(contraption.energise(Ray { direction: Direction::East, x: 1, y: 1 }).iter().map(|r| (r.x, r.y)).unique().count(), 46);
    }

    #[test]
    fn it_gets_best_energisation() {
        let contraption: Contraption = EXAMPLE1.parse().unwrap();
        assert_eq!(contraption.best_energisation(), 51);
    }
}