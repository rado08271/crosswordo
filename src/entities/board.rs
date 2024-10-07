use std::cmp::max;
use std::collections::HashMap;
use std::fmt::format;
use crate::entities::direction::Direction;
use crate::entities::solution::Solution;
use crate::entities::word::Word;

pub struct Board {
    pub cols: usize,
    pub rows: usize,
    pub board: Vec<Vec<char>>,
    // will track each position with am info of first iteration
    contributions: Vec<Vec<Option<usize>>>,
    tracker: usize
}

impl Board {
    // Determine the function signature. The function should accept the dimensions of the grid (rows and columns) and return an initialized grid.
    pub fn new(rows: usize, cols: usize) -> Board {
        // if cols > 10 || rows > 10 {
        //     panic!("The board seems too big to handle");
        // }

        if cols <= 3 || rows <= 3 {
            panic!("The board seems too small to handle");
        }

        return Board {
            rows, cols,
            // Implement the logic to initialize the grid with wildcards ('*').
            board: vec![vec!['?'; cols]; rows],
            contributions: vec![vec![None; cols]; rows],
            tracker: 0
        }
    }

    pub fn print_board(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}-{:0>2}\t", self.board[row][col], self.contributions[row][col].unwrap_or(99));
            }
            println!();
        }
    }

    pub fn is_board_populated(&self) -> bool {
        // if board has at least 1 questionmark it's not finished yet
        for row in 0..self.rows {
            for col in 0..self.cols {
                if Some(self.board[row][col]).unwrap() == '?' {
                    return false
                }
            }
        }

        return true;

    }

    pub fn put_solution_on_board(&mut self, solution: &Solution) {
        for (position, c) in solution.locations.clone() {
            let row = position / self.cols;
            let col = position % self.cols;

            // self.board[row][col] =  *c;
            self.board[row][col] =  '*';
        }
    }

    // TODO : direction, position, word
    pub fn put_word_on_board(&mut self, word: &Word) {
        let Word {word: w, direction, coords: (row, col), .. } = word.clone();

        self.put_sequence_on_board(w, row, col, direction);
        self.tracker += 1;

        println!("===== putting {}. word {} at R{}C{}[{}] =====", self.tracker, word.clone().word, row, col, direction.getIndex());
        self.print_board()
    }

    pub fn remove_word_from_board(&mut self, word: &Word) {
        let Word {word: w, direction, coords: (row, col), ..} = word.clone();

        self.tracker -= 1;
        self.remove_sequence_from_board(w, row, col, direction);

        println!("===== removing {}. word {} at R{}C{}[{}] =====", self.tracker + 1, word.clone().word, row, col, direction.getIndex());
        self.print_board()
    }

    pub fn get_sequences_from_position(&self, row: usize, col: usize) -> Option<HashMap<Direction, String>> {
        let c = self.board[row][col];

        if c == '*' {
            // return [Default::default(); 9];
            return None;
        }

        // let mut sequences: [String; 9] = [Default::default(); 9];
        let mut sequences: HashMap<Direction, String> = HashMap::new();

        let max_depth = max(i32::try_from(self.rows).unwrap(), i32::try_from(self.cols).unwrap());

        // first check if it's not a solution stuff
        for DIRECTION_MATRIX_CELL in Direction::DIRECTION_MATRIX() {
            let sequence = (self.get_current_sequence(max_depth, i32::try_from(row).unwrap(), i32::try_from(col).unwrap(), &DIRECTION_MATRIX_CELL));
            sequences.insert(DIRECTION_MATRIX_CELL, sequence.unwrap_or(String::new()));
            // sequences[DIRECTION_MATRIX_CELL.getIndex()] = sequence;
        }

        return Some(sequences);
    }

    fn get_current_sequence(&self, max_length: i32, row: i32, col: i32, direction: &Direction) -> Option<String> {
        if *direction == Direction::CENTER() {
            return None;
        }

        let mut sequence = String::from("");

        for depth in 0..max_length {
            let row_direction = (direction.getRow() * depth) + row;
            let col_direction = (direction.getCol() * depth) + col;

            if row_direction < 0 || col_direction < 0 || row_direction >= (i32::try_from(self.rows).unwrap()) || col_direction >= (i32::try_from(self.cols).unwrap()) {
                break;
            }

            let c = self.board[row_direction as usize][col_direction as usize];
            if (c == '*') {
                break;
            }

            sequence.push(c);
        }

        // FIXME if sequence is all existing chars, we should skit it
        if !sequence.contains('?') {
            return None;
        }
        return Some(sequence);
    }

    fn remove_sequence_from_board(&mut self, sequence: String, row: usize, col: usize, direction: Direction) {
        for (depth, c) in sequence.char_indices() {
            let row = i32::try_from(row).unwrap() + (direction.getRow() * i32::try_from(depth).unwrap());
            let col = i32::try_from(col).unwrap() + (direction.getCol() * i32::try_from(depth).unwrap());

            if (row >= 0 && col >= 0) {
                if Some(self.tracker) == self.contributions[row as usize][col as usize] {
                    self.board[row as usize][col as usize] = '?';
                    self.contributions[row as usize][col as usize] = None;
                }
            }
        }
    }

    fn put_sequence_on_board(&mut self, sequence: String, row: usize, col: usize, direction: Direction) {
        for (depth, c) in sequence.char_indices() {
            let row = i32::try_from(row).unwrap() + (direction.getRow() * i32::try_from(depth).unwrap());
            let col = i32::try_from(col).unwrap() + (direction.getCol() * i32::try_from(depth).unwrap());

            if (row >= 0 && col >= 0) {
                self.board[row as usize][col as usize] = c;
                self.contributions[row as usize][col as usize].get_or_insert(self.tracker);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "too big")]
    fn test_big_board() {
        let rows = 100;
        let cols = 100;
        let board = Board::new(rows, cols);
    }

    #[test]
    #[should_panic(expected = "too small")]
    fn test_empty_board() {
        let rows = 0;
        let cols = 0;
        let board = Board::new(rows, cols);
    }


    #[test]
    fn test_ok_board() {
        let rows = 8;
        let cols = 4;
        let board = Board::new(rows, cols);
    }

    #[test]
    fn test_content_board() {
        let rows = 8;
        let cols = 4;
        let board = Board::new(rows, cols);
        assert_eq!(board.board, vec![vec!['?'; cols]; rows])
    }

    #[test]
    fn test_items_board() {
        let rows = 8;
        let cols = 4;
        let board = Board::new(rows, cols);

        assert_eq!(board.board.len(), rows);
        for row in board.board {
            assert_eq!(row.len(), cols);
            for cell in row {
                assert_eq!(cell, '?');
            }
        }
    }

}