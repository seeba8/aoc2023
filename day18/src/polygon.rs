#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self {x, y}
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Polygon {
    corners: Vec<Point>,
}

impl Polygon {
    pub fn new(corners: &[Point]) -> Self {
        Self {
            corners: corners.to_vec()
        }
    }

    /// Implementation of the <a href="https://en.wikipedia.org/wiki/Shoelace_formula">Shoelace formula</a>.
    ///
    /// This assumes that it is a "simple polygon" (no holes, no intersections) and that the corners
    /// are ordered.
    pub fn area(&self) -> usize {
        let mut a = 0isize;
        let len = self.corners.len();
        for i in 0..len {
            a += (self.corners[i].y + self.corners[(i + 1) % len].y) * (self.corners[i].x - self.corners[(i + 1) % len].x);
        }
        usize::try_from(a.abs()).unwrap() / 2
    }
    pub fn manhattan_circumference(&self) -> usize {
         let mut c = 0;
        let len = self.corners.len();
        for i in 0..len {
            let j = (i+1) % len;
            c += self.corners[i].x.abs_diff(self.corners[j].x);
            c += self.corners[i].y.abs_diff(self.corners[j].y);
        }
        c
    }

    pub fn area_with_1_wide_edge(&self) -> usize {
        self.area() + self.manhattan_circumference() / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_square() {
        let s = Polygon{corners: vec![Point::new(0,0), Point::new(2,0), Point::new(2,2), Point::new(0, 2)]};
        assert_eq!(s.area(), 4);
    }

    #[test]
    fn it_gets_square_with_edge() {
        let s = Polygon{corners: vec![Point::new(0,0), Point::new(2,0), Point::new(2,2), Point::new(0, 2)]};
        assert_eq!(s.area_with_1_wide_edge(), 9);
    }

    #[test]
    fn it_gets_circumference() {
        let s = Polygon{corners: vec![Point::new(0,0), Point::new(2,0), Point::new(2,2), Point::new(0, 2)]};
        println!("{}", s.manhattan_circumference());
    }
}