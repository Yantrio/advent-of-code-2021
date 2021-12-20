use ansi_term::Colour::{Black, Blue, White};
use ndarray::prelude::*;
use std::str::Lines;

fn main() {
    let lines = include_str!("input").lines();
    let (mut map, algo) = parse_input(lines);

    // hacky but it works
    for _ in 0..120 {
        map = expand(&map);
    }

    map = step(&map, &algo);
    map = step(&map, &algo);
    println!("Part 1: {}", count_lit(&map));

    for _ in 0..48 {
        map = step(&map, &algo);
        // print!("{}[2J", 27 as char);
        // render(&map);
    }

    println!("Part 2: {}", count_lit(&map))
}
fn parse_input(mut lines: Lines) -> (Array2<(bool, u8)>, Vec<u8>) {
    let algorithm = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c == '#' {
            true => 1,
            false => 0,
        })
        .collect::<Vec<_>>();

    lines.next();

    let map_raw = lines
        .map(|l| {
            l.chars()
                .map(|c| match c == '#' {
                    true => (true, 1),
                    false => (true, 0),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let shape = (100, 100);
    (Array2::from_shape_vec(shape, map_raw).unwrap(), algorithm)
}

fn count_lit(map: &Array2<(bool, u8)>) -> usize {
    map.iter()
        .filter(|pix| pix.0)
        .filter(|&&pix| pix.1 == 1)
        .count()
}

fn step(map: &Array2<(bool, u8)>, algorithm: &[u8]) -> Array2<(bool, u8)> {
    let mut result = expand(map);
    let reference = result.clone();

    for ((y, x), p) in map.indexed_iter() {
        let surrounding = &get_surrounding(y + 1, x + 1, &reference);

        result[[y + 1, x + 1]] = (
            surrounding.iter().any(|s| s.0),
            algorithm[to_decimal(surrounding)],
        );
    }

    result
}

fn render(map: &Array2<(bool, u8)>) {
    let mut y = 0;
    for (pos, pixel) in map.indexed_iter() {
        if y != pos.0 {
            y = pos.0;
            println!();
        }
        match pixel.1 {
            1 => match pixel.0 {
                true => print!("{}", Blue.paint("#")),
                false => print!("{}", Black.paint("#")),
            },
            0 => match pixel.0 {
                true => print!("{}", White.paint(".")),
                false => print!("{}", Black.paint(".")),
            },
            _ => unreachable!(),
        }
    }
    println!();
    println!("---");
}

fn to_decimal(bits: &[(bool, u8)]) -> usize {
    let mut result: usize = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit.1 as usize;
    });
    result
}

fn get_surrounding(y: usize, x: usize, map: &Array2<(bool, u8)>) -> Vec<(bool, u8)> {
    vec![
        // top
        map[[y - 1, x - 1]],
        map[[y - 1, x]],
        map[[y - 1, x + 1]],
        // middle
        map[[y, x - 1]],
        map[[y, x]],
        map[[y, x + 1]],
        // botttom
        map[[y + 1, x - 1]],
        map[[y + 1, x]],
        map[[y + 1, x + 1]],
    ]
}

fn expand(map: &Array2<(bool, u8)>) -> Array2<(bool, u8)> {
    let current_shape = map.shape();
    let mut res = Array2::from_elem((current_shape[0] + 2, current_shape[1] + 2), (false, 0));

    for ((x, y), pixel) in map.indexed_iter() {
        res[[x + 1, y + 1]] = *pixel;
    }

    res
}
