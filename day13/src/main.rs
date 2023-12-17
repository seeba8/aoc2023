#![feature(debug_closure_helpers)]
#![feature(let_chains)]

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::Report;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let mut mirrors = input.trim().split("\n\n").map(Mirror::from_str).collect::<Result<Vec<_>, _>>()?;
    let s = mirrors.iter().map(Mirror::reflection_value).sum::<Option<usize>>().ok_or_else(||eyre!("Cannot sum"))?;
    println!("Day 13 part 1: {s}");
    let mut s = 0;
    for mirror in &mut mirrors {
        let x = mirror.reflection_value_with_smudge().ok_or_else(|| eyre!("cannot find smudge in\n{mirror}"))?;
        s += x;
    }
    println!("Day 13 part 2: {s}");

    Ok(())
}

type Num = u32;

#[derive(Clone, Eq, PartialEq)]
struct Mirror {
    rows: Vec<Num>,
    columns: Vec<Num>,
}
#[allow(clippy::missing_fields_in_debug)]
impl Debug for Mirror {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        f.debug_struct("Mirror")
            .field_with("grid", |f| {
                writeln!(f)?;
                for row in &self.rows {
                    writeln!(f, "{row:b}")?;
                }
                Ok(())
            })
            .finish()
    }
}

impl Mirror {
    fn new(rows: Vec<Num>, columns: Vec<Num>) -> Self {
        Self {
            rows,
            columns,
        }
    }

    fn reflection_value_with_smudge(&mut self) -> Option<usize> {
        let normal_reflection = self.reflection_value().unwrap();
        for r in 0..self.rows.len() {
            for c in 0..self.columns.len() {
                self.columns[c] ^= 1 << r;
                self.rows[r] ^= 1 << c;
                let reflection_values = self.reflection_values();
                for v in reflection_values {
                    if v != normal_reflection {
                        return Some(v);
                    }
                }
                self.columns[c] ^= 1 << r;
                self.rows[r] ^= 1 << c;
            }
        }
        None
    }

    fn reflection_values(&self) -> Vec<usize> {
        let mut values = vec![];
        let (vertical, horizontal) = self.reflections();
        for v in vertical {
            values.push(v);
        }
        for h in horizontal {
            values.push(100*h);
        }
        values
    }
    fn reflection_value(&self) -> Option<usize> {
        match self.reflection() {
            (Some(vertical), None) => Some(vertical),
            (None, Some(horizontal)) => Some(horizontal * 100),
            _ => None
        }
    }

    fn reflection(&self) -> (Option<usize>, Option<usize>) {
        (Self::reflection_line(&self.columns), Self::reflection_line(&self.rows))
    }


    fn reflections(&self) -> (Vec<usize>, Vec<usize>) {
        (Self::reflection_lines(&self.columns), Self::reflection_lines(&self.rows))
    }

    fn reflection_line(grid: &[Num]) -> Option<usize> {
        Self::reflection_lines(grid).first().copied()
    }

    fn reflection_lines(grid: &[Num]) -> Vec<usize> {
        let mut reflection_lines = vec![];
        'outer: for i in 1..grid.len() {
            let w = i.min(grid.len() - i);
            for offset in 0..w {
                if grid[i - offset - 1] != grid[i + offset] {
                    continue 'outer;
                }
            }
            reflection_lines.push(i);
        }
        reflection_lines
    }
}

impl FromStr for Mirror {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or_else(|| eyre!("Cannot get first line"))?.len();
        let height= s.lines().count();
        let mut rows: Vec<Num> = Vec::with_capacity(height);
        let mut columns :Vec<Num> = vec![0; width];
        for (y, line) in s.lines().enumerate() {
            let mut row = 0;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        row |= 1;
                        columns[x] |= 1 << (height - y - 1);
                    }
                    '.' => {}
                    _ => { Err(eyre!("Illegal character: {c}"))?; }
                }
                row <<= 1;
            }
            row >>= 1;
            rows.push(row);
        }
        Ok(Self::new(rows, columns))
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for i in (0usize..self.columns.len()).rev() {
                if row & (1 << i) == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    #[test]
    fn it_parses_grid() {
        let mirror: Mirror = EXAMPLE1.parse().unwrap();
        dbg!(&mirror);
        assert_eq!(mirror.to_string(), EXAMPLE1);
    }

    #[test]
    fn it_finds_vertical_mirror_line() {
        let mirror: Mirror = EXAMPLE1.parse().unwrap();
        assert_eq!(Mirror::reflection_line(&mirror.columns), Some(5));
    }

    #[test]
    fn it_finds_horizontal_mirror_line() {
        let mirror: Mirror = EXAMPLE2.parse().unwrap();
        assert_eq!(Mirror::reflection_line(&mirror.rows), Some(4));
    }

    #[test]
    fn it_finds_reflections() {
        let mirror: Mirror = EXAMPLE1.parse().unwrap();
        assert_eq!(mirror.reflection(), (Some(5), None));
        let mirror: Mirror = EXAMPLE2.parse().unwrap();
        assert_eq!(mirror.reflection(), (None, Some(4)));
    }
    #[test]
    fn it_finds_reflection_value() {
        let mirror: Mirror = EXAMPLE1.parse().unwrap();
        let mirror2: Mirror = EXAMPLE2.parse().unwrap();
        assert_eq!(mirror.reflection_value().unwrap() + mirror2.reflection_value().unwrap(), 405);
    }

    #[test]
    fn it_finds_example3() {
        let mirror: Mirror = include_str!("example3.txt").parse().unwrap();
        let v = Mirror::reflection_line(&mirror.columns);
        assert_eq!(v, Some(1));
    }

    #[test]
    fn it_finds_reflection_with_smudge() {
        let mut mirror: Mirror = EXAMPLE1.parse().unwrap();
        assert_eq!(mirror.reflection_value_with_smudge(), Some(300));
        let mut mirror: Mirror = EXAMPLE2.parse().unwrap();
        assert_eq!(mirror.reflection_value_with_smudge(), Some(100));
    }
}