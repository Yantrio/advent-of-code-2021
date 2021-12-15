use pathfinding::prelude::*;

const SHAPE: (i32, i32) = (100, 100);
fn main() {
    let map = parse_input();

    println!("Part 1: {}", solve(&map, (100, 100)));
    println!("Part 2: {}", solve(&map, (500, 500)));
}

fn solve(map: &[(Pos, usize)], max_shape: (i32, i32)) -> usize {
    let start = Pos(0, 0);
    let end = Pos(max_shape.0 - 1, max_shape.1 - 1);
    dijkstra(&start, |p| p.successors(map, max_shape), |p| *p == end)
        .unwrap()
        .1
}

fn parse_input() -> Vec<(Pos, usize)> {
    include_str!("input")
        .lines()
        .enumerate()
        .map(|(y, l)| {
            (
                y,
                l.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .enumerate()
                    .map(|(x, val)| (Pos(x as i32, y as i32), val))
                    .collect::<Vec<(Pos, usize)>>(),
            )
        })
        .map(|(_, val)| val)
        .flatten()
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, map: &[(Pos, usize)], shape: (i32, i32)) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        vec![Pos(x, y - 1), Pos(x + 1, y), Pos(x, y + 1), Pos(x - 1, y)]
            .into_iter()
            .filter(|p| p.0 >= 0 && p.0 < shape.0)
            .filter(|p| p.1 >= 0 && p.1 < shape.1)
            .map(|p| (p, get_val_at_pos(map, p)))
            .collect()
    }
}

fn get_val_at_pos(map: &[(Pos, usize)], pos: Pos) -> usize {
    map.iter()
        // TODO: FIX MODULO
        .find(|(p, _)| *p == Pos(pos.0 % SHAPE.0, pos.1 % SHAPE.1))
        .map(|(_, val)| *val + ((pos.0 / SHAPE.1) as usize) + ((pos.1 / SHAPE.1) as usize))
        .map(|v| match v > 9 {
            true => v % 9,
            false => v,
        })
        .unwrap()
}
