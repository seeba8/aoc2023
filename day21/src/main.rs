use std::collections::VecDeque;
use std::str::FromStr;
use color_eyre::eyre::eyre;
const INPUT: &str = include_str!("input.txt");
fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    let map: Map = INPUT.parse()?;
    let out = map.dijkstra(64);
    let num = out.iter().filter(|v| **v <= 64 && **v % 2 == 0).count();
    println!("Day 21 part 1: {num}");
    Ok(())
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Block {
    Garden,
    Rock,
}

struct Map {
    grid: Vec<Block>,
    width: usize,
    height: usize,
    start: usize,
}

impl Map {
    fn iget(&self, index: usize) -> Block {
        self.grid[index]
    }

    fn get(&self, x: usize, y: usize) -> Block {
        self.grid[y * self.width + x]
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<usize> {
        let mut neighbours: Vec<usize> = Vec::with_capacity(4);
        if x > 0 && self.get(x - 1, y) == Block::Garden {
            neighbours.push(y * self.width + x - 1);
        }
        if y > 0 && self.get(x, y - 1) == Block::Garden {
            neighbours.push((y - 1) * self.width + x);
        }
        if x < self.width - 1 && self.get(x + 1, y) == Block::Garden {
            neighbours.push(y * self.width + x + 1);
        }
        if y < self.height - 1 && self.get(x, y + 1) == Block::Garden {
            neighbours.push((y + 1) * self.width + x);
        }
        neighbours
    }

    fn ineighbours(&self, index: usize) -> Vec<usize> {
        self.neighbours(index % self.width, index / self.width)
    }

    fn dijkstra(&self, max_distance: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.grid.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();
        distances[self.start] = 0;
        queue.push_back(self.start);
        while let Some(current) = queue.pop_front() {
            if distances[current] > max_distance {
                return distances;
            }
            for n in self.ineighbours(current) {
                if distances[n] > distances[current] + 1 {
                    distances[n] = distances[current] + 1;
                    queue.push_back(n);
                }
            }
        }
        distances
    }
    #[allow(unused)]
    fn result_to_string(&self, result: &[usize]) -> String {
        let mut out = String::with_capacity((self.width + 1) * self.height + 1);
        for (i, r) in result.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                out.push('\n');
            }
            if *r < usize::MAX && r % 2 == 0 {
                out.push('O');
            } else {
                out.push(match self.iget(i) {
                    Block::Garden => '.',
                    Block::Rock => '#',
                });
            }
        }
        out.push('\n');
        out
    }
}

impl FromStr for Map {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or_else(|| eyre!("Cannot get first line"))?.len();
        let height = s.lines().count();
        let mut start = 0;
        let grid: Vec<Block> = s
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .enumerate()
            .map(|(i, c)| {
                match c {
                    '#' => Block::Rock,
                    '.' => Block::Garden,
                    'S' => {
                        start = i;
                        Block::Garden
                    }
                    _ => panic!("Illegal character: {c}"),
                }
            }).collect();
        Ok(Self { grid, width, height, start })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE2X2: &str = include_str!("example2.txt");
    const EXAMPLE3X3: &str = include_str!("example3.txt");
    #[test]
    fn it_parses_example() {
        let map: Map = EXAMPLE.parse().unwrap();
        assert_eq!(map.height, 11);
        assert_eq!(map.width, 11);
        assert_eq!(map.start, 60);
    }

    #[test]
    fn it_dijkstras() {
        let map: Map = EXAMPLE.parse().unwrap();
        let result = map.dijkstra(6);
        assert_eq!(result.iter().filter(|v| **v <= 6 && **v % 2 == 0).count(), 16);
    }

    #[test]
    fn it_displays_output() {
        let map: Map = EXAMPLE.parse().unwrap();
        let result = map.dijkstra(6);
        let expected = r"...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........
";
        assert_eq!(map.result_to_string(&result), expected);
    }

    #[test]
    fn it_finds_even_number_of_steps() {
        let map: Map = EXAMPLE3X3.parse().unwrap();
        let result = map.dijkstra(usize::MAX);
        assert_eq!(50, result.iter().filter(|v| **v <= 10 && **v % 2 == 0).count());
        println!("{}", map.result_to_string(&result.iter().map(|v| if *v <= 10 { v} else { &usize::MAX}).copied().collect::<Vec<_>>()));
    }
}