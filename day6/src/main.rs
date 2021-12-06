fn main() {
    let mut fish: [u64; 9] = include_str!("input")
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .fold([0; 9], |mut acc, val| {
            acc[val] += 1;
            acc
        });

    for d in 0..=256 {
        if d == 80 || d == 256 {
            println!("Day {}, Fish: {:?}", d, fish.iter().sum::<u64>());
        }
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
}
