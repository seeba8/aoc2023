#![feature(let_chains)]

use std::ops::Add;
use std::str::FromStr;
use color_eyre::eyre::eyre;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let hailstones: Vec<Hailstone> = INPUT
        .lines()
        .map(Hailstone::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let rect = (
        Vec3::new(200_000_000_000_000_f64, 200_000_000_000_000., 0.),
        Vec3::new(400_000_000_000_000_f64, 400_000_000_000_000., 0.));
    let intersects = hailstones
        .iter()
        .combinations(2)
        .filter_map(|a| a[0].ray_intersect_2d(a[1]))
        .filter(|a| a.is_in_rect(&rect))
        .count();
    println!("Day 24 part 1: {intersects}"); // 17908 is too high
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl Add for Vec3<i128> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Vec3<T> where T: PartialOrd + Copy {
    const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    fn is_in_rect(&self, rect: &(Self, Self)) -> bool {
        self.x >= rect.0.x
            && self.x <= rect.1.x
            && self.y >= rect.0.y
            && self.y <= rect.1.y
            && self.z >= rect.0.z
            && self.z <= rect.1.z
    }
}

impl Vec3<i128> {
    const fn xy(&self) -> (i128, i128) {
        (self.x, self.y)
    }
}

impl FromStr for Vec3<i128> {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<i128> = s.split(',').map(|i| i128::from_str(i.trim())).collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(segments[0], segments[1], segments[2]))
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
struct Hailstone {
    position: Vec3<i128>,
    velocity: Vec3<i128>,
}

impl Hailstone {
    fn next_position(&self) -> Vec3<i128> {
        self.position + self.velocity
    }

    /// Source: [https://stackoverflow.com/a/20677983](https://stackoverflow.com/a/20677983)
    ///
    /// I have no idea how to do this without allowing the cast to float with precision loss.
    /// It's probably lucky that it works, but without using i128,
    /// there would be an integer overflow in `det()`.
    #[allow(clippy::cast_precision_loss)]
    fn line_intersect_2d(&self, other: &Self) -> Option<Vec3<f64>> {
        const fn det(a: (i128, i128), b: (i128, i128)) -> i128 {
            a.0 * b.1 - a.1 * b.0
        }
        let x_diff = (-self.velocity.x, -other.velocity.x);
        let y_diff = (-self.velocity.y, -other.velocity.y);
        let div = det(x_diff, y_diff);
        if div == 0 {
            return None;
        }
        let d = (det(self.position.xy(), self.next_position().xy()), det(other.position.xy(), other.next_position().xy()));
        let x = det(d, x_diff) as f64 / div as f64;
        let y = det(d, y_diff) as f64 / div as f64;
        Some(Vec3::new(x, y, 0.))
    }

    #[allow(clippy::cast_precision_loss)]
    fn ray_intersect_2d(&self, other: &Self) -> Option<Vec3<f64>> {
        if let Some(i) = self.line_intersect_2d(other) && (i.x - self.position.x as f64) / self.velocity.x as f64 > 0.
            && (i.x - other.position.x as f64) / other.velocity.x as f64 > 0. {
            Some(i)
        } else {
            None
        }
    }
}

impl FromStr for Hailstone {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once('@').ok_or_else(|| eyre!("Cannot split line by @: {s}"))?;
        Ok(Self {
            position: pos.parse()?,
            velocity: vel.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn it_finds_line_intersect() {
        let a: Hailstone = "18, 19, 22 @ -1, -1, -2".parse().unwrap();
        let b: Hailstone = "12, 31, 28 @ -1, -2, -1".parse().unwrap();
        let c: Hailstone = "19, 13, 30 @ -2,  1, -2".parse().unwrap();
        assert_eq!(a.line_intersect_2d(&b), Some(Vec3::new(-6., -5., 0.)));
        assert_eq!(c.line_intersect_2d(&b), Some(Vec3::new(6.2, 19.4, 0.)));
        assert!(c.line_intersect_2d(&a).is_some());
    }

    #[test]
    fn it_finds_ray_intersects() {
        let hailstones: Vec<Hailstone> = EXAMPLE.lines().map(Hailstone::from_str).collect::<Result<Vec<_>, _>>().unwrap();
        assert!(hailstones[0].ray_intersect_2d(&hailstones[1]).is_some());
        assert!(hailstones[0].ray_intersect_2d(&hailstones[2]).is_some());
        assert!(hailstones[0].ray_intersect_2d(&hailstones[3]).is_some());
        assert!(hailstones[0].ray_intersect_2d(&hailstones[4]).is_none());
        assert!(hailstones[1].ray_intersect_2d(&hailstones[2]).is_none());
        assert!(hailstones[1].ray_intersect_2d(&hailstones[3]).is_some());
        assert!(hailstones[2].ray_intersect_2d(&hailstones[4]).is_none());
    }

    #[test]
    fn it_finds_ray_intersects_in_rect() {
        let hailstones: Vec<Hailstone> = EXAMPLE.lines().map(Hailstone::from_str).collect::<Result<Vec<_>, _>>().unwrap();
        let rect = (Vec3::new(7., 7., 0.), Vec3::new(27., 27., 0.));
        let intersects = hailstones
            .iter()
            .combinations(2)
            .filter_map(|a| a[0].ray_intersect_2d(a[1]))
            .filter(|a| a.is_in_rect(&rect))
            .collect_vec();
        assert_eq!(intersects.len(), 2);
    }
}