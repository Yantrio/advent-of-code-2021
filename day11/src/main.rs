use ansi_term::Colour::{Blue, White};
use ndarray::prelude::*;

const RENDER: bool = false;

fn main() {
    let input = parse_input();

    p1(input.clone());
    p2(input);
}

fn parse_input() -> Array2<Octopus> {
    let input = Array2::from_shape_vec(
        (10, 10),
        include_str!("input")
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| Octopus::new(c.to_string().parse().unwrap(), (y, x)))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<Octopus>>(),
    )
    .unwrap();
    input
}

fn p1(mut octopi: Array2<Octopus>) {
    let mut flashes = 0;
    for _ in 1..=100 {
        step(&mut octopi);
        flashes += count_flashes(&octopi);
        reset_flash_flags(&mut octopi);
    }
    println!("Part 1: {}", flashes);
}

fn p2(mut octopi: Array2<Octopus>) {
    render(&octopi);
    for i in 1..=1000 {
        step(&mut octopi);
        render(&octopi);
        if is_all_in_sync(&octopi) {
            println!("Part 2: {}", i);
            break;
        }
        reset_flash_flags(&mut octopi);
    }
}

fn is_all_in_sync(octopi: &Array2<Octopus>) -> bool {
    octopi.iter().all(|o| o.did_flash)
}

fn count_flashes(octopi: &Array2<Octopus>) -> usize {
    octopi.iter().filter(|o| o.did_flash).count()
}

fn render(octopi: &Array2<Octopus>) {
    if RENDER {
        let mut y = 0;
        for (pos, oct) in octopi.indexed_iter() {
            if y != pos.0 {
                y = pos.0;
                println!();
            }
            match oct.did_flash {
                true => print!("{}", Blue.paint(oct.value.to_string())),
                false => print!("{}", White.paint(oct.value.to_string())),
            }
        }
        println!();
        println!();
    }
}

fn get_surrounding(position: (usize, usize), arr: &Array2<T>) -> Vec<&T> {
    //left side
    let mut result = vec![];
    let (x, y) = position;

    if let [xsize, ysize] = arr.shape() {
        let on_top = y == 0;
        let on_right = x == xsize - 1;
        let on_left = x == 0;
        let on_bottom = y == ysize - 1;

        //left side
        if !on_left {
            //top
            if !on_top {
                result.push(&arr[[x - 1, y - 1]]);
            }
            //middle
            result.push(&arr[[x - 1, y]]);

            //bottom
            if !on_bottom {
                result.push(&arr[[x - 1, y + 1]]);
            }
        }

        // above
        if !on_top {
            result.push(&arr[[x, y - 1]])
        }
        if !on_bottom {
            result.push(&arr[[x, y + 1]])
        }

        //right side
        if !on_right {
            //top
            if !on_top {
                result.push(&arr[[x + 1, y - 1]]);
            }
            //middle
            result.push(&arr[[x + 1, y]]);
            //bottom
            if !on_bottom {
                result.push(&arr[[x + 1, y + 1]]);
            }
        }
    }

    result
}

fn reset_flash_flags(octopi: &mut Array2<Octopus>) {
    for o in octopi {
        o.did_flash = false;
        o.was_flashed = false;
    }
}

fn step(octopi: &mut Array2<Octopus>) {
    let mut to_flash = get_to_flash(octopi);

    let copy = octopi.clone();

    while !to_flash.is_empty() {
        for p in &to_flash {
            // set did flash flag
            let mut_oct = octopi.get_mut(*p).unwrap();
            mut_oct.did_flash = true;

            //get surrounding, increment the number
            let surrounding = get_surrounding(*p, &copy);

            for surr in surrounding {
                let mut_oct = octopi.get_mut(surr.position).unwrap();
                mut_oct.was_flashed = true;
                mut_oct.value += 1;
            }
        }

        to_flash = get_to_flash(octopi);
    }

    for o in octopi {
        o.value += 1;
        if o.value > 9 {
            o.value = 0;
        }
    }
}

fn get_to_flash(
    octopi: &ArrayBase<ndarray::OwnedRepr<Octopus>, Dim<[usize; 2]>>,
) -> Vec<(usize, usize)> {
    octopi
        .iter()
        .filter(|r| r.value >= 9 && !r.did_flash)
        .map(|o| o.position)
        .collect::<Vec<(usize, usize)>>()
}

#[derive(Debug, Copy, Clone)]
struct Octopus {
    value: usize,
    did_flash: bool,
    was_flashed: bool,
    position: (usize, usize),
}

impl Octopus {
    fn new(value: usize, position: (usize, usize)) -> Octopus {
        Octopus {
            value,
            position,
            did_flash: false,
            was_flashed: false,
        }
    }
}
