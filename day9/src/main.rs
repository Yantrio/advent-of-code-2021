use std::io;
use std::io::prelude::*;

use ansi_term::Colour::{Blue, White};
use ndarray::prelude::*;

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let map = Array2::from_shape_vec(
        (input.len(), input[0].len()),
        input.into_iter().flatten().collect::<Vec<usize>>(),
    )
    .unwrap();

    let low = get_low_points(&map);
    println!(
        "Part 1: {:?}",
        low.iter().map(|(_, &l)| l + 1).sum::<usize>()
    );

    let mut used = vec![];
    let mut basins = vec![];

    for lp in &low {
        let mut basin = get_basin(lp.0 .0, lp.0 .1, &map, &mut used.clone());
        basins.push(basin.clone());
        used.append(&mut basin);
    }

    let mut sizes = basins.iter().map(|b| b.len()).collect::<Vec<_>>();
    sizes.sort_unstable();
    println!("basin sizes: {:?}", sizes);

    let mut stdin = io::stdin();

    viz(
        &basins.clone().into_iter().flatten().collect::<Vec<_>>(),
        &map,
    );
}

fn viz(basin: &Vec<Point>, map: &Array2<usize>) {
    let reversed = map.clone();
    let mut y = 0;
    for item in reversed.indexed_iter() {
        if y != item.0 .0 {
            y = item.0 .0;
            println!();
        }
        match already_used(item.0 .0, item.0 .1, &mut basin.clone()) {
            true => print!("{}", Blue.paint(item.1.to_string())),
            false => print!("{}", White.paint(item.1.to_string())),
        }
    }
    println!();
}

type Point = ((usize, usize), usize);

fn already_used(x: usize, y: usize, used: &mut Vec<Point>) -> bool {
    used.iter().any(|((x2, y2), _)| *x2 == x && *y2 == y)
}

fn get_basin(x: usize, y: usize, map: &Array2<usize>, used: &mut Vec<Point>) -> Vec<Point> {
    let current = map[[x, y]];
    let surrounding = get_surrounding(x, y, map);
    let to_check = surrounding
        .iter()
        .filter(|(_, v)| *v != 9)
        .filter(|(_, v)| *v >= current)
        .collect::<Vec<_>>();

    used.push(((x, y), current));

    let mut found = vec![];
    for p in to_check {
        if !already_used(p.0 .0, p.0 .1, used) {
            let b = get_basin(p.0 .0, p.0 .1, map, used);
            used.append(&mut b.clone());
            found.append(&mut b.clone());
        }
    }

    found.push(((x, y), current));
    found.sort_unstable();
    found.dedup();
    found
}

fn get_low_points(map: &Array2<usize>) -> Vec<((usize, usize), &usize)> {
    let mut result = vec![];
    for ((x, y), v) in map.indexed_iter() {
        if get_surrounding(x, y, map)
            .iter()
            .all(|(_, mapval)| mapval > v)
        {
            result.push(((x, y), v));
        }
    }
    result
}

fn get_surrounding(x: usize, y: usize, map: &Array2<usize>) -> Vec<Point> {
    let mut result = vec![];

    if x != 0 {
        // left
        result.push(((x - 1, y), map[[x - 1, y]]))
    }
    if y != 0 {
        // up
        result.push(((x, y - 1), map[[x, y - 1]]))
    }
    if x != map.shape()[0] - 1 {
        // right
        result.push(((x + 1, y), map[[x + 1, y]]))
    }
    if y != map.shape()[1] - 1 {
        // right
        result.push(((x, y + 1), map[[x, y + 1]]))
    }

    result
}
