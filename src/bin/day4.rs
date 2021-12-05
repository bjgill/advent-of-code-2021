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

    fn mark(&mut self, called_number: u32) -> Option<()> {
        for mut entry in &mut self.board {
            if entry.0 == called_number {
                entry.1 = true;
            }
        }

        if self.has_won() {
            Some(())
        } else {
            None
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            if self.board.iter().skip(i).take(5).all(|&(_, marked)| marked) {
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
            if bingo_board.mark(*called_number).is_some() {
                let sum_of_unmarked_entries = bingo_board.get_sum_of_unmarked_entries();
                println!(
                    "number: {}, sum: {}, product: {}",
                    called_number,
                    sum_of_unmarked_entries,
                    called_number * sum_of_unmarked_entries
                );
                return;
            }
        }
    }
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

    play_boards_to_first_victory(&called_numbers, &bingo_boards)
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
    fn test_mark_rows() {
        let mut board = get_test_board();

        assert_eq!(board.mark(0), None);
        assert_eq!(board.mark(1), None);
        assert_eq!(board.mark(2), None);
        assert_eq!(board.mark(3), None);
        assert_eq!(board.mark(4), Some(()));

        assert_eq!(board.get_sum_of_unmarked_entries(), (5..25).sum());
    }

    #[test]
    fn test_mark_columns() {
        let mut board = get_test_board();

        assert_eq!(board.mark(0), None);
        assert_eq!(board.mark(5), None);
        assert_eq!(board.mark(10), None);
        assert_eq!(board.mark(15), None);
        assert_eq!(board.mark(20), Some(()));
    }
}
