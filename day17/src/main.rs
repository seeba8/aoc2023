#![feature(let_chains)]

use std::collections::HashMap;
use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::Report;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let mut graph: Graph = input.parse()?;
    let res = graph.get_min_heat_loss(1, 3);
    println!("Day 17 part 1: {res}");

    let mut graph: Graph = input.parse()?;
    let res = graph.get_min_heat_loss(4, 10);
    println!("Day 17 part 2: {res}");
    Ok(())
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

impl Direction {
    fn is_allowed(self, direction: Self) -> bool {
        match self {
            Self::North | Self::South => direction != Self::North && direction != Self::South,
            Self::East | Self::West => direction != Self::East && direction != Self::West,
            Self::None => true,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::South, Direction::West, Direction::East, Direction::North];

struct Graph {
    /// Direction is the movement towards this location, meaning it can't continue the same direction
    distance: HashMap<(Direction, usize), usize>,
    /// Direction is the movement towards this location, meaning it can't continue the same direction
    prev: HashMap<(Direction, usize), (Direction, usize)>,
    cost: Vec<usize>,
    width: usize,
    height: usize,
    /// Direction is the movement towards this location, meaning it can't continue the same direction
    queue: Vec<(Direction, usize)>,
}

impl Graph {
    fn new(cost: Vec<usize>, width: usize, height: usize) -> Self {
        Self {
            distance: HashMap::new(),
            prev: HashMap::new(),
            cost,
            width,
            height,
            queue: vec![],
        }
    }
    fn dijkstra(&mut self, start: usize, min_movement: usize, max_movement: usize) {
        self.init(start);
        while !self.queue.is_empty() {
            let mut u = 0;
            for i in 1..self.queue.len() {
                if self.distance[&self.queue[i]] < self.distance[&self.queue[u]] {
                    u = i;
                }
            }
            let u = self.queue.remove(u);
            for direction in DIRECTIONS {
                if !direction.is_allowed(u.0) {
                    continue;
                }
                let mut dist = 0;

                for i in 1..=max_movement {
                    if let Some(neighbour) = match direction {
                        Direction::None => unreachable!(),
                        Direction::North => u.1.checked_sub(i * self.width),
                        Direction::South => {
                            let neighbour = u.1 + i * self.width;
                            if neighbour >= self.width * self.height {
                                None
                            } else {
                                Some(neighbour)
                            }
                        }
                        Direction::East => {
                            let neighbour = u.1 + i;
                            if neighbour % self.width < u.1 % self.width // we wrapped around
                                || neighbour >= self.width * self.height {
                                None
                            } else {
                                Some(neighbour)
                            }
                        }
                        Direction::West => {
                            match u.1.checked_sub(i) {
                                None => { None }
                                Some(neighbour) => {
                                    if neighbour % self.width > u.1 % self.width { // we wrapped around
                                        None
                                    } else {
                                        Some(neighbour)
                                    }
                                }
                            }
                        }
                    } {
                        let n = (direction, neighbour);
                        dist += self.cost[neighbour];
                        if i < min_movement {
                            continue;
                        }
                        //println!("neighbour: {neighbour}");
                        let alternative = self.distance[&u] + dist;
                        match self.distance.get(&n) {
                            None => {
                                self.distance.insert(n, alternative);
                                self.prev.insert(n, u);
                                self.queue.push(n);
                            }
                            Some(d) => {
                                if alternative < *d {
                                    self.distance.insert(n, alternative);
                                    self.prev.insert(n, u);
                                    self.queue.push(n);
                                }
                            }
                        };
                    }
                }
            }
        }
    }

    fn init(&mut self, start: usize) {
        self.distance.insert((Direction::None, start), 0);
        self.queue.push((Direction::None, start));
    }

    fn get_min_heat_loss(&mut self, min_movement: usize, max_movement: usize) -> usize {
        self.dijkstra(0, min_movement, max_movement);
        return *DIRECTIONS
            .iter()
            .filter_map(|direction| self.distance.get(&(*direction, self.width * self.height - 1)))
            .min()
            .unwrap();
    }
}


impl FromStr for Graph {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().ok_or_else(|| eyre!("Cannot get first line"))?.len();
        let cost: Vec<usize> = s.chars().filter_map(|c| c.to_digit(10).and_then(|v| v.try_into().ok())).collect();
        Ok(Self::new(cost, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");

    #[test]
    fn it_parses_graph() {
        let graph: Graph = EXAMPLE1.parse().unwrap();
        assert_eq!(graph.width, 13);
        assert_eq!(graph.height, 13);
        assert_eq!(graph.cost[0], 2);
    }

    #[test]
    fn it_finds_cost() {
        let mut graph: Graph = EXAMPLE1.parse().unwrap();
        assert_eq!(graph.get_min_heat_loss(1, 3), 102);
        //graph.dijkstra(0, graph.width * graph.height - 1);
        //for direction in DIRECTIONS {
        //    let mut n = (direction, graph.width * graph.height - 1);
        //    println!("{:?}", graph.distance.get(&n));
        // while let Some(pr) = graph.prev.get(&n) {
        //     println!("{pr:?}");
        //     n = *pr;
        // }
        //}
    }

    #[test]
    fn it_finds_cost_of_ultra_crucible() {
        let mut graph: Graph = EXAMPLE1.parse().unwrap();
        assert_eq!(graph.get_min_heat_loss(4, 10), 94);
    }

    #[test]
    fn it_finds_cost_of_ultra_crucible2() {
        let mut graph: Graph = EXAMPLE2.parse().unwrap();
        assert_eq!(graph.get_min_heat_loss(4, 10), 71);
    }
}