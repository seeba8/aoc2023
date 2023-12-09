use std::str::FromStr;
const INPUT: &str = include_str!("input.txt");
fn main() {
    let (almanac, numbers) = parse(INPUT).unwrap();
    println!("Day 05 part 1: {}", almanac.get_minimum(&numbers));
    println!("Day 05 part 2: {}", almanac.get_minimum_from_range(&numbers));
}


#[derive(Debug, Clone, Eq, PartialEq)]
struct MapFunction {
    destination_start: usize,
    source_start: usize,
    len: usize,
}

impl MapFunction {
    const fn apply(&self, value: usize) -> Option<usize> {
        if value < self.source_start || value >= self.source_start + self.len {
            None
        } else {
            Some(self.destination_start + (value - self.source_start))
        }
    }
}

impl FromStr for MapFunction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<usize> = s.trim().split_ascii_whitespace().map(str::parse).collect::<Result<Vec<_>, _>>()?;
        if values.len() == 3 {
            Ok(Self {
                destination_start: values[0],
                source_start: values[1],
                len: values[2],
            })
        } else {
            Err(color_eyre::eyre::eyre!("Invalid map function: {s}"))
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    map_functions: Vec<MapFunction>,
}

impl Map {
    fn apply(&self, value: usize) -> usize {
        self.map_functions.iter().find_map(|f| f.apply(value)).unwrap_or(value)
    }
}

impl FromStr for Map {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.lines().skip(1).map(MapFunction::from_str).collect::<Result<Vec<MapFunction>, _>>()?;
        Ok(Self {
            map_functions: x,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn apply(&self, value: usize) -> usize {
        self.maps.iter().fold(value, |acc, map| map.apply(acc))
    }

    fn get_minimum(&self, values: &[usize]) -> usize {
        values.iter().map(|v| self.apply(*v)).min().unwrap()
    }

    fn get_minimum_from_range(&self, values: &[usize]) -> usize {
        let mut minimum = usize::MAX;
        for v in values.chunks_exact(2) {
            let range: Vec<usize> = (v[0]..(v[0]+v[1])).collect();
            minimum = minimum.min(self.get_minimum(&range));
            dbg!(minimum);
        }
        minimum
    }
}

fn parse(input: &str) -> color_eyre::Result<(Almanac, Vec<usize>)> {
    let input = input.trim().replace("\r\n", "\n");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let numbers: Vec<usize> = parts[0].split_ascii_whitespace().skip(1).map(str::parse).collect::<Result<Vec<_>, _>>()?;
    let maps: Vec<Map> = parts.iter().skip(1).map(|section| section.parse()).collect::<Result<Vec<_>, _>>()?;
    Ok((Almanac { maps }, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn it_applies_map_function() {
        let mf = MapFunction {
            destination_start: 50,
            source_start: 98,
            len: 2,
        };
        assert_eq!(mf.apply(97), None);
        assert_eq!(mf.apply(98), Some(50));
        assert_eq!(mf.apply(99), Some(51));
        assert_eq!(mf.apply(100), None);
    }

    #[test]
    fn it_applies_map() {
        let map = Map {
            map_functions: vec![
                MapFunction {
                    destination_start: 50,
                    source_start: 98,
                    len: 2,
                },
                MapFunction {
                    destination_start: 52,
                    source_start: 50,
                    len: 48,
                }],
        };
        assert_eq!(map.apply(79), 81);
        assert_eq!(map.apply(14), 14);
        assert_eq!(map.apply(55), 57);
        assert_eq!(map.apply(13), 13);
    }

    #[test]
    fn it_parses_function() {
        let input = "50 98 2";
        let mf = MapFunction {
            destination_start: 50,
            source_start: 98,
            len: 2,
        };
        assert_eq!(MapFunction::from_str(input).unwrap(), mf);
    }

    #[test]
    fn it_parses_map() {
        let input = r"seed-to-soil map:
50 98 2
52 50 48";
        let map = Map {
            map_functions: vec![
                MapFunction {
                    destination_start: 50,
                    source_start: 98,
                    len: 2,
                },
                MapFunction {
                    destination_start: 52,
                    source_start: 50,
                    len: 48,
                }],
        };
        assert_eq!(Map::from_str(input).unwrap(), map);
    }

    #[test]
    fn it_parses_almanac() {
        let (almanac, numbers) = parse(EXAMPLE).unwrap();
        assert_eq!(almanac.maps.len(), 7);
        assert_eq!(numbers, vec![79usize, 14, 55, 13]);
    }

    #[test]
    fn it_applies_almanac() {
        let (almanac, numbers) = parse(EXAMPLE).unwrap();
        assert_eq!(almanac.apply(numbers[0]), 82);
        assert_eq!(almanac.apply(numbers[1]), 43);
        assert_eq!(almanac.apply(numbers[2]), 86);
        assert_eq!(almanac.apply(numbers[3]), 35);
    }
    #[test]
    fn it_gets_minimum() {
        let (almanac, numbers) = parse(EXAMPLE).unwrap();
        assert_eq!(almanac.get_minimum(&numbers), 35);
    }

    #[test]
    fn it_gets_minimum_with_ranges() {
        let (almanac, numbers) = parse(EXAMPLE).unwrap();
        assert_eq!(almanac.get_minimum_from_range(&numbers), 46);
    }
}