#![feature(let_chains)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use color_eyre::eyre::eyre;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut grid: Grid = INPUT.parse()?;
    grid.find_edges(true);
    println!("Day 23 part 1: {}", grid.longest_path_dijkstra());

    let mut grid: Grid = INPUT.parse()?;
    grid.find_edges(false);
    println!("Day 23 part 2: {}", grid.longest_path());
    Ok(())
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Block {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Path => '.',
            Self::Forest => '#',
            Self::SlopeUp => '^',
            Self::SlopeDown => 'v',
            Self::SlopeLeft => '<',
            Self::SlopeRight => '>',
        })
    }
}

impl TryFrom<char> for Block {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            'v' => Ok(Self::SlopeDown),
            '<' => Ok(Self::SlopeLeft),
            '^' => Ok(Self::SlopeUp),
            '>' => Ok(Self::SlopeRight),
            _ => Err(eyre!("Illegal character: {value}"))
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Edge {
    start: usize,
    end: usize,
    length: usize,
    // maybe remove later?
    path: Vec<usize>,
}

struct Grid {
    tiles: Vec<Block>,
    edges: HashSet<Edge>,
    width: usize,
    start: usize,
    end: usize,
}

impl FromStr for Grid {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().ok_or_else(|| eyre!("Cannot get first line"))?.len();
        let tiles: Vec<Block> = s.chars().filter_map(|c| Block::try_from(c).ok()).collect();
        assert_eq!(tiles.len(), width * height);
        Ok(Self {
            tiles,
            edges: HashSet::new(),
            width,
            start: 1,
            end: width * height - 2,
        })
    }
}

impl Grid {
    /// since we come from one direction, we can go in at most 3 directions. But iterating through all 4 is easier
    fn neighbours(&self, position: usize, previous: usize, impassible_slopes: bool) -> [Option<usize>; 4] {
        let mut res: [Option<usize>; 4] = [None; 4];
        if position / self.width == 0 || position >= self.tiles.len() - self.width {
            // top or bottom row where we can't be outside of start/end
            return res;
        }
        if position % self.width == 0 || (position + 1) % self.width == 0 {
            // left or right most column
            return res;
        }

        for (i, (neighbour, impassible_slope)) in [(position - self.width, Block::SlopeDown),
            (position + 1, Block::SlopeLeft),
            (position + self.width, Block::SlopeUp),
            (position - 1, Block::SlopeRight)]
            .into_iter()
            .enumerate() {
            let tile = self.tiles[neighbour];
            if neighbour != previous && tile != Block::Forest
                && (!impassible_slopes || tile != impassible_slope) {
                res[i] = Some(neighbour);
            }
        }
        res
    }

    fn find_edges(&mut self, slopes_are_impassible: bool) {
        let current_edge = Edge { start: self.start, ..Default::default() };
        let previous = self.start;
        let position = self.start + self.width;
        self.follow_edge(current_edge, position, previous, slopes_are_impassible);
    }

    fn follow_edge(&mut self, mut current_edge: Edge, mut position: usize, mut previous: usize, slopes_are_impassible: bool) {
        while position != self.end {
            current_edge.length += 1;

            let neighbours = self.neighbours(position, previous, slopes_are_impassible);
            if neighbours.iter().filter_map(|c| *c).count() == 1 {
                // no intersection
                #[cfg(debug_assertions)]
                {
                    current_edge.path.push(position);
                }
                previous = position;
                position = neighbours.into_iter().find_map(|n| n).unwrap();
                continue;
            }
            // we have reached an intersection!
            current_edge.end = position;
            if self.edges.insert(current_edge) {
                for neighbour in neighbours.into_iter().flatten() {
                    self.follow_edge(Edge { start: position, ..Default::default() }, neighbour, position, slopes_are_impassible);
                }
            }
            return;
        }
        current_edge.length += 1;
        current_edge.end = position;
        self.edges.insert(current_edge);
    }

    fn longest_path(&self) -> usize {
        let mut res = 0;
        self.longest_path_bruteforce(self.start, &mut HashSet::new(), 0, &mut res);
        res
    }

    /// Dijkstra with negative edge weight, assuming we are in a DAG.
    /// if not: brute force should work, not too many edges
    fn longest_path_dijkstra(&self) -> usize {
        assert!(!self.edges.is_empty(), "First run `find_edges()`");
        let mut visited_vertices = HashSet::new();
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(self.start);
        distances.insert(self.start, 0);
        while let Some(u) = queue.pop_front() {
            let distance_to_u = distances[&u];
            let new_node = visited_vertices.insert(u);
            for n in self.edges.iter().filter(|edge| edge.start == u) {
                distances.entry(n.end).and_modify(|distance| {
                    if *distance < distance_to_u + n.length {
                        *distance = distance_to_u + n.length;
                        if new_node {
                            queue.push_back(n.end);
                        }
                    }
                }).or_insert_with(|| {
                    if new_node {
                        queue.push_back(n.end);
                    }
                    distance_to_u + n.length
                });
            }
        }
        distances[&self.end]
    }

    /// Yay recursive brute force algorithm
    fn longest_path_bruteforce(&self, position: usize, visited: &mut HashSet<usize>, distance: usize, longest_distance: &mut usize) {
        if position == self.end {
            if distance > *longest_distance {
                *longest_distance = distance;
            }
            return;
        }
        let unvisited_edges = self.edges.iter().filter(|edge| edge.start == position && !visited.contains(&edge.end)).collect_vec();
        for edge in  unvisited_edges{
            visited.insert(edge.end);
            self.longest_path_bruteforce(edge.end, visited, distance + edge.length, longest_distance);
            visited.remove(&edge.end);
        }
    }

    /// Returns a Mermaid.js compatible string for a flowchart
    #[allow(unused)]
    fn mermaid(&self) -> String {
        assert!(!self.edges.is_empty(), "First run `find_edges()`");
        let unique_edges = self.edges.iter().unique_by(|e| {
            (e.length, e.start.min(e.end), e.start.max(e.end))
        }).collect_vec();
        dbg!(unique_edges.len());
        dbg!(self.edges.len());
        let arrow_sign = if unique_edges.len() == self.edges.len() {
            '>'
        } else {
            '-'
        };
        let mut out = String::from("flowchart TD\n");
        for &Edge { start, length, end, .. } in unique_edges {
            out.push_str(&format!("\t{start} --{arrow_sign}|{length}| {end}\n"));
        }
        out
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.tiles.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            if *c == Block::Path && let Some(edge) = self.edges.iter().position(|e| e.path.contains(&i)) {
                write!(f, "{}", u8::try_from(65 + (edge % 26)).unwrap() as char)?;
            } else {
                write!(f, "{c}")?;
            }
        }
        if !self.edges.is_empty() {
            writeln!(f)?;
            for (i, edge) in self.edges.iter().enumerate() {
                writeln!(f, "{}: {{ start: {}, end: {}, length: {} }}", u8::try_from(65 + (i % 26)).unwrap() as char, edge.start, edge.end, edge.length)?;
            }
        }
        Ok(())
    }
}

/*
Hypothesis: Are all intersections surrounded by direction markers?
There are different intersections:
- 1->2 (e.g. B)
- 2->1 (e.g. G) -> Not really an intersection, can be ignored.
- 2->2 (e.g. H)
A -15-> B

Are we in a DAG? https://en.wikipedia.org/wiki/Longest_path_problem#Acyclic_graphs

*/
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn it_parses_example() {
        let grid: Grid = EXAMPLE.parse().unwrap();
        assert_eq!(grid.width, 23);
        assert_eq!(grid.tiles[0], Block::Forest);
        assert_eq!(grid.tiles[1], Block::Path);
    }

    #[test]
    fn it_gets_neighbours() {
        let grid: Grid = EXAMPLE.parse().unwrap();
        let neighbours = grid.neighbours(grid.width + 1, grid.start, true);
        assert_eq!(neighbours.iter().filter_map(|c| *c).collect::<Vec<_>>(), vec![grid.width + 2]);
    }

    /**
    ```mermaid
    flowchart TD
        80 -->|24| 312
        312 -->|33| 527
        312 -->|25| 527
        118 -->|22| 80
        1 -->|15| 118
        304 -->|53| 527
        304 -->|12| 312
        118 -->|22| 304
        80 -->|45| 527
    ```
     */
    #[test]
    fn it_finds_edges() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        grid.find_edges(true);
        println!("{grid}");
        assert_eq!(grid.edges.len(), 9);
    }

    #[test]
    fn it_gets_mermaid_diagram() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        grid.find_edges(true);
        let expected = r"flowchart TD
	80 -->|24| 312
	312 -->|33| 527
	312 -->|25| 527
	118 -->|22| 80
	1 -->|15| 118
	304 -->|53| 527
	304 -->|12| 312
	118 -->|22| 304
	80 -->|45| 527";
        let actual = grid.mermaid();
        assert!(actual.lines().all(|line| expected.contains(line)));
    }

    #[test]
    fn it_finds_longest_path() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        grid.find_edges(true);
        assert_eq!(grid.longest_path(), 94);
    }

    #[test]
    fn it_finds_longest_path_with_dijkstra() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        grid.find_edges(true);
        assert_eq!(grid.longest_path_dijkstra(), 94);
    }

    #[test]
    fn it_finds_edges_ignoring_slopes() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        grid.find_edges(false);
        assert_eq!(grid.edges.len(), 23);
    }

    #[test]
    fn it_finds_longest_path_ignoring_slopes() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        grid.find_edges(false);
        assert_eq!(grid.longest_path(), 154);
    }
}