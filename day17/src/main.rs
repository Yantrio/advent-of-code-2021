use std::ops::RangeInclusive;

fn main() {
    let target = TargetArea {
        xrange: 288..=330,
        yrange: -96..=-50,
    };

    let (max_y, total_in_target) = solve(target);

    println!("Part 1: {}", max_y);
    println!("Part 2: {}", total_in_target);
}

fn solve(target: TargetArea) -> (isize, i32) {
    let mut max_y = isize::MIN;
    let mut total_in_target = 0;
    for x in 1..=*target.xrange.end() {
        for y in -100..100 {
            if let Some(max) = attempt(x, y, &target) {
                total_in_target += 1;
                if max > max_y {
                    max_y = max;
                }
            }
        }
    }
    (max_y, total_in_target)
}

fn attempt(x: isize, y: isize, target: &TargetArea) -> Option<isize> {
    let mut p = Probe::new(x, y);

    if p.step_until_in_range(target, 10000).is_some() {
        return Some(p.max_y);
    }

    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TargetArea {
    xrange: RangeInclusive<isize>,
    yrange: RangeInclusive<isize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Probe {
    original_vel: Vec2,
    vel: Vec2,
    pos: Vec2,
    max_y: isize,
}

impl Probe {
    fn new(xvel: isize, yvel: isize) -> Probe {
        Probe {
            original_vel: Vec2 { x: xvel, y: yvel },
            vel: Vec2 { x: xvel, y: yvel },
            pos: Vec2 { x: 0, y: 0 },
            max_y: isize::MIN,
        }
    }

    fn step(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        if self.pos.y > self.max_y {
            self.max_y = self.pos.y
        }

        if self.vel.x != 0 {
            self.vel.x -= 1;
        }
        self.vel.y -= 1;
    }

    fn step_until_in_range(&mut self, target: &TargetArea, max_iters: usize) -> Option<usize> {
        for s in 1..max_iters {
            self.step();

            if self.in_target_area(target) {
                return Some(s);
            }
            if self.missed_target_area(target) {
                return None;
            }
        }

        None
    }

    fn missed_target_area(&self, target: &TargetArea) -> bool {
        self.pos.x > *target.xrange.end() && self.pos.y < *target.xrange.start()
    }
    fn in_target_area(&self, target: &TargetArea) -> bool {
        target.xrange.contains(&self.pos.x) && target.yrange.contains(&self.pos.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vec2 {
    x: isize,
    y: isize,
}
