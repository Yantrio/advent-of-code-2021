fn main() {
    // let input = "16,1,2,0,4,2,7,1,2,14";
    let input = include_str!("input");

    let positions = input
        .split(',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let max = positions.iter().max().unwrap();
    let min = positions.iter().min().unwrap();

    let calc = |triangle| {
        (*min..*max)
            .into_iter()
            .map(|p| calc_total_fuel_cost(&positions, &p, triangle))
            .min()
            .unwrap()
    };

    println!("Part 1: {}", calc(false));
    println!("Part 2: {}", calc(true));
}

fn calc_total_fuel_cost(positions: &[isize], dest: &isize, use_triangle: bool) -> isize {
    positions
        .iter()
        .map(|p| (p - dest).abs())
        .map(|p| match use_triangle {
            false => p,
            true => triangle(p),
        })
        .sum()
}

fn triangle(x: isize) -> isize {
    (x * (x + 1)) / 2
}
