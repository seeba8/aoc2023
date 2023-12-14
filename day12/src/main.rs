use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::{install, Report};

fn main() -> color_eyre::Result<()> {
    install()?;
    let input = include_str!("input.txt");
    let rows: Vec<Row> = input.trim().lines().map(Row::from_str).collect::<Result<Vec<_>, _>>()?;
    let mut s = 0;
    for r in rows {
        s += r.count_matches();
    }
    println!("Day 12 part 1: {s}");
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    springs: Springs,
    groups: Vec<u8>,
}

impl Row {
    fn matches(&self, mut value: u32) -> bool {
        let total_broken: u8 = self.groups.iter().sum();
        if value.count_ones() != u32::from(total_broken) {
            return false;
        }
        if (!self.springs.unknown & self.springs.damaged) == (value & !self.springs.unknown) {
            //println!("{value:b}");
            let mut groups = self.groups.iter().rev();
            let mut n = *groups.next().unwrap();
            while value > 0 {
                if value.trailing_ones() == u32::from(n) {
                    value >>= n;
                    n = match groups.next() {
                        Some(x) => *x,
                        None => return true,
                    };
                }
                value >>= 1;
            }
        }
        false
    }

    fn count_matches(&self) -> usize {
        let mut c = 0;
        for i in 0..(1 << self.springs.length) {
            if self.matches(i) {
                c += 1;
            }
        }
        c
    }
}

impl FromStr for Row {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.trim().split_once(' ').ok_or_else(|| eyre!("Cannot split line by space"))?;
        let groups: Vec<u8> = groups.trim().split(',').map(str::parse).collect::<Result<Vec<_>, _>>()?;
        let springs: Springs = springs.parse()?;
        Ok(Self {
            springs,
            groups,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Springs {
    damaged: u32,
    unknown: u32,
    length: u8,
}

impl Debug for Springs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}")?;
        f.debug_struct("Springs")
            .field("damaged", &self.damaged)
            .field("unknown", &self.unknown)
            .field("length", &self.length)
            .finish()
    }
}

impl FromStr for Springs {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut damaged = 0;
        let mut unknown = 0;
        let mut length = 0;
        for c in s.trim().chars() {
            length += 1;
            match c {
                '?' => {
                    unknown |= 1;
                }
                '#' => {
                    damaged |= 1;
                }
                '.' => {}
                _ => { Err(eyre!("Illegal character: {c}"))?; }
            }
            damaged <<= 1;
            unknown <<= 1;
        }
        damaged >>= 1;
        unknown >>= 1;
        Ok(Self { damaged, unknown, length })
    }
}

impl Display for Springs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in (0usize..self.length as usize).rev() {
            if self.unknown & (1 << i) > 0 {
                write!(f, "?")?;
            } else if self.damaged & (1 << i) == 0 {
                write!(f, ".")?;
            } else {
                write!(f, "#")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_displays_known_springs() {
        let springs = Springs {
            length: 7,
            damaged: 0b101_0111,
            unknown: 0,
        };
        assert_eq!("#.#.###", springs.to_string());
    }

    #[test]
    fn it_displays_unknown_springs() {
        let springs = Springs {
            length: 7,
            damaged: 0b101_0111,
            unknown: 0b0111_0000,
        };
        assert_eq!("???.###", springs.to_string());
    }

    #[test]
    fn it_parses_springs() {
        assert_eq!(Springs::from_str("???.###").unwrap(), Springs {
            length: 7,
            damaged: 0b000_0111,
            unknown: 0b0111_0000,
        });
    }

    #[test]
    fn it_matches() {
        let r: Row = "???.### 1,1,3".parse().unwrap();
        assert_eq!(r.count_matches(), 1);

        let r: Row = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(r.count_matches(), 4);

        let r: Row = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(r.count_matches(), 10);
    }
}