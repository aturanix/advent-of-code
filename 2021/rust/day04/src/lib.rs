const BOARD_ROWS: usize = 5;
const BOARD_COLS: usize = 5;
const BOARD_SIZE: usize = BOARD_ROWS * BOARD_COLS;

#[derive(Debug)]
enum Number {
    Marked(u8),
    Unmarked(u8),
}

type Board = Vec<Number>;

fn parse_values(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut it = input.split_ascii_whitespace();

    let numbers: Vec<u8> = it
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    loop {
        let board: Board = it
            .by_ref()
            .take(BOARD_SIZE)
            .filter_map(|x| x.parse().ok())
            .map(Number::Unmarked)
            .collect();

        if board.len() != BOARD_SIZE {
            break;
        }

        boards.push(board);
    }

    (numbers, boards)
}

pub fn solve1(input: &str) -> usize {
    let (numbers, mut boards) = parse_values(input);
    for number in numbers.into_iter() {
        for board in boards.iter_mut() {
            if let Some(position) = find_unmarked_number(board, number) {
                board[position] = Number::Marked(number);

                let (row, col) = (position / BOARD_COLS, position % BOARD_ROWS);

                let start = row * BOARD_ROWS;
                let end = start + BOARD_COLS;
                debug_assert_eq!((start..end).count(), BOARD_COLS);

                let row_marked = (start..end).all(|i| matches!(board[i], Number::Marked(_)));

                let start = col;
                let end = BOARD_SIZE;
                debug_assert_eq!((start..end).step_by(BOARD_COLS).count(), BOARD_ROWS);

                let col_marked = (start..end)
                    .step_by(BOARD_COLS)
                    .all(|i| matches!(board[i], Number::Marked(_)));

                if row_marked || col_marked {
                    let sum: usize = board
                        .iter()
                        .filter_map(|x| match x {
                            Number::Unmarked(v) => Some(*v as usize),
                            Number::Marked(_) => None,
                        })
                        .sum();
                    return number as usize * sum;
                }
            }
        }
    }

    0
}

fn find_unmarked_number(board: &[Number], number: u8) -> Option<usize> {
    board.iter().position(|x| {
        if let Number::Unmarked(v) = x {
            *v == number
        } else {
            false
        }
    })
}

pub fn solve2(input: &str) -> usize {
    let (numbers, mut boards) = parse_values(input);
    for number in numbers.into_iter() {
        let mut winners: Vec<usize> = Vec::new();
        for (i, board) in boards.iter_mut().enumerate() {
            if let Some(position) = find_unmarked_number(board, number) {
                board[position] = Number::Marked(number);

                let (row, col) = (position / BOARD_COLS, position % BOARD_ROWS);

                let start = row * BOARD_ROWS;
                let end = start + BOARD_COLS;
                debug_assert_eq!((start..end).count(), BOARD_COLS);

                let row_marked = (start..end).all(|i| matches!(board[i], Number::Marked(_)));

                let start = col;
                let end = BOARD_SIZE;
                debug_assert_eq!((start..end).step_by(BOARD_COLS).count(), BOARD_ROWS);

                let col_marked = (start..end)
                    .step_by(BOARD_COLS)
                    .all(|i| matches!(board[i], Number::Marked(_)));

                if row_marked || col_marked {
                    winners.push(i);
                }
            }
        }

        if boards.len() == winners.len() {
            let sum: usize = boards[winners.pop().unwrap()]
                .iter()
                .filter_map(|x| match x {
                    Number::Unmarked(v) => Some(*v as usize),
                    Number::Marked(_) => None,
                })
                .sum();
            return number as usize * sum;
        }

        while let Some(v) = winners.pop() {
            boards.swap_remove(v);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 4512);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 1924);
    }
}
