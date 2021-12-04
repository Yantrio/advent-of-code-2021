use ndarray::{Array2, ArrayBase, Axis, Dim, ViewRepr};

fn main() {
    let input = include_str!("input");
    let lines = input.lines().collect::<Vec<_>>();

    let calls = lines[0]
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parse_boards(input);

    for call in calls {
        select(&mut boards, call);
        while let Some((idx, winning_board)) = detect_winner(&boards) {
            println!(
                "A winner was found with score {}",
                calc_score(winning_board, call)
            );
            boards.remove(idx);
        }
    }
}

fn parse_boards(input: &str) -> Vec<Array2<BingoNumber>> {
    let mut boards: Vec<Array2<BingoNumber>> = vec![];
    let mut lines_on_board = vec![];
    for line in input.lines().skip(1) {
        if line.trim().is_empty() {
            if !lines_on_board.is_empty() {
                boards.push(create_board(lines_on_board));
                lines_on_board = vec![];
                continue;
            }
        } else {
            lines_on_board.push(line);
        }
    }
    boards.push(create_board(lines_on_board));
    boards
}

fn calc_score(board: &Array2<BingoNumber>, winning_call: usize) -> usize {
    // find the sum of all unmarked numbers
    board
        .iter()
        .filter(|bingonum| !bingonum.selected)
        .map(|n| n.value)
        .sum::<usize>()
        * winning_call
}

fn all_selected(lane: ArrayBase<ViewRepr<&BingoNumber>, Dim<[usize; 1]>>) -> bool {
    for item in lane.iter() {
        if !item.selected {
            return false;
        }
    }
    true
}

fn detect_winner(boards: &[Array2<BingoNumber>]) -> Option<(usize, &Array2<BingoNumber>)> {
    for (idx, board) in boards.iter().enumerate() {
        for axis in (0..=1).map(Axis) {
            for lane in board.lanes(axis).into_iter() {
                if all_selected(lane) {
                    return Some((idx, board));
                }
            }
        }
    }

    None
}

fn select(boards: &mut Vec<Array2<BingoNumber>>, value: usize) {
    boards.iter_mut().for_each(|board| {
        board
            .iter_mut()
            .filter(|n| n.value == value)
            .for_each(|n| n.selected = true)
    });
}

fn create_board(lines: Vec<&str>) -> Array2<BingoNumber> {
    let raw = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.trim()
                .split(' ')
                .filter(|i| !i.trim().is_empty())
                .map(|n| BingoNumber::new(n.parse().unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    Array2::from_shape_vec((5, 5), raw).unwrap()
}

#[derive(Debug)]
struct BingoNumber {
    selected: bool,
    value: usize,
}

impl BingoNumber {
    fn new(value: usize) -> BingoNumber {
        BingoNumber {
            selected: false,
            value,
        }
    }
}
