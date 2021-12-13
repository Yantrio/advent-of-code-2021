use ndarray::prelude::*;

fn main() {
    let lines = include_str!("input").lines().collect::<Vec<_>>();

    let dots = lines
        .clone()
        .iter()
        .take_while(|l| !l.trim().is_empty())
        .map(|&l| Position::from_str(l))
        .collect::<Vec<_>>();

    let folds = lines
        .clone()
        .iter()
        .skip_while(|l| !l.trim().is_empty())
        .skip(1)
        .map(|&l| Fold::from_str(l))
        .collect::<Vec<_>>();

    let max_x = dots.iter().map(|d| d.x).max().unwrap();
    let max_y = dots.iter().map(|d| d.y).max().unwrap();

    let mut paper = Array2::from_elem((max_x + 1, max_y + 1), false);

    for d in dots {
        paper[[d.x, d.y]] = true;
    }

    println!("Part 1: {}", part1(&paper, &folds));
    println!("Part 2:");
    part2(&paper, &folds);

    // println!("dots: {:?}", dots);
    // println!("folds: {:?}", folds);
}

fn part1(paper: &Array2<bool>, folds: &[Fold]) -> usize {
    fold_paper(paper, &folds[0]).iter().filter(|&&d| d).count()
}

fn part2(paper: &Array2<bool>, folds: &[Fold]) {
    let mut current = paper.clone();
    for f in folds {
        current = fold_paper(&current, f);
    }
    render(&current);
}

fn fold_paper(paper: &Array2<bool>, fold: &Fold) -> Array2<bool> {
    let mut folded = get_folded_paper(fold, paper);

    for p in paper
        .indexed_iter()
        .filter(|(_, d)| **d)
        .map(|((y, x), _)| Position::new(x, y))
        .map(|p| p.get_folded_position(fold))
    {
        folded[[p.y, p.x]] = true;
    }

    folded
}

fn get_folded_paper(fold: &Fold, paper: &Array2<bool>) -> Array2<bool> {
    match fold.y_axis {
        true => Array2::from_elem((fold.position, paper.shape()[1]), false),
        false => Array2::from_elem((paper.shape()[0], fold.position), false),
    }
}

fn render(paper: &Array2<bool>) {
    let mut y = 0;
    for (pos, dot) in paper.indexed_iter() {
        if y != pos.0 {
            y = pos.0;
            println!();
        }
        print!(
            "{}",
            match dot {
                true => '#',
                false => '.',
            }
        );
    }
    println!();
    println!();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Fold {
    x_axis: bool,
    y_axis: bool,
    position: usize,
}

impl Fold {
    fn from_str(string: &str) -> Fold {
        let spl = string
            .split("fold along ")
            .nth(1)
            .unwrap()
            .split('=')
            .collect::<Vec<_>>();
        let axis = spl.get(0).unwrap();
        let pos = spl.get(1).unwrap().parse::<usize>().unwrap();
        Fold {
            x_axis: *axis == "x",
            y_axis: *axis == "y",
            position: pos,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn from_str(string: &str) -> Position {
        let mut spl = string.split(',').collect::<Vec<_>>();
        Position {
            x: spl.pop().unwrap().parse().unwrap(),
            y: spl.pop().unwrap().parse().unwrap(),
        }
    }

    fn get_folded_position(&self, fold: &Fold) -> Position {
        match fold.y_axis {
            true => match self.y < fold.position {
                true => Position::new(self.x, self.y),
                false => Position::new(self.x, self.y - (self.y - fold.position) * 2),
            },
            false => match self.x < fold.position {
                true => Position::new(self.x, self.y),
                false => Position::new(self.x - (self.x - fold.position) * 2, self.y),
            },
        }
    }

    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}
