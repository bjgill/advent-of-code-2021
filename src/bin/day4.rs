//! Let's have a nice game of bingo

#[derive(Debug, PartialEq, Clone)]
struct Bingo {
    board: [(u32, bool); 25],
}

impl Bingo {
    fn new(numbers: [u32; 25]) -> Self {
        Bingo {
            board: numbers.map(|n| (n, false)),
        }
    }

    fn mark(&mut self, called_number: u32) -> Option<u32> {
        for mut entry in &mut self.board {
            if entry.0 == called_number {
                entry.1 = true;
            }
        }

        if self.has_won() {
            Some(self.get_sum_of_unmarked_entries())
        } else {
            None
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            if self
                .board
                .iter()
                .skip(5 * i)
                .take(5)
                .all(|&(_, marked)| marked)
            {
                return true;
            }
        }

        for i in 0..5 {
            if self
                .board
                .iter()
                .skip(i)
                .step_by(5)
                .all(|&(_, marked)| marked)
            {
                return true;
            }
        }

        false
    }

    fn get_sum_of_unmarked_entries(&self) -> u32 {
        self.board
            .iter()
            .filter(|(_, marked)| !marked)
            .fold(0, |acc, &(entry, _)| acc + entry)
    }
}

fn play_boards_to_first_victory(called_numbers: &[u32], bingo_boards: &[Bingo]) {
    let mut bingo_boards = bingo_boards.to_vec();

    for called_number in called_numbers {
        for bingo_board in &mut bingo_boards {
            match bingo_board.mark(*called_number) {
                Some(sum_of_unmarked_entries) => {
                    println!(
                        "number: {}, sum: {}, product: {}",
                        called_number,
                        sum_of_unmarked_entries,
                        called_number * sum_of_unmarked_entries
                    );
                    return;
                }
                None => {}
            }
        }
    }
}

fn play_board_to_victory(called_numbers: &[u32], bingo_board: &mut Bingo) -> (usize, u32, u32) {
    for (index, called_number) in called_numbers.iter().enumerate() {
        match bingo_board.mark(*called_number) {
            Some(sum_unmarked) => return (index, *called_number, sum_unmarked),
            None => {}
        }
    }

    panic!("Game did not finish")
}

fn play_boards_to_last_victory(called_numbers: &[u32], bingo_boards: &[Bingo]) {
    let mut bingo_boards = bingo_boards.to_vec();

    let (victory_time, last_called_number, sum_of_unmarked_entries) = bingo_boards
        .iter_mut()
        .map(|board| play_board_to_victory(called_numbers, board))
        .max_by_key(|(victory_time, _last_called_number, _sum_unmarked)| victory_time.clone())
        .unwrap();

    println!(
        "Won at {}, number: {}, sum: {}, product: {}",
        victory_time,
        last_called_number,
        sum_of_unmarked_entries,
        last_called_number * sum_of_unmarked_entries
    );
}

fn main() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();
    let mut input_entries = input.split_whitespace();

    let called_numbers: Vec<u32> = input_entries
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut bingo_numbers: Vec<u32> = input_entries.map(|c| c.parse().unwrap()).collect();
    let bingo_boards: Vec<Bingo> = bingo_numbers
        .chunks_exact_mut(25)
        .map(|n| Bingo::new(n.to_owned().try_into().unwrap()))
        .collect();

    play_boards_to_first_victory(&called_numbers, &bingo_boards);
    play_boards_to_last_victory(&called_numbers, &bingo_boards);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_board() -> Bingo {
        Bingo::new((0..25).collect::<Vec<u32>>().try_into().unwrap())
    }

    #[test]
    fn test_has_not_won() {
        assert_eq!(get_test_board().has_won(), false);
    }

    #[test]
    fn test_mark_first_row() {
        let mut board = get_test_board();

        assert_eq!(board.mark(0), None);
        assert_eq!(board.mark(1), None);
        assert_eq!(board.mark(2), None);
        assert_eq!(board.mark(3), None);
        assert!(board.mark(4).is_some());

        assert_eq!(board.get_sum_of_unmarked_entries(), (5..25).sum());
    }

    #[test]
    fn test_mark_second_row() {
        let mut board = get_test_board();

        assert!(board.mark(5).is_none());
        assert!(board.mark(6).is_none());
        assert!(board.mark(7).is_none());
        assert!(board.mark(8).is_none());
        assert!(board.mark(9).is_some());
    }

    #[test]
    fn test_mark_columns() {
        let mut board = get_test_board();

        assert_eq!(board.mark(0), None);
        assert_eq!(board.mark(5), None);
        assert_eq!(board.mark(10), None);
        assert_eq!(board.mark(15), None);
        assert!(board.mark(20).is_some());
    }

    #[test]
    fn test_play_board_to_victory() {
        let mut board = get_test_board();

        assert_eq!(
            play_board_to_victory(&[0, 1, 2, 3, 4, 5], &mut board),
            (4, 4, (5..25).sum())
        );
    }

    #[test]
    fn test_expected_victory_example3() {
        let mut board = Bingo::new([
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);

        let (_, called_number, sum_unmarked) = play_board_to_victory(
            &[
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            &mut board,
        );

        assert_eq!(called_number, 24);
        assert_eq!(sum_unmarked, 188);
    }

    #[test]
    fn test_expected_victory_example2() {
        let mut board = Bingo::new([
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ]);

        let (_, called_number, sum_unmarked) = play_board_to_victory(
            &[
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            &mut board,
        );

        assert_eq!(called_number, 13);
        assert_eq!(sum_unmarked, 148);
    }
}
