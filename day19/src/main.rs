#![feature(int_abs_diff)]
use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
    str::FromStr,
};

use itertools::Itertools;

fn main() {
    let mut scans = include_str!("input")
        .split("\n\n")
        .map(|s| s.lines().skip(1).join("\n"))
        .map(|s| s.parse::<Probe>().unwrap())
        .collect::<VecDeque<_>>();

    let mut first_set = scans
        .pop_front()
        .unwrap()
        .0
        .into_iter()
        .collect::<HashSet<_>>();

    let mut distances = vec![];
    while !scans.is_empty() {
        println!("Scanners remaining: {}", scans.len());
        for idx in (0..(scans.len())).rev() {
            if let Some(distance) = merge_all_scans(&mut first_set, &scans[idx]) {
                distances.push(distance);
                scans.remove(idx);
            }
        }
    }

    println!("Part 1: {:?}", first_set.len());

    let max_dist = distances
        .iter()
        .permutations(2)
        .map(|a| a[0].manhattan_distance_from(a[1]))
        .max();

    println!("Part 2: {:?}", max_dist.unwrap());
}

/*

for each probe's scan set:
Check all 24 rotations, check the distances between all points
check overlapping points == 12, if it is, add the probe's scansets
*/

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vec3(isize, isize, isize);

impl FromStr for Vec3 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl = s.split(',').collect::<Vec<_>>();
        Ok(Vec3(spl[0].parse()?, spl[1].parse()?, spl[2].parse()?))
    }
}

impl Vec3 {
    fn offset_by(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn distance_from(&self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    fn manhattan_distance_from(&self, other: &Vec3) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2)
    }
}

impl Vec3 {
    // shamelessly stolen from someone on github a couple years back
    fn rotate(&self, rot: u8) -> Vec3 {
        let (x, y, z) = (self.0, self.1, self.2);
        match rot {
            0 => Vec3(x, y, z),
            1 => Vec3(y, -x, z),
            2 => Vec3(-x, -y, z),
            3 => Vec3(-y, x, z),
            4 => Vec3(z, y, -x),
            5 => Vec3(y, -z, -x),
            6 => Vec3(-z, -y, -x),
            7 => Vec3(-y, z, -x),
            8 => Vec3(z, -x, -y),
            9 => Vec3(-x, -z, -y),
            10 => Vec3(-z, x, -y),
            11 => Vec3(x, z, -y),
            12 => Vec3(z, -y, x),
            13 => Vec3(-y, -z, x),
            14 => Vec3(-z, y, x),
            15 => Vec3(y, z, x),
            16 => Vec3(z, x, y),
            17 => Vec3(x, -z, y),
            18 => Vec3(-z, -x, y),
            19 => Vec3(-x, z, y),
            20 => Vec3(-x, y, -z),
            21 => Vec3(y, x, -z),
            22 => Vec3(x, -y, -z),
            23 => Vec3(-y, -x, -z),
            _ => unreachable!(),
        }
    }
}

// returns the distance between the two if it was found
fn merge_all_scans(result: &mut HashSet<Vec3>, probe: &Probe) -> Option<Vec3> {
    for (_, rotation_set) in probe.get_all_rotations() {
        let mut resclone = result.clone();
        let offsets = get_offsets(&mut resclone, &rotation_set);

        for offset in offsets {
            let translated = rotation_set.iter().map(|pos| pos.offset_by(&offset));
            let found_points = translated.clone().filter(|trans| result.contains(trans));
            if found_points.count() >= 12 {
                // we have it overlapped! :D
                result.extend(translated);
                return Some(offset);
            }
        }
    }
    None
}

fn get_offsets<'a>(
    result: &'a mut HashSet<Vec3>,
    rotation_set: &'a [Vec3],
) -> impl Iterator<Item = Vec3> + 'a {
    result
        .iter()
        .cartesian_product(rotation_set)
        .map(|(a, b)| a.distance_from(b))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Probe(Vec<Vec3>);

impl Probe {
    fn get_all_rotations(&self) -> impl Iterator<Item = (isize, Vec<Vec3>)> + '_ {
        (0..24).map(|r| {
            (
                r as isize,
                self.0.iter().map(|s| s.rotate(r)).collect::<Vec<_>>(),
            )
        })
    }
}

impl FromStr for Probe {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Probe(
            s.lines()
                .map(|l| l.parse::<Vec3>().unwrap())
                .collect::<Vec<_>>(),
        ))
    }
}
