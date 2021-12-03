use bitvec::prelude::*;

fn main() {
    let input = include_str!("input");
    let inputs = input
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect::<BitVec>())
        .collect::<Vec<_>>();

    let (gamma, epsilon) = gamma_and_epsilon(&inputs);
    println!(
        "Part 1: {:#?}",
        gamma.load::<usize>() * epsilon.load::<usize>()
    );

    let oxygen = filter_away(&inputs, true).load::<usize>();
    let co2 = filter_away(&inputs, false).load::<usize>();
    println!("Part 2: {:#?}", oxygen * co2)
}

fn filter_away(inputs: &[BitVec], co2: bool) -> BitVec {
    let mut filtered = inputs.to_owned();
    let mut idx = 0;

    while !filtered.is_empty() {
        let mut most_common = most_common_val_for_bit(&filtered, idx);
        if co2 {
            most_common = !most_common
        }

        filtered = filtered
            .iter()
            .filter(|i| i[idx] == most_common)
            .cloned()
            .collect::<Vec<_>>();

        // cant figure out bitvecs and msb vs lsb, nothing works, so just reverse the bits
        if filtered.len() == 1 {
            filtered[0].reverse();
            return filtered[0].clone();
        }
        idx += 1;
    }
    panic!() // Shouldn't reach here!
}

fn gamma_and_epsilon(inputs: &[BitVec]) -> (BitVec, BitVec) {
    let mut gamma = bitvec![];
    let mut epsilon = bitvec![];
    for idx in 0..inputs[0].len() {
        // count the 1s, count the zeros
        let most_common = most_common_val_for_bit(inputs, idx);
        gamma.push(most_common);
        epsilon.push(!most_common);
    }
    // cant figure out bitvecs and msb vs lsb, nothing works, so just reverse the bits
    gamma.reverse();
    epsilon.reverse();
    (gamma, epsilon)
}

fn most_common_val_for_bit(inputs: &[BitVec], idx: usize) -> bool {
    inputs.iter().map(|i| i[idx]).filter(|&i| i).count() * 2 >= inputs.len()
}
