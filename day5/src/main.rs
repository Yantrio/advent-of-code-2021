use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

use bmp::{px, Image, Pixel};
use ndarray::Array2;

fn main() {
    let lines = include_str!("input")
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .collect::<Vec<_>>();

    let mut map = Map::new(1000, 1000);

    for l in lines.iter().filter(|l| l.is_hori_or_vert()) {
        map.add_line(l);
    }
    println!(
        "part 1: {:#?}",
        map.map.iter().filter(|&&coord| coord >= 2).count()
    );

    for l in lines.iter().filter(|l| !l.is_hori_or_vert()) {
        map.add_line(l);
    }
    println!(
        "part 2: {:#?}",
        map.map.iter().filter(|&&coord| coord >= 2).count()
    );

    // map.render_to_bmp();
}

struct Map {
    map: Array2<isize>,
}

impl Map {
    fn new(x: usize, y: usize) -> Map {
        Map {
            map: Array2::<isize>::zeros((x, y)),
        }
    }

    fn add_line(&mut self, line: &Line) {
        if line.is_hori_or_vert() {
            // iterate the line
            for xpos in create_range(line.start.x, line.end.x) {
                for ypos in create_range(line.start.y, line.end.y) {
                    self.map[[xpos as usize, ypos as usize]] += 1;
                }
            }
        } else {
            // if its diagonal, we need to find which direction
            let (xdir, ydir) = line.get_direction();
            let mut current = line.start;
            while current != line.end {
                self.map[[current.x as usize, current.y as usize]] += 1;
                current.x += xdir;
                current.y += ydir;
            }
            self.map[[current.x as usize, current.y as usize]] += 1;
        }
    }

    fn render(&mut self) {
        for y in 0..self.map.shape()[0] {
            for x in 0..self.map.shape()[1] {
                let val = self.map[[x as usize, y as usize]];
                if val == 0 {
                    print!(".")
                } else {
                    print!("{}", val)
                }
            }
            println!();
        }
    }

    fn render_to_bmp(&mut self) {
        let mut img = Image::new(1000, 1000);

        for ((x, y), val) in self.map.indexed_iter() {
            img.set_pixel(
                x as u32,
                y as u32,
                match val {
                    0 => px!(255, 255, 255),
                    1 => px!(255, 0, 0),
                    2 => px!(0, 255, 255),
                    3 => px!(0, 0, 255),
                    4 => px!(0, 255, 0),
                    5 => px!(255, 0, 255),
                    _ => px!(0, 0, 0),
                },
            )
        }
        img.save("output.bmp").unwrap();
    }
}

fn create_range(start: isize, end: isize) -> RangeInclusive<isize> {
    match start > end {
        true => end..=start,
        false => start..=end,
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl FromStr for Vec2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        Ok(Vec2 {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    fn is_hori_or_vert(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn get_direction(&self) -> (isize, isize) {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        (x / x.abs(), y / y.abs())
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ").collect::<Vec<_>>();
        Ok(Line {
            start: parts[0].parse()?,
            end: parts[1].parse()?,
        })
    }
}
