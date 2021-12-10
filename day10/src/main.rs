fn main() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", input.iter().map(get_score_1).sum::<usize>());

    let mut scores = input.iter().map(get_score_2).flatten().collect::<Vec<_>>();

    scores.sort_unstable();

    println!("Part 2: {:?}", scores[scores.len() / 2]);
}

fn get_score_1(chars: &Vec<char>) -> usize {
    let mut stack = vec![];
    let mut score = 0;
    for c in chars {
        match *c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            other => {
                if Some(other) != stack.pop() {
                    score += map_score(other)
                }
            }
        }
    }

    score
}

fn get_score_2(chars: &Vec<char>) -> Option<usize> {
    let mut stack = vec![];
    let mut score = 0;
    for c in chars {
        match *c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            other => {
                if Some(other) != stack.pop() {
                    return None;
                }
                continue;
            }
        }
    }
    while let Some(c) = stack.pop() {
        score *= 5;
        score += map_score_2(c);
    }

    Some(score)
}

fn map_score(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn map_score_2(ch: char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}
