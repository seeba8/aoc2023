use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;
use color_eyre::Result;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    color_eyre::install()?;
    let space: Space = INPUT.parse()?;
    println!("Day 11 part 1: {}", space.get_sum_of_pairwise_distances(2));
    println!("Day 11 part 1: {}", space.get_sum_of_pairwise_distances(1_000_000));
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => { Ordering::Less }
            Ordering::Greater => { Ordering::Greater }
            Ordering::Equal => { self.x.cmp(&other.x) }
        }
    }
}

#[derive(Debug, Default)]
struct Space {
    galaxies: Vec<Point>,
    column_is_empty: Vec<bool>,
    row_is_empty: Vec<bool>,
}

impl Space {
    fn new(mut galaxies: Vec<Point>) -> Self {
        galaxies.sort_unstable();
        let x_es: HashSet<usize> = galaxies.iter().map(|galaxy| galaxy.x).collect();

        let get_unused_values = |values: &HashSet<usize>| -> Vec<bool> {
            let mut empty_columns = vec![];
            for x in *values.iter().min().unwrap()..=*values.iter().max().unwrap() {
                empty_columns.push(!values.contains(&x));
            }
            empty_columns
        };
        let empty_columns = get_unused_values(&x_es);

        let y_es: HashSet<usize> = galaxies.iter().map(|galaxy| galaxy.y).collect();
        let empty_rows = get_unused_values(&y_es);
        Self {
            galaxies,
            column_is_empty: empty_columns,
            row_is_empty: empty_rows,
        }
    }

    fn get_distance(&self, index_a: usize, index_b: usize, empty_factor: usize) -> usize {
        self.galaxies[index_a].manhattan_distance(&self.galaxies[index_b])
            + self.column_is_empty
            .iter()
            .enumerate()
            .filter(|(x, is_empty)| **is_empty && *x >= self.galaxies[index_a].x.min(self.galaxies[index_b].x) && *x <= self.galaxies[index_a].x.max(self.galaxies[index_b].x))
            .count() * (empty_factor - 1)
            + self.row_is_empty
            .iter()
            .enumerate()
            .filter(|(y, is_empty)| **is_empty && *y >= self.galaxies[index_a].y.min(self.galaxies[index_b].y) && *y <= self.galaxies[index_a].y.max(self.galaxies[index_b].y))
            .count() * (empty_factor - 1)
    }
    fn get_sum_of_pairwise_distances(&self, empty_factor: usize) -> usize {
        let mut s = 0;
        for a in 0..self.galaxies.len() {
            for b in (a + 1)..self.galaxies.len() {
                s += self.get_distance(a, b, empty_factor);
            }
        }
        s
    }
}

impl FromStr for Space {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies: Vec<Point> = vec![];
        for (y, row) in s.trim().lines().enumerate() {
            for (x, c) in row.trim().char_indices() {
                if c == '#' {
                    galaxies.push(Point { x, y });
                }
            }
        }
        Ok(Self::new(galaxies))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_parse_space() {
        let space: Space = EXAMPLE.parse().unwrap();
        assert_eq!(space.galaxies.len(), 9);
        assert!(space.galaxies.contains(&Point { x: 3, y: 0 }));
    }

    #[test]
    fn it_finds_empty_rows_and_columns() {
        let space: Space = EXAMPLE.parse().unwrap();
        assert!(space.row_is_empty[3]);
        assert!(!space.row_is_empty[2]);
        assert!(space.column_is_empty[2]);
        assert!(!space.column_is_empty[1]);
        assert_eq!(space.column_is_empty.iter().filter(|c| **c).count(), 3);
        assert_eq!(space.row_is_empty.iter().filter(|c| **c).count(), 2);
    }

    #[test]
    fn it_measures_distances() {
        let space: Space = EXAMPLE.parse().unwrap();
        assert_eq!(space.get_sum_of_pairwise_distances(2), 374);
    }

    #[test]
    fn it_measures_distances_with_factor() {
        let space: Space = EXAMPLE.parse().unwrap();
        assert_eq!(space.get_sum_of_pairwise_distances(10), 1030);
        assert_eq!(space.get_sum_of_pairwise_distances(100), 8410);
    }

    #[test]
    fn it_measures_distance() {
        let space: Space = EXAMPLE.parse().unwrap();
        let a = space.galaxies.iter().position(|p| *p == Point { x: 1, y: 5 }).unwrap();
        let b = space.galaxies.iter().position(|p| *p == Point { x: 4, y: 9 }).unwrap();
        assert_eq!(a, 4);
        assert_eq!(b, 8);
        dbg!(&space);
        assert_eq!(space.galaxies[a].manhattan_distance(&space.galaxies[b]), 7);
        assert_eq!(space.get_distance(a, b, 2), 9);
        assert_eq!(space.get_distance(0, 6, 2), 15);
        assert_eq!(space.get_distance(2, 5, 2), 17);
        assert_eq!(space.get_distance(7, 8, 2), 5);
    }
}