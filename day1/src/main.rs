fn main() {
    let input = include_str!("input");
    let depths: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!("Part 1: {}", find_result_1(&depths));
    println!("Part 2: {}", find_result_2(&depths));
}

fn find_result_1(depths: &[usize]) -> usize {
    count_increments(depths)
}

fn find_result_2(depths: &[usize]) -> usize {
    let windows = depths
        .windows(3)
        .map(|win| win.iter().sum::<usize>())
        .collect::<Vec<_>>();

    count_increments(&windows)
}

fn count_increments(input: &[usize]) -> usize {
    input.windows(2).filter(|win| win[0] < win[1]).count()
}

#[cfg(test)]
mod day1 {
    use super::find_result_1;
    use super::find_result_2;

    #[test]
    fn part1() {
        let vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = find_result_1(&vec);
        assert_eq!(7, result);
    }

    #[test]
    fn part2() {
        let vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = find_result_2(&vec);
        assert_eq!(5, result)
    }
}
