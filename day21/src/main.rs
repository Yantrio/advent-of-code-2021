use cached::proc_macro::cached;

const FREQ_TABLE: [(i128, i128); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn main() {
    // cba parsing input for 2 ints, humans are faster!
    let (pos1, pos2) = (7, 10);

    // Part 1
    let mut p1 = Player::new(pos1);
    let mut p2 = Player::new(pos2);
    let rolls = play_part_1(&mut p1, &mut p2);

    println!("Part 1: {}", p1.score.min(p2.score) * rolls);

    // Part 2
    let (p1wins, p2wins) = play_part_2(Player::new(pos1), Player::new(pos2));

    println!("Part 2: {:?}", p1wins.max(p2wins))
}

#[cached]
fn play_part_2(p1: Player, p2: Player) -> (i128, i128) {
    if p2.score >= 21 {
        return (0, 1);
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for (roll, freq) in FREQ_TABLE.into_iter() {
        let mut clone = p1;
        clone.apply_rolls(&[roll]);

        // flip them and also use the cached recursion result
        let (p2_wins_in_multiverse, p1_wins_in_multiverse) = play_part_2(p2, clone);
        p1_wins += p1_wins_in_multiverse * freq;
        p2_wins += p2_wins_in_multiverse * freq;
    }

    (p1_wins, p2_wins)
}

fn play_part_1(p1: &mut Player, p2: &mut Player) -> i128 {
    let mut rolls = 0;
    'outer: loop {
        for p in [&mut *p1, &mut *p2].iter_mut() {
            p.apply_rolls(&[
                (rolls) % 100 + 1,
                (rolls + 1) % 100 + 1,
                (rolls + 2) % 100 + 1,
            ]);
            rolls += 3;

            if p.score >= 1000 {
                break 'outer;
            }
        }
    }
    rolls
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    score: i128,
    position: i128,
}

impl Player {
    fn new(position: i128) -> Self {
        Player {
            score: 0,
            position: position - 1,
        }
    }

    fn apply_rolls(&mut self, rolls: &[i128]) {
        self.position += rolls.iter().sum::<i128>();

        if self.position >= 10 {
            self.position %= 10;
        }
        self.score += self.position + 1;
    }
}
