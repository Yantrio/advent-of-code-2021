use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let combos = input.lines().map(Display::new).collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        combos.iter().map(Display::count_1478).sum::<usize>()
    );

    println!(
        "Part 2: {}",
        combos
            .iter()
            .map(|c| c.decode_display().unwrap())
            .sum::<usize>()
    );
}

#[derive(Debug, Clone)]
struct Display {
    inputs: Vec<Vec<char>>,
    outputs: Vec<Vec<char>>,
}

impl Display {
    fn count_1478(&self) -> usize {
        self.outputs
            .iter()
            .map(Vec::len)
            .filter(|&len| len == 2 || len == 3 || len == 4 || len == 7)
            .count()
    }

    fn new(s: &str) -> Display {
        let split = s.split(" | ").collect::<Vec<_>>();

        let parse = |s: &str| {
            s.trim()
                .split(' ')
                .map(|spl| spl.trim().chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<_>>>()
        };
        Display {
            inputs: parse(split[0]),
            outputs: parse(split[1]),
        }
    }

    fn get_valid_combo(&self) -> Option<HashMap<char, char>> {
        "abcdefg"
            .chars()
            .permutations(7)
            .map(|perm| {
                perm.into_iter()
                    .zip("abcdefg".chars())
                    .collect::<HashMap<_, _>>()
            })
            .find(|perm| self.is_valid(perm))
    }

    fn is_valid(&self, map: &HashMap<char, char>) -> bool {
        self.inputs.iter().all(|segment| {
            segment
                .iter()
                .map(|c| map.get(c))
                .collect::<Option<String>>()
                .map(|i| i.chars().collect::<Vec<char>>())
                .and_then(decode)
                .is_some()
        })
    }

    fn decode_display(&self) -> Option<usize> {
        let valid_combo = self.get_valid_combo()?;
        let decoded = self
            .outputs
            .iter()
            .map(|segs| {
                segs.iter()
                    .flat_map(|s| valid_combo.get(s))
                    .copied()
                    .collect::<Vec<char>>()
            })
            .map(decode)
            .collect::<Option<Vec<_>>>()?;

        Some(digits_to_num(decoded))
    }
}

fn digits_to_num(input: Vec<usize>) -> usize {
    input.into_iter().fold(0, |acc, x| acc * 10 + x)
}

fn decode(segments: Vec<char>) -> Option<usize> {
    let mut segments = segments; // copy
    segments.sort_unstable();

    let s = segments.into_iter().collect::<String>();
    match s.as_str() {
        "abcefg" => Some(0),
        "cf" => Some(1),
        "acdeg" => Some(2),
        "acdfg" => Some(3),
        "bcdf" => Some(4),
        "abdfg" => Some(5),
        "abdefg" => Some(6),
        "acf" => Some(7),
        "abcdefg" => Some(8),
        "abcdfg" => Some(9),
        _ => None,
    }
}
