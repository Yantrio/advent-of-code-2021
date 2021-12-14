use std::{collections::HashMap, str::Lines};

type PairMap = HashMap<String, usize>;
type CharMap = HashMap<char, usize>;
type RuleMap = HashMap<String, char>;
type Rule = (String, char);

fn main() {
    let (polymer, rules) = parse_input(include_str!("input"));

    let counted_pairs = get_counted_pairs(&polymer);
    let counted_chars = get_counted_chars(polymer);

    let mut result = (counted_pairs, counted_chars);
    for i in 0..40 {
        result = step(result.0, result.1, &rules);
        println!("Step {}, Score: {}", i, score(&result.1))
    }
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, char>) {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().chars().collect::<Vec<_>>();
    let rules: RuleMap = lines.skip(1).map(get_rule).collect();
    (polymer, rules)
}

fn score(charmap: &HashMap<char, usize>) -> usize {
    charmap.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
        - charmap.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1
}

fn get_counted_chars(polymer: Vec<char>) -> HashMap<char, usize> {
    polymer
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, c| {
            let count = acc.entry(*c).or_default();
            *count += 1;
            acc
        })
}

fn get_counted_pairs(polymer: &[char]) -> HashMap<String, usize> {
    polymer
        .windows(2)
        .fold(HashMap::<String, usize>::new(), |mut acc, win| {
            let count = acc.entry(win.iter().collect::<String>()).or_default();
            *count += 1;
            acc
        })
}

fn get_rule(line: &str) -> Rule {
    let spl = line.split(" -> ").collect::<Vec<_>>();
    (spl[0].to_string(), spl[1].chars().next().unwrap())
}

fn step(pairmap: PairMap, charmap: CharMap, rules: &RuleMap) -> (PairMap, CharMap) {
    let mut res_pairs = pairmap.clone();
    let mut res_chars = charmap;

    for entry in pairmap {
        let pair = entry.0.clone();
        let mut chars = pair.chars();
        let to_insert = *rules.get(&entry.0).unwrap();

        // old pair down
        *res_pairs.entry(entry.0).or_default() -= entry.1;

        // new pairs up!
        increment_pair(
            &mut res_pairs,
            [chars.next().unwrap(), to_insert]
                .iter()
                .collect::<String>(),
            entry.1,
        );
        increment_pair(
            &mut res_pairs,
            [to_insert, chars.next().unwrap()]
                .iter()
                .collect::<String>(),
            entry.1,
        );

        *res_chars.entry(to_insert).or_default() += entry.1;
    }

    (res_pairs, res_chars)
}

fn increment_pair(pairs: &mut HashMap<String, usize>, pair: String, pair_count: usize) {
    *pairs.entry(pair).or_default() += pair_count;
}
