use std::str::FromStr;
use color_eyre::eyre::eyre;


const INPUT: &str = include_str!("input.txt");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut tetris: Tetris = INPUT.parse()?;
    tetris.gravity();
    println!("{}\n", tetris.plot_x());
    println!("{}\n", tetris.plot_y());
    // EX-TER-MI-NATE
    let dalek = (0..tetris.bricks.len()).filter(|i| tetris.clone().can_disintegrate(*i)).count();

    println!("Day 22 part 1: {dalek}");
    let cascade = (0..tetris.bricks.len()).map(|i| tetris.disintegrate_cascade(i)).sum::<usize>();
    println!("Day 22 part 2: {cascade}");
    Ok(())
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct Block {
    x: u16,
    y: u16,
    z: u16,
}

impl Block {
    const fn new(x: u16, y: u16, z: u16) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
/// A is the lower corner (if it is not a plane
struct Brick {
    a: Block,
    b: Block,
    /// This is needed to make sure that identically shaped blocks that are right above each other are not "merged"
    /// when simulating gravity by them being equal.
    ///
    /// When parsing the input, this contains the line number.
    ///
    /// For dummy bricks, it's None since there should only ever be one of them anyway.
    id: Option<usize>,
}

impl Brick {
    const fn new(a: Block, b: Block) -> Self {
        if b.z < a.z {
            Self { a: b, b: a, id: None}
        } else {
            Self { a, b, id: None }
        }
    }

    const fn placeholder() -> Self {
        let placeholder_block = Block::new(u16::MAX, u16::MAX, u16::MAX);
        Self::new(placeholder_block, placeholder_block)
    }
    fn min_x(&self) -> u16 {
        self.a.x.min(self.b.x)
    }
    fn max_x(&self) -> u16 {
        self.a.x.max(self.b.x)
    }

    fn min_y(&self) -> u16 {
        self.a.y.min(self.b.y)
    }
    fn max_y(&self) -> u16 {
        self.a.y.max(self.b.y)
    }

    fn min_z(&self) -> u16 {
        self.a.z.min(self.b.z)
    }
    fn max_z(&self) -> u16 {
        self.a.z.max(self.b.z)
    }

    fn move_z(&mut self, delta: i16) {
        self.a.z = self.a.z.saturating_add_signed(delta);
        self.b.z = self.b.z.saturating_add_signed(delta);
    }

    fn intersects(&self, other: &Self) -> bool {
        self.max_x() >= other.min_x()
            && self.min_x() <= other.max_x()
            && self.max_y() >= other.min_y()
            && self.min_y() <= other.max_y()
            && self.max_z() >= other.min_z()
            && self.min_z() <= other.max_z()
    }

    fn fall(&mut self, others: &[Self]) -> bool {
        if self.a.z == u16::MAX {
            return false;
        }
        let initial_z = self.a.z;
        loop {
            if others.iter().any(|o| o != self && self.intersects(o)) {
                self.move_z(1);
                return initial_z != self.a.z;
            }
            if self.a.z == 1 {
                return initial_z != self.a.z;
            }
            self.move_z(-1);
        }
    }
}

impl FromStr for Block {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(',');
        let x: u16 = splitted.next().ok_or_else(|| eyre!("cannot get first coordinate"))?.parse()?;
        let y: u16 = splitted.next().ok_or_else(|| eyre!("cannot get first coordinate"))?.parse()?;
        let z: u16 = splitted.next().ok_or_else(|| eyre!("cannot get first coordinate"))?.parse()?;
        if splitted.next().is_some() {
            return Err(eyre!("Block has more than 3 coordinates"));
        }
        Ok(Self::new(x, y, z))
    }
}

impl FromStr for Brick {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('~').ok_or_else(|| eyre!("cannot split by tilde"))?;
        Ok(Self::new(a.parse()?, b.parse()?))
    }
}

#[derive(Debug, Clone)]
struct Tetris {
    bricks: Vec<Brick>,
}

impl Tetris {
    fn can_disintegrate(&mut self, index: usize) -> bool {
        let mut placeholder_brick = Brick::placeholder();
        std::mem::swap(&mut self.bricks[index], &mut placeholder_brick);
        let res = !self.gravity();
        std::mem::swap(&mut self.bricks[index], &mut placeholder_brick);
        res
    }

    fn disintegrate_cascade(&self, index: usize) -> usize {
        let mut t = self.clone();
        let mut placeholder_brick = Brick::placeholder();
        std::mem::swap(&mut t.bricks[index], &mut placeholder_brick);
        t.gravity();
        std::mem::swap(&mut t.bricks[index], &mut placeholder_brick);
        self.bricks.iter().enumerate().filter(|(i, b)|*i != index && **b != t.bricks[*i]).count()
    }

    fn gravity(&mut self) -> bool {
        let mut placeholder_brick = Brick::placeholder();
        let mut final_movement = false;
        let mut one_has_moved = true;
        while one_has_moved {
            one_has_moved = false;
            for i in 0..self.bricks.len() {
                std::mem::swap(&mut self.bricks[i], &mut placeholder_brick);
                if placeholder_brick.fall(&self.bricks) {
                    one_has_moved = true;
                    final_movement = true;
                    //println!("{i} has fallen");
                }

                std::mem::swap(&mut self.bricks[i], &mut placeholder_brick);
            }
        }
        final_movement
    }
    #[allow(unused)]
    fn plot_x(&self) -> String {
        let min_x = self.bricks.iter().map(Brick::min_x).min().unwrap();
        let max_x = self.bricks.iter().map(Brick::max_x).max().unwrap();
        let mut s = String::new();
        s.push_str(&format!("{:^1$}   \n", "x", (1 + max_x - min_x) as usize));
        for i in min_x..=max_x {
            s.push_str(&format!("{}", i % 10));
        }
        s.push('\n');
        let min_z = self.bricks.iter().map(Brick::min_z).min().unwrap();
        let max_z = self.bricks.iter().map(Brick::max_z).max().unwrap();
        for z in (min_z..=max_z).rev() {
            // if z == 1 {
            //     dbg!("z");
            // }
            for x in min_x..=max_x {
                let line = Brick::new(Block::new(x, u16::MIN, z), Block::new(x, u16::MAX, z));
                let intersections: Vec<usize> = (0..self.bricks.len()).filter(|i| {
                    line.intersects(&self.bricks[*i])
                }).collect();
                // if z == 1 && x == 2 {
                //     dbg!(line);
                //     dbg!(line.min_x());
                //     dbg!(line.max_x());
                //     dbg!(line.min_y());
                //     dbg!(line.max_y());
                //     dbg!(line.min_z());
                //     dbg!(line.max_z());
                // }
                #[allow(clippy::cast_possible_truncation)]
                match intersections.len() {
                    0 => s.push('.'),
                    1 => s.push((65 + (intersections[0] % 58)) as u8 as char),
                    _ => s.push('?'),
                };
            }
            s.push_str(&format!(" {z}{}\n", if z == (max_z + 2 - min_z) / 2 { " z" } else { "" }));
        }
        s.push_str(&format!("{} 0", "-".repeat((max_x - min_x + 1) as usize)));
        s
    }
    #[allow(unused)]
    fn plot_y(&self) -> String {
        let min_y = self.bricks.iter().map(Brick::min_y).min().unwrap();
        let max_y = self.bricks.iter().map(Brick::max_y).max().unwrap();
        let mut s = String::new();
        s.push_str(&format!("{:^1$}   \n", "y", (1 + max_y - min_y) as usize));
        for i in min_y..=max_y {
            s.push_str(&format!("{}", i % 10));
        }
        s.push('\n');
        let min_z = self.bricks.iter().map(Brick::min_z).min().unwrap();
        let max_z = self.bricks.iter().map(Brick::max_z).max().unwrap();
        for z in (min_z..=max_z).rev() {
            // if z == 1 {
            //     dbg!("z");
            // }
            for y in min_y..=max_y {
                let line = Brick::new(Block::new(u16::MIN, y, z), Block::new(u16::MAX, y, z));
                let intersections: Vec<usize> = (0..self.bricks.len()).filter(|i| {
                    line.intersects(&self.bricks[*i])
                }).collect();
                // if z == 1 && x == 2 {
                //     dbg!(line);
                //     dbg!(line.min_x());
                //     dbg!(line.max_x());
                //     dbg!(line.min_y());
                //     dbg!(line.max_y());
                //     dbg!(line.min_z());
                //     dbg!(line.max_z());
                // }
                #[allow(clippy::cast_possible_truncation)]
                match intersections.len() {
                    0 => s.push('.'),
                    1 => s.push((65 + (intersections[0] % 58)) as u8 as char),
                    _ => s.push('?'),
                };
            }
            s.push_str(&format!(" {z}{}\n", if z == (max_z + 2 - min_z) / 2 { " z" } else { "" }));
        }
        s.push_str(&format!("{} 0", "-".repeat((max_y - min_y + 1) as usize)));
        s
    }
}

impl FromStr for Tetris {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bricks: s.lines().enumerate().map(|(i, line)| {
                let brick = Brick::from_str(line);
                brick.map(|mut b| {
                    b.id = Some(i);
                    b
                })
                }).collect::<Result<Vec<_>, _>>()?
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE: &str = include_str!("example.txt");

        #[test]
        fn it_parses_tetris() {
            let tetris: Tetris = EXAMPLE.parse().unwrap();
            assert_eq!(tetris.bricks.len(), 7);
        }

        #[test]
        fn it_simulates_gravity() {
            let mut tetris: Tetris = EXAMPLE.parse().unwrap();
            dbg!(&tetris);
            assert_eq!(tetris.bricks[6].min_z(), 8);
            assert!(tetris.gravity());
            dbg!(&tetris);
            assert_eq!(tetris.bricks[6].min_z(), 5);
            assert!(!tetris.gravity());
        }

        #[test]
        fn it_can_safely_disintegrate() {
            let mut tetris: Tetris = EXAMPLE.parse().unwrap();
            println!("{:?}", tetris.bricks[1]);
            println!("{}", tetris.plot_x());
            println!("{}", tetris.plot_y());
            assert!(tetris.gravity());
            println!("{}", tetris.plot_x());
            println!("{}", tetris.plot_y());
            assert!(!tetris.clone().can_disintegrate(0));
            assert!(tetris.can_disintegrate(1));
            assert!(tetris.can_disintegrate(2));
            assert!(tetris.can_disintegrate(3));
            assert!(tetris.can_disintegrate(4));
            assert!(!tetris.clone().can_disintegrate(5));
            assert!(tetris.can_disintegrate(6));
        }

        #[test]
        fn it_can_disintegrate_sum() {
            let mut tetris: Tetris = EXAMPLE.parse().unwrap();
            dbg!(&tetris);
            tetris.gravity();
            let dalek = (0..tetris.bricks.len()).filter(|i| tetris.clone().can_disintegrate(*i)).count();
            assert_eq!(dalek, 5);
        }

        #[test]
        fn it_counts_disintegration_cascade() {
            let mut tetris: Tetris = EXAMPLE.parse().unwrap();
            tetris.gravity();
            assert_eq!(tetris.disintegrate_cascade(0), 6);
            assert_eq!(tetris.disintegrate_cascade(5), 1);
            assert_eq!((0..tetris.bricks.len()).map(|i| tetris.disintegrate_cascade(i)).sum::<usize>(), 7);
        }
    }