#![feature(ascii_char)]
#![feature(inline_const)]

use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let hash: usize = input.split(',').map(|s| s.reindeer_hash() as usize).sum();
    println!("Day 15 part 1: {hash}");

    let operations: Vec<Operation> = input.split(',').map(str::parse).collect::<Result<Vec<Operation>, _>>().unwrap();
    let boxes: Boxes = operations.try_into().unwrap();
    println!("Day 15 part 2: {}", boxes.focusing_power());
}

trait ReindeerHash {
    fn reindeer_hash(&self) -> u8;
}

impl ReindeerHash for &str {
    fn reindeer_hash(&self) -> u8 {
        let mut hash: u8 = 0;
        for c in self.trim().as_ascii().unwrap() {
            hash = hash.wrapping_add(c.to_u8());
            hash = hash.wrapping_mul(17);
        }
        hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Operation {
    Remove(String),
    AddReplace(Lens),
}

impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\w+)(-)?=?(\d)?").unwrap());
        let cap = RE.captures(s).ok_or_else(|| eyre!("Cannot match {s}"))?;
        if let Some(v) = cap.get(3) {
            let focal_length: u8 = v.as_str().parse()?;
            Ok(Self::AddReplace(Lens { focal_length, label: cap[1].to_string() }))
        } else {
            Ok(Self::Remove(cap[1].to_string()))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Boxes {
    boxes: [Vec<Lens>; 256],
}

impl Boxes {
    const fn new() -> Self {
        Self {
            boxes: [const { vec![] }; 256],
        }
    }

    fn focusing_power(&self) -> usize {
        let mut s = 0;
        for (idx, b) in self.boxes.iter().enumerate() {
            for (slot, lens) in b.iter().enumerate() {
                s += (idx + 1) * (slot + 1) * (lens.focal_length as usize);
            }
        }
        s
    }
}


impl TryFrom<Vec<Operation>> for Boxes {
    type Error = Report;

    fn try_from(operations: Vec<Operation>) -> Result<Self, Self::Error> {
        let mut boxes = Self::new();
        for op in operations {
            match op {
                Operation::Remove(l) => {
                    let b = boxes.boxes.get_mut(l.as_str().reindeer_hash() as usize).ok_or_else(|| eyre!("cannot get hash"))?;
                    let idx = b.iter().position(|p| p.label == *l);
                    if let Some(idx) = idx {
                        b.remove(idx);
                    }
                }
                Operation::AddReplace(l) => {
                    let hash = l.label.as_str().reindeer_hash();
                    let b = boxes.boxes.get_mut(hash as usize).ok_or_else(|| eyre!("cannot get hash"))?;
                    let idx = b.iter().position(|p| p.label == l.label);
                    if let Some(idx) = idx {
                        b[idx] = l;
                    } else {
                        b.push(l);
                    }
                }
            }
        }
        Ok(boxes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hashes() {
        assert_eq!("rn=1".reindeer_hash(), 30);
        assert_eq!("cm-".reindeer_hash(), 253);
    }

    #[test]
    fn it_hashes_sums() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let hash: usize = input.split(',').map(|s| s.reindeer_hash() as usize).sum();
        assert_eq!(hash, 1320);
    }

    #[test]
    fn it_parses_operation() {
        let op: Operation = "rn=1".parse().unwrap();
        assert_eq!(op, Operation::AddReplace(Lens { label: "rn".to_string(), focal_length: 1 }));

        let op: Operation = "cm-".parse().unwrap();
        assert_eq!(op, Operation::Remove("cm".to_string()));
    }

    #[test]
    fn it_gets_boxes() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let operations: Vec<Operation> = input.split(',').map(str::parse).collect::<Result<Vec<Operation>, _>>().unwrap();
        let boxes: Boxes = operations.try_into().unwrap();
        assert_eq!(boxes.boxes[0].len(), 2);
        assert_eq!(boxes.boxes[3].len(), 3);
    }

    #[test]
    fn it_gets_focusing_power() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let operations: Vec<Operation> = input.split(',').map(str::parse).collect::<Result<Vec<Operation>, _>>().unwrap();
        let boxes: Boxes = operations.try_into().unwrap();
        assert_eq!(boxes.focusing_power(), 145);
    }
}