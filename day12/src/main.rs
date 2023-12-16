use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::{install, Report};

fn main() -> color_eyre::Result<()> {
    install()?;
    let input = include_str!("input.txt");
    let mut rows: Vec<Row> = input.trim().lines().map(Row::from_str).collect::<Result<Vec<_>, _>>()?;
    let mut s = 0;
    for r in &rows {
        r.count_arrangements(&mut s, 0, r.springs);
    }
    println!("Day 12 part 1: {s}");
    for row in &mut rows {
        row.unfold();
    }
    let mut s = 0;
    for r in &rows {
        r.count_arrangements(&mut s, 0, r.springs);
        println!("{s}");
    }
    println!("Day 12 part 2: {s}");


    Ok(())
}

type Num = u128;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    springs: Springs,
    groups: Vec<u8>,
    total_broken: u32,
}

impl Row {

    fn unfold(&mut self) {
        self.groups = self.groups.repeat(5);
        let damaged = self.springs.damaged;
        let unknown = self.springs.unknown;
        for i in 0..4 {
            self.springs.damaged = (self.springs.damaged << (1 + self.springs.length)) | damaged;
            self.springs.unknown <<= 1;
            self.springs.unknown |= 1;
            self.springs.unknown <<= self.springs.length;
            self.springs.unknown |= unknown;
        }

        self.springs.length = self.springs.length * 5 + 4;
        self.total_broken *= 5;
    }

    fn count_arrangements(&self, count: &mut Num, bit_position: u32, springs: Springs) {
        if springs.unknown == 0 && self.matches(springs.damaged) {
            *count += 1;
            return;
        }
        if springs.contradicts(self) {
            return;
        }
        for i in bit_position..Num::from(0u8).count_zeros() {
            if springs.unknown & (1 <<i) == 0 {
                continue;
            }
            let mut s = springs;
            s.unknown &= !(1 << i);
            s.damaged |= 1 << i;
            // damaged branch first
            // println!("{:b}", s.damaged);
            // std::io::stdout().flush().unwrap();
            self.count_arrangements(count, i + 1, s);
            s.damaged &= !(1 << i);
            // println!("{:b}", s.damaged);
            // std::io::stdout().flush().unwrap();
            self.count_arrangements(count, i + 1, s);
            return;
        }
    }



    #[inline]
    fn matches(&self, mut value: Num) -> bool {
        if value.count_ones() != self.total_broken {
            return false;
        }
        if (!self.springs.unknown & self.springs.damaged) == (value & !self.springs.unknown) {
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
        //dbg!(self.springs.length);
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
        //let groups = groups.repeat(5);
        //let springs: Springs = format!("{springs}?{springs}?{springs}?{springs}?{springs}").parse()?;
        let total_broken = u32::from(groups.iter().sum::<u8>());
        Ok(Self {
            springs: springs.parse()?,
            groups,
            total_broken
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Springs {
    damaged: Num,
    unknown: Num,
    length: u8,
}

impl Springs {

    fn contradicts(&self, row: &Row) -> bool{
        if self.damaged.count_ones() > row.total_broken {
            return true;
        }
        if self.damaged.count_ones() + self.unknown.count_ones() < row.total_broken {
            return true;
        }
        let mut groups = row.groups.iter().rev();
        let mut n = *groups.next().unwrap();
        let mut unknown = self.unknown;
        let mut damaged = self.damaged;
        let mut trimmed_damaged_springs = 0;
        while damaged > 0 {
            if unknown % 2 == 1 {
                return false;
            }
            match damaged.trailing_ones().cmp(&u32::from(n)) {
                Ordering::Less => {
                    if damaged % 2 == 1
                        &&  damaged.count_ones() + unknown.count_ones() >= row.total_broken
                        && (damaged | unknown).trailing_ones() + trimmed_damaged_springs < u32::from(n) {
                        return true;
                    }
                }
                Ordering::Equal =>  {
                    unknown >>= n;
                    damaged >>= n;
                    trimmed_damaged_springs = 0;
                    n = match groups.next() {
                        Some(x) => *x,
                        None => return false,
                    }}
                Ordering::Greater => return true,
            };
            if damaged % 2 == 1 {
                trimmed_damaged_springs += 1;
            }
            unknown >>= 1;
            damaged >>= 1;
        }
        false
    }
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

        // let mut c = 0;
        // r.count_arrangements(&mut c, 0, r.springs);
        // assert_eq!(c, 1);

        let r: Row = ".??..??...?##. 1,1,3".parse().unwrap();
        let mut c = 0;
        r.count_arrangements(&mut c, 0, r.springs);
        assert_eq!(c, 4);

        let r: Row = "?###???????? 3,2,1".parse().unwrap();
        let mut c = 0;
        r.count_arrangements(&mut c, 0, r.springs);
        assert_eq!(c, 10);
    }

    #[test]
    fn it_unfolds() {
        let mut r: Row = ".# 1".parse().unwrap();
        r.unfold();
        assert_eq!(r.springs.to_string(), ".#?.#?.#?.#?.#");
        assert_eq!(r.springs.length, 14);
        assert_eq!(r.groups, vec![1,1,1,1,1]);

        let mut r: Row = "???.### 1,1,3".parse().unwrap();
        r.unfold();
        assert_eq!(r.springs.to_string(), "???.###????.###????.###????.###????.###");
        assert_eq!(r.groups, vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]);

        let r2: Row = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".parse().unwrap();
        assert_eq!(r, r2);
    }

    #[test]
    fn it_matches_part2() {
        let mut r: Row = "???.### 1,1,3".parse().unwrap();
        r.unfold();
        let mut c = 0;
        r.count_arrangements(&mut c, 0, r.springs);
        assert_eq!(c, 1);

        let r: Row = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".parse().unwrap();
        let mut c = 0;
        r.count_arrangements(&mut c, 0, r.springs);
        assert_eq!(c, 1);
        println!("before");

        let mut r: Row = ".??..??...?##. 1,1,3".parse().unwrap();
        r.unfold();
        let mut c = 0;
        r.count_arrangements(&mut c, 0, r.springs);
        assert_eq!(c, 16384);
        println!("first");

        let mut r: Row = "?###???????? 3,2,1".parse().unwrap();
        r.unfold();
        let mut c = 0;
        r.count_arrangements(&mut c, 0, r.springs);
        assert_eq!(c, 506250);
    }
}