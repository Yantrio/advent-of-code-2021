use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use regex::Regex;

fn main() {
    let steps: Vec<Step> = include_str!("input")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    let p1 = part_1(&steps);
    println!("Part 1: {:#?}", p1);

    let p2 = part_2(&steps);
    println!("Part 2: {:#?}", p2);
}

fn part_2(steps: &[Step]) -> i128 {
    let mut cuboids: Vec<Cuboid> = vec![];
    for step in steps.iter() {
        let mut new_cuboids = vec![];
        for c in cuboids {
            new_cuboids.append(&mut c.exclude_cuboid(&step.cuboid));
        }
        if step.state {
            new_cuboids.push(step.cuboid);
        }
        cuboids = new_cuboids;
    }
    cuboids.iter().map(|c| c.volume()).sum::<i128>()
}

fn part_1(steps: &[Step]) -> usize {
    let mut space = Space::new();
    for step in steps.iter() {
        for position in step.cuboid.get_all(|&n| n >= -50 && n <= 50) {
            space.set_position(position, step.state)
        }
    }
    space.count_on()
}

#[derive(Debug)]
struct Space {
    map: HashMap<Vec3, bool>,
}

impl Space {
    fn new() -> Self {
        Space {
            map: HashMap::new(),
        }
    }

    fn set_position(&mut self, pos: Vec3, state: bool) {
        let entry = self.map.entry(pos).or_default();
        *entry = state;
    }

    fn count_on(&self) -> usize {
        self.map.iter().filter(|(_, val)| **val).count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    cuboid: Cuboid,
    state: bool,
}

const INPUT_REGEX: &str = r"^(.*) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$";

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(INPUT_REGEX).unwrap();
        let cap = re.captures_iter(s).next().unwrap();

        Ok(Step {
            state: &cap[1] == "on",
            cuboid: Cuboid {
                // not adding the +1's here wasted me an hour :(
                x: (cap[2].parse()?, (cap[3].parse::<i128>()?) + 1),
                y: (cap[4].parse()?, (cap[5].parse::<i128>()?) + 1),
                z: (cap[6].parse()?, (cap[7].parse::<i128>()?) + 1),
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    x: (i128, i128),
    y: (i128, i128),
    z: (i128, i128),
}

impl Cuboid {
    fn get_all(&self, filter: fn(&i128) -> bool) -> Vec<Vec3> {
        let mut res = vec![];

        for x in (self.x.0..=self.x.1).filter(filter) {
            for y in (self.y.0..=self.y.1).filter(filter) {
                for z in (self.z.0..=self.z.1).filter(filter) {
                    res.push(Vec3 { x, y, z })
                }
            }
        }
        res
    }

    fn is_valid_cuboid(&self) -> bool {
        self.x.1 > self.x.0 && self.y.1 > self.y.0 && self.z.1 > self.z.0
    }

    fn volume(&self) -> i128 {
        (self.x.1 - self.x.0) * (self.y.1 - self.y.0) * (self.z.1 - self.z.0)
    }

    fn intersects(&self, o: &Self) -> bool {
        self.x.0 < o.x.1
            && self.y.0 < o.y.1
            && self.z.0 < o.z.1
            && self.x.1 > o.x.0
            && self.y.1 > o.y.0
            && self.z.1 > o.z.0
    }

    fn get_partial_axis(a: (i128, i128), b: (i128, i128), e: i128) -> (i128, i128) {
        let start_max = a.0.max(b.0);
        let end_min = a.1.min(b.1);
        match e {
            0 => (a.0, start_max),
            1 => (start_max, end_min),
            2 => (end_min, a.1),
            _ => unreachable!(),
        }
    }

    fn iter_cuboid_parts() -> impl Iterator<Item = (i128, i128, i128)> {
        let mut res: Vec<(i128, i128, i128)> = vec![];
        for x in 0..=2 {
            for y in 0..=2 {
                for z in 0..=2 {
                    // dont handle 1,1,1
                    if x != 1 || y != 1 || z != 1 {
                        res.push((x, y, z));
                    }
                }
            }
        }
        res.into_iter()
    }

    fn exclude_cuboid(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return vec![*self];
        }

        let mut res = vec![];

        for (x, y, z) in Self::iter_cuboid_parts() {
            res.push(Cuboid {
                x: Self::get_partial_axis(self.x, other.x, x),
                y: Self::get_partial_axis(self.y, other.y, y),
                z: Self::get_partial_axis(self.z, other.z, z),
            })
        }

        res.into_iter().filter(|c| c.is_valid_cuboid()).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Vec3 {
    x: i128,
    y: i128,
    z: i128,
}
