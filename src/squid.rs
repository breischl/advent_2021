use itertools::Itertools;
use std::fmt::Display;

pub fn run(input: String) -> Result<String, String> {
    let lines: Vec<&str> = input.lines().collect();
    let draws: Vec<u8> = lines[0]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = lines
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

    let mut winning_board_idx: Option<usize> = None;
    let mut winning_number: Option<u8> = None;
    for draw in draws {
        //Needed to use the indexed loop here because otherwise there was no way to both modify the board and return it if it had won
        //Fought the borrow checker for waaaaay too long trying to figure out how to avoid this
        for idx in 0..boards.len() {
            let board = boards.get_mut(idx).unwrap();
            board.record_draw(draw);
            if board.has_won() {
                winning_board_idx = Some(idx);
                winning_number = Some(draw);
                break;
            }
        }
        if winning_board_idx.is_some() {
            break;
        }
    }

    if let Some(idx) = winning_board_idx {
        let winning_board = &boards[idx];
        println!("Winning number: {}", winning_number.unwrap());
        println!("Winning board: \n{}", winning_board);
    }

    Err(String::from("Not implemented"))
}

const MARK_MASK: u8 = 0b10000000;
const VALUE_MASK: u8 = 0b01111111;
struct BingoSquare {
    val: u8,
}

impl BingoSquare {
    fn new(val: u8) -> BingoSquare {
        BingoSquare { val }
    }

    fn mark(&mut self) {
        self.val = self.val | MARK_MASK;
    }

    fn is_marked(&self) -> bool {
        (self.val & MARK_MASK) == MARK_MASK
    }

    fn get_value(&self) -> u8 {
        self.val | VALUE_MASK
    }
}

struct BingoBoard {
    squares: Vec<BingoSquare>,
    size: usize,
}

impl BingoBoard {
    fn new(size: usize, squares: Vec<u8>) -> BingoBoard {
        let squares = squares.into_iter().map(BingoSquare::new).collect();
        BingoBoard {
            size: size,
            squares: squares,
        }
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

    fn get_unmarked_numbers<'a>(&self) -> Vec<u8> {
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
                write!(fmt, "{} ", sq.get_value().to_string())?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}
