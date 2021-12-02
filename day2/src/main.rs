fn main() {
    let input = include_str!("input");
    let inputs: Vec<Instruction> = input.lines().map(|l| Instruction::from(l.trim())).collect();

    let sub = follow_directions_1(&inputs);
    println!("Part 1 {:?}", sub.position.x * sub.position.y);

    let sub = follow_directions_2(&inputs);
    println!("Part 2 {:?}", sub.position.x * sub.position.y);
}

fn follow_directions_1(instructions: &[Instruction]) -> Sub {
    let mut sub = Sub {
        position: Vector2 { x: 0, y: 0 },
        aim: 0,
    };

    for inst in instructions.iter() {
        match inst.direction {
            "forward" => sub.position.x += inst.amount,
            "up" => sub.position.y -= inst.amount,
            "down" => sub.position.y += inst.amount,
            _ => panic!(),
        }
    }

    sub
}

fn follow_directions_2(instructions: &[Instruction]) -> Sub {
    let mut sub = Sub::new();

    for inst in instructions.iter() {
        match inst.direction {
            "forward" => {
                sub.position.x += inst.amount;
                sub.position.y += sub.aim * inst.amount;
            }
            "up" => sub.aim -= inst.amount,
            "down" => sub.aim += inst.amount,
            _ => panic!(),
        }
    }

    sub
}

#[derive(Debug)]
struct Sub {
    position: Vector2,
    aim: i32,
}

impl Sub {
    fn new() -> Sub {
        Sub {
            position: Vector2 { x: 0, y: 0 },
            aim: 0,
        }
    }
}

#[derive(Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Instruction<'a> {
    amount: i32,
    direction: &'a str,
}

impl<'a> Instruction<'a> {
    fn new(amount: i32, dir: &str) -> Instruction {
        Instruction {
            amount,
            direction: dir,
        }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(s: &'a str) -> Self {
        let split = s.trim().split(' ').collect::<Vec<_>>();
        Instruction::new(split[1].parse::<i32>().unwrap(), split[0])
    }
}
