use itertools::Itertools;
use std::fmt::Display;

pub fn run(input: String) -> Result<String, String> {
    let lines: Vec<&str> = input.lines().collect();
    let draws: Vec<u8> = lines[0]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = lines
        .iter()
        .skip(1)
        .chunks(6)
        .into_iter()
        .map(|boarditer| {
            let board_squares: Vec<u8> = boarditer
                .skip(1)
                .flat_map(|line| line.split_whitespace())
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            BingoBoard::new(5, board_squares)
        })
        .collect();

    //Finds the first winning board (ie, part 1)
    // let mut draws_made: Vec<u8> = Vec::with_capacity(draws.len());
    // if let ControlFlow::Break((winning_draw_idx, winning_board_idx)) = draws
    //     .iter()
    //     .enumerate()
    //     .try_fold::<(usize, usize), _, _>((0, 0), |_, (draw_idx, &draw)| {
    //         draws_made.push(draw);
    //         if let ControlFlow::Break(winning_board_idx) = boards
    //             .iter_mut()
    //             .enumerate()
    //             .try_fold::<usize, _, _>(0, |_, (board_idx, board)| {
    //                 board.record_draw(draw);
    //                 if board.has_won() {
    //                     ControlFlow::Break(board_idx)
    //                 } else {
    //                     ControlFlow::Continue(0)
    //                 }
    //             })
    //         {
    //             ControlFlow::Break((draw_idx, winning_board_idx))
    //         } else {
    //             ControlFlow::Continue((0, 0))
    //         }
    //     })
    // {
    //     let winning_board = &boards[winning_board_idx];
    //     let winning_number = *(&draws[winning_draw_idx]);
    //     let unmarked_numbers = winning_board.get_unmarked_numbers();
    //     let unmarked_sum: u64 = unmarked_numbers.iter().map(|b| *b as u64).sum();
    //     let score = unmarked_sum * winning_number as u64;

    //     println!("Draws made: {}", draws_made.iter().join(", "));
    //     for board in &boards {
    //         if !board.has_won() {
    //             println!("Losing board:\n{}", board);
    //         }
    //     }
    //     println!("Winning number: {}", winning_number);
    //     println!("Winning board: \n{}", winning_board);
    //     println!("Unmarked numbers: {}", unmarked_numbers.iter().join(", "));
    //     println!("Unmarked sum: {}", unmarked_sum);
    //     println!("Score: {}", score);
    //     return Ok(format!("Score: {}", score));
    // } else {
    //     return Err(String::from("Failed to find a winner"));
    // }

    // Find the _last_ winning board (ie, part 2)
    let draws_made: Vec<u8> = Vec::with_capacity(draws.len());
    let mut non_won_boards = boards;
    let mut last_number: Option<u8> = None;

    for draw in draws {
        if non_won_boards.len() > 1 {
            for board in &mut non_won_boards {
                board.record_draw(draw);
            }
            non_won_boards = non_won_boards
                .into_iter()
                .filter(|b| !b.has_won())
                .collect();
        } else {
            let lb = non_won_boards.get_mut(0).unwrap();
            lb.record_draw(draw);
            if lb.has_won() {
                last_number = Some(draw);
                break;
            }
        }
    }

    let last_board = &non_won_boards[0];
    let unmarked_numbers = last_board.get_unmarked_numbers();
    let unmarked_sum: u64 = unmarked_numbers.iter().map(|b| *b as u64).sum();
    let score = unmarked_sum * last_number.unwrap() as u64;

    println!("Draws made: {}", draws_made.iter().join(", "));
    println!("Last number: {}", last_number.unwrap());
    println!("Last board: \n{}", last_board);
    println!("Unmarked numbers: {}", unmarked_numbers.iter().join(", "));
    println!("Unmarked sum: {}", unmarked_sum);
    println!("Score: {}", score);
    return Ok(format!("Score: {}", score));
}

const MARK_MASK: u8 = 0b10000000;
const VALUE_MASK: u8 = 0b01111111;

#[derive(Debug, Eq, PartialEq, Clone)]
struct BingoSquare {
    val: u8,
}

impl BingoSquare {
    fn new(val: u8) -> BingoSquare {
        BingoSquare { val }
    }

    fn mark(&mut self) {
        self.val |= MARK_MASK;
    }

    fn is_marked(&self) -> bool {
        (self.val & MARK_MASK) == MARK_MASK
    }

    fn get_value(&self) -> u8 {
        self.val & VALUE_MASK
    }
}

#[derive(Debug, Clone)]
struct BingoBoard {
    squares: Vec<BingoSquare>,
    size: usize,
}

impl BingoBoard {
    fn new(size: usize, squares: Vec<u8>) -> BingoBoard {
        let squares = squares.into_iter().map(BingoSquare::new).collect();
        BingoBoard { size, squares }
    }

    fn record_draw(&mut self, num: u8) {
        for i in 0..self.squares.len() {
            let sq = &self.squares[i];
            if !sq.is_marked() && sq.get_value() == num {
                self.squares[i].mark();
            }
        }
    }

    fn has_won(&self) -> bool {
        self.get_winning_numbers().is_some()
    }

    fn get_winning_numbers(&self) -> Option<Vec<&BingoSquare>> {
        self.rows()
            .chain(self.columns())
            .find(|squares| squares.iter().all(|c| c.is_marked()))
    }

    fn get_unmarked_numbers(&self) -> Vec<u8> {
        self.squares
            .iter()
            .filter(|s: &&BingoSquare| !s.is_marked())
            .map(|s| s.get_value())
            .collect()
    }

    fn rows(&self) -> impl Iterator<Item = Vec<&BingoSquare>> {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.size {
                let row = self.get_row(index);
                index += 1;
                Some(row)
            } else {
                None
            }
        })
    }

    fn columns(&self) -> impl Iterator<Item = Vec<&BingoSquare>> {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.size {
                let row = self.get_column(index);
                index += 1;
                Some(row)
            } else {
                None
            }
        })
    }

    fn get_row(&self, row_idx: usize) -> Vec<&BingoSquare> {
        self.squares
            .iter()
            .skip(row_idx * self.size)
            .take(self.size)
            .collect()
    }

    fn get_column(&self, col_idx: usize) -> Vec<&BingoSquare> {
        self.squares
            .iter()
            .skip(col_idx)
            .step_by(self.size)
            .collect()
    }
}

impl Display for BingoBoard {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for r in self.rows() {
            for sq in r.into_iter() {
                let mark = if sq.is_marked() { "*" } else { "" };
                write!(fmt, "{}{} ", sq.get_value().to_string(), mark)?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn can_mark_square_and_get_value() {
        let mut sq = BingoSquare::new(127);
        assert_eq!(127, sq.get_value());
        assert_eq!(false, sq.is_marked());

        sq.mark();
        assert_eq!(127, sq.get_value());
        assert_eq!(true, sq.is_marked());
    }

    #[test]
    pub fn board_marking_and_reading() {
        let mut board = BingoBoard::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("{}", board);

        assert_eq!(false, board.has_won());
        assert_eq!(None, board.get_winning_numbers());
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            board.get_unmarked_numbers()
        );

        board.record_draw(2);
        assert_eq!(false, board.has_won());
        assert_eq!(vec![1, 3, 4, 5, 6, 7, 8, 9], board.get_unmarked_numbers());
        board.record_draw(3);
        assert_eq!(false, board.has_won());
        assert_eq!(vec![1, 4, 5, 6, 7, 8, 9], board.get_unmarked_numbers());

        board.record_draw(5);
        board.record_draw(9);
        assert_eq!(false, board.has_won());
        assert_eq!(vec![1, 4, 6, 7, 8], board.get_unmarked_numbers());

        board.record_draw(8);
        assert_eq!(true, board.has_won());
        let winning_squares: Vec<u8> = board
            .get_winning_numbers()
            .unwrap()
            .into_iter()
            .map(|bs| bs.get_value())
            .collect();

        println!("{}", board);

        assert_eq!(vec![2, 5, 8], winning_squares);
        assert_eq!(vec![1, 4, 6, 7], board.get_unmarked_numbers());
    }
}
