use std::{collections::VecDeque, fmt};

fn main() {
    let snails = include_str!("input")
        .lines()
        .map(|l| parse(&mut l.chars()))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&snails));
    println!("Part 2: {}", part2(&snails));
}

fn part1(all: &[Element]) -> usize {
    let mut current = all[0].clone();
    for a in all.iter().skip(1) {
        current = Element::new_pair(current, a.clone());
        current.reduce();
    }
    current.magnitude()
}

fn part2(all: &[Element]) -> usize {
    let mut max_mag = 0;
    for right in all.iter() {
        for left in all.iter().filter(|&l| l != right) {
            let res = Element::new_pair(left.clone(), right.clone())
                .reduce()
                .magnitude();
            if res > max_mag {
                max_mag = res;
            }
        }
    }
    max_mag
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Element {
    Number(usize),
    Pair(Box<(Element, Element)>),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(v) => f.write_fmt(format_args!("{}", v)),
            Self::Pair(p) => f.write_fmt(format_args!("[{:?},{:?}]", p.0, p.1)),
        }
    }
}

fn parse<T>(s: &mut T) -> Element
where
    T: Iterator<Item = char>,
{
    let mut stack = VecDeque::new();
    while let Some(c) = s.next() {
        match c {
            '[' => stack.push_back(parse(s)),
            c @ '0'..='9' => stack.push_back(Element::Number(c.to_digit(10).unwrap() as usize)),
            ',' => {}
            ']' => {
                return Element::new_pair(stack.pop_front().unwrap(), stack.pop_front().unwrap())
            }
            _ => unreachable!(),
        }
    }
    stack.pop_front().unwrap()
}

impl Element {
    fn new_pair(l: Element, r: Element) -> Element {
        Element::Pair(Box::new((l, r)))
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Number(val) => *val,
            Element::Pair(pair) => pair.0.magnitude() * 3 + pair.1.magnitude() * 2,
        }
    }

    fn reduce(&mut self) -> &Self {
        while self.explode(0).0 || self.split() {}
        self
    }

    fn explode(&mut self, depth: isize) -> (bool, Option<usize>, Option<usize>) {
        match self {
            // Can't explode if just a number
            Element::Number(_) => (false, None, None),
            Element::Pair(pair) => {
                // if both are numbers
                if let Element::Number(l) = pair.0 {
                    if let Element::Number(r) = pair.1 {
                        if depth >= 4 {
                            // i hate mutating *self :(
                            *self = Element::Number(0);
                            return (true, Some(l), Some(r));
                        }
                    }
                }
                // try and explode the left
                if let Some(value) = explode_children(pair, depth) {
                    return value;
                }
                (false, None, None)
            }
        }
    }

    fn set_left(&mut self, num: usize) {
        match self {
            Element::Number(ref mut l) => *l += num,
            // recurse down all the way left
            Element::Pair(p) => p.0.set_left(num),
        }
    }

    fn set_right(&mut self, num: usize) {
        match self {
            Element::Number(ref mut r) => *r += num,
            // recurse down all the way right
            Element::Pair(p) => p.1.set_right(num),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            // recurse down
            Element::Pair(pair) => pair.0.split() || pair.1.split(),
            Element::Number(value) => match *value >= 10 {
                true => {
                    *self = Element::new_pair(
                        Element::Number((*value as f64 / 2.0).floor() as usize),
                        Element::Number((*value as f64 / 2.0).ceil() as usize),
                    );
                    true
                }

                false => false,
            },
        }
    }
}

fn explode_children(
    pair: &mut Box<(Element, Element)>,
    depth: isize,
) -> Option<(bool, Option<usize>, Option<usize>)> {
    // left
    let (did_explode, left, mut right) = pair.0.explode(depth + 1);
    if did_explode {
        if right.is_some() {
            pair.1.set_left(right.unwrap());
            right = None;
        }
        return Some((true, left, right));
    }
    // right
    let (did_explode, mut left, right) = pair.1.explode(depth + 1);
    if did_explode {
        if left.is_some() {
            pair.0.set_right(left.unwrap());
            left = None;
        }
        return Some((true, left, right));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_reduce_example_1() {
        let input = "[[[[[9,8],1],2],3],4]";
        let expected = parse(&mut "[[[[0,9],2],3],4]".chars());
        single_explode_test(input, expected);
    }

    #[test]
    fn simple_reduce_example_2() {
        let input = "[7,[6,[5,[4,[3,2]]]]]";
        let expected = parse(&mut "[7,[6,[5,[7,0]]]]".chars());
        single_explode_test(input, expected);
    }

    #[test]
    fn simple_reduce_example_3() {
        let input = "[[6,[5,[4,[3,2]]]],1]";
        let expected = parse(&mut "[[6,[5,[7,0]]],3]".chars());
        single_explode_test(input, expected);
    }

    #[test]
    fn simple_reduce_example_4() {
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let expected = parse(&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars());
        single_explode_test(input, expected);
    }

    #[test]
    fn simple_reduce_example_5() {
        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let expected = parse(&mut "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars());
        single_explode_test(input, expected);
    }

    fn single_explode_test(input: &str, expected: Element) {
        let mut parsed = parse(&mut input.chars());
        parsed.explode(0);
        assert_eq!(parsed, expected);
    }
}
