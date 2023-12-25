mod polygon;

use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::str::FromStr;
use color_eyre::eyre::eyre;
use owo_colors::{DynColors, Stream};
use owo_colors::OwoColorize;
use color_eyre::Report;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::polygon::{Point, Polygon};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let digplan: DigPlan = input.parse().unwrap();
    let dig_area = digplan.dig_area();
    println!("Day 18 part 1: {}", dig_area.volume());
    let polygon: Polygon = Polygon::new(&parse_part2(input));
    println!("Day 18 part 2: {}", polygon.area_with_1_wide_edge());
    Ok(())
}

#[allow(clippy::cast_possible_wrap)]
fn parse_part2(s: &str) -> Vec<Point> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[RLDU] \d+ \(#([0-9a-f]{5})([0-9a-f])\)").unwrap());
    let mut digs = vec![];
    for line in s.lines() {
        let cap = RE.captures(line).unwrap();
        digs.push(Trench {
            distance: usize::from_str_radix(&cap[1], 16).unwrap(),
            direction: Direction::from_str(&cap[2]).unwrap(),
            colour: owo_colors::DynColors::Rgb(0xff, 0, 0),
        });
    }
    let mut p = Point::new(0, 0);
    let mut corners: Vec<Point> = vec![p];
    for trench in digs {
        let offset = match trench.direction {
            Direction::Up => (0isize, -1isize),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        p.x += offset.0 * trench.distance as isize;
        p.y += offset.1 * trench.distance as isize;
        corners.push(p);
    }
    corners
}

/// Implementation of the Even-Odd rule, again.
///
/// Source this time: <a href="https://de.wikipedia.org/wiki/Punkt-in-Polygon-Test_nach_Jordan">Wikipedia</a>
///
/// It fixes some edge cases with horizontal and vertical lines, and when the ray exactly cuts a corner.
///
/// I  still don't understand it...
fn point_in_polygon(x: usize, y: usize, corners: &[(usize, usize)]) -> bool {
    let cross_prod_test = |x_a: usize, y_a: usize, (mut x_b, mut y_b): (usize, usize), (mut x_c, mut y_c): (usize, usize)| -> isize {
        if y_a == y_b && y_b == y_c {
            if x_b <= x_a && x_a <= x_c || x_c <= x_a && x_a <= x_b {
                return 0;
            }
            return 1;
        }
        if y_a == y_b && x_a == x_b {
            return 0;
        }
        if y_b > y_c {
            swap(&mut x_b, &mut x_c);
            swap(&mut y_b, &mut y_c);
        };

        if y_a <= y_b || y_a > y_c {
            return 1;
        }
        #[allow(clippy::cast_possible_wrap)]
            let delta: isize = (x_b as isize - x_a as isize) * (y_c as isize - y_a as isize) - (y_b as isize - y_a as isize) * (x_c as isize - x_a as isize);
        match delta {
            x if x > 0 => {
                -1
            }
            x if x < 0 => {
                1
            }
            _ => 0,
        }
    };
    let mut t = -1;
    for i in 0..corners.len() {
        if i == 0 {
            t *= cross_prod_test(x, y, corners[corners.len() - 1], corners[i]);
        } else {
            t *= cross_prod_test(x, y, corners[i - 1], corners[i]);
        }
        if t == 0 {
            break;
        }
    }
    t >= 0
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Up => "U",
            Self::Down => "D",
            Self::Left => "L",
            Self::Right => "R",
        })
    }
}

impl FromStr for Direction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "3" => Ok(Self::Up),
            "R" | "0" => Ok(Self::Right),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            _ => Err(eyre!("Cannot parse direction {s}")),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Trench {
    distance: usize,
    direction: Direction,
    colour: DynColors,
}

impl Display for Trench {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.direction.color(self.colour), self.distance.color(self.colour), self.colour.color(self.colour))
    }
}

impl FromStr for Trench {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([RLDU]) (\d+) \((#[0-9a-f]{6})\)").unwrap());
        let cap = RE.captures(s).ok_or_else(|| eyre!("Regex does not match {s}"))?;
        Ok(Self {
            distance: cap[2].parse()?,
            direction: cap[1].parse()?,
            colour: cap[3].parse().map_err(|e| eyre!("Cannot parse colour: {e:?}"))?,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
struct DigPlan {
    trenches: Vec<Trench>,
}

impl Display for DigPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for trench in &self.trenches {
            writeln!(f, "{trench}")?;
        }
        Ok(())
    }
}

impl FromStr for DigPlan {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            trenches: s
                .lines()
                .map(Trench::from_str)
                .collect::<Result<Vec<Trench>, _>>()?
        })
    }
}

#[derive(Debug, PartialEq)]
struct DigArea {
    width: usize,
    height: usize,
    digs: Vec<Option<usize>>,
    dig_plan: DigPlan,
    corners: Vec<(usize, usize)>,
}

impl DigPlan {
    fn dig_area(self) -> DigArea {
        let mut position = (0isize, 0isize);
        let mut dig_places: Vec<(isize, isize, usize)> = vec![(position.0, position.1, 0)];
        let mut corners: Vec<(isize, isize)> = vec![];
        for (idx, trench) in self.trenches.iter().enumerate() {
            corners.push(position);
            let offset = match trench.direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            for _ in 0..trench.distance {
                position.0 += offset.0;
                position.1 += offset.1;
                dig_places.push((position.0, position.1, idx));
            }
        }
        let min_x = dig_places.iter().min_by_key(|v| v.0).unwrap().0;
        let max_x = dig_places.iter().max_by_key(|v| v.0).unwrap().0;
        let x_range = max_x.abs_diff(min_x) + 1;
        let min_y = dig_places.iter().min_by_key(|v| v.1).unwrap().1;
        let max_y = dig_places.iter().max_by_key(|v| v.1).unwrap().1;
        let y_range = max_y.abs_diff(min_y) + 1;
        let mut digs: Vec<Option<usize>> = vec![None; x_range * y_range];
        for (x, y, trench_index) in dig_places {
            digs[y.abs_diff(min_y) * x_range + x.abs_diff(min_x)] = Some(trench_index);
        }
        DigArea {
            width: x_range,
            height: y_range,
            digs,
            dig_plan: self,
            corners: corners.into_iter().map(|(x, y)| (x.abs_diff(min_x), y.abs_diff(min_y))).collect(),
        }
    }
}

impl Display for DigArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, t) in self.digs.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            match t {
                None => write!(f, "."),
                Some(trench_index) => write!(f, "{}", "#".if_supports_color(Stream::Stdout, |t| t.color(self.dig_plan.trenches[*trench_index].colour))),
            }?;
        }
        Ok(())
    }
}

impl DigArea {
    fn volume(&self) -> usize {
        let mut v = 0;
        //let edges = self.edges();
        for y in 0..self.height {
            for x in 0..self.width {
                if point_in_polygon(x, y, &self.corners) {
                    v += 1;
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::polygon::{Point, Polygon};
    use super::*;

    const EXAMPLE1: &str = include_str!("example.txt");

    #[test]
    fn it_parses_line() {
        color_eyre::install().unwrap();
        let input = "R 6 (#70c710)";
        let trench: Trench = input.parse().unwrap();
        assert_eq!(trench, Trench {
            distance: 6,
            direction: Direction::Right,
            colour: DynColors::Rgb(0x70, 0xc7, 0x10),
        });
    }

    #[test]
    fn it_parses_digplan() {
        let digplan: DigPlan = EXAMPLE1.parse().unwrap();
        println!("{digplan}");
        assert_eq!(digplan.trenches.len(), 14);
    }

    #[test]
    fn it_gets_dig_area() {
        let digplan: DigPlan = EXAMPLE1.parse().unwrap();
        let dig_area = digplan.dig_area();
        let expected = r"#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######";
        owo_colors::with_override(false, || {
            let actual = dig_area.to_string();
            assert_eq!(actual, expected);
        });
        owo_colors::set_override(true);
        println!("{dig_area}");
    }

    #[test]
    fn it_gets_volume() {
        let digplan: DigPlan = EXAMPLE1.parse().unwrap();
        let dig_area = digplan.dig_area();
        assert_eq!(dig_area.volume(), 62);
    }

    #[test]
    fn it_tests_polygon() {
        let digplan: DigPlan = EXAMPLE1.parse().unwrap();
        let dig_area = digplan.dig_area();
        let corners: Vec<Point> = dig_area.corners.iter().map(|(x, y)| polygon::Point { x: *x as isize, y: *y as isize }).collect();
        let polygon = Polygon::new(&corners);
        assert_eq!(polygon.area_with_1_wide_edge(), 62);
    }

    #[test]
    fn it_solves_part2() {
        let polygon: Polygon = Polygon::new(&parse_part2(EXAMPLE1));
        assert_eq!(polygon.area_with_1_wide_edge(), 952_408_144_115);
    }
}