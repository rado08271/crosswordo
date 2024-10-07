use std::cmp::max;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::time::SystemTime;
use crate::entities::board::Board;
use crate::entities::direction::Direction;
use crate::entities::solution::Solution;
use crate::entities::trie::Trie;
use crate::entities::word::Word;
use crate::logic::wfc::WFC;
use crate::MAX;

pub enum LoopState {
    INITIALIZED, PLAYING, FINISHED, FAILED
}

pub struct Controller {
    rows: usize, cols: usize,
    board: Board,
    solution: Solution,
    // Trie is necessary only for entropy search
    dictionary: Trie,
    // History holds information about word being placed and words that were already used
    history: Vec<Word>,
    // Holds current state for a board. If a word is placed on board and game is not finished, it is
    // recalculated in all possible directions
    states: HashMap<usize, Vec<Word>>,
    // Will hold information about sequences so no dictionary search is needed
    sequence_cache: HashMap<String, HashSet<String>>,
}


impl Controller {
    pub fn new(solution: &str, rows: usize, cols: usize, dictionary: Vec<String>) -> Self {
        let mut trie = Trie::new();

        dictionary.iter()
            .filter(|word| word.len() >= MAX)
            .for_each(|word| trie.insert(word));

        let mut controller = Controller {
            rows, cols,
            board: Board::new(rows, cols),
            solution: Solution::new(solution, rows, cols),
            dictionary: trie,
            history: Vec::new(),
            states: HashMap::new(),
            sequence_cache: HashMap::new()
        };

        // FIXME : We could rather have special function for this
        controller.prepare_solution();


        return controller;
    }


    fn prepare_solution(&mut self) -> bool {
        let state = self.solution.calculate_constraints();

        if (!state) {
            panic!("Could not place solution, try again")
        }

        self.solution.print_solution_on_board();
        self.board.put_solution_on_board(&self.solution);

        state
    }


    pub fn print_board(&self) {
        for row in &self.board.board {
            for col in row {
                print!("{}\t", col);
            }
            println!();
        }
    }

    pub fn perform_action(&mut self) {
        self.print_board();

        self.calculate_possible_states();

        let words = &self.filterWords();

        self.putOrBacktrack(words);
    }

    fn backtrack(&mut self) -> bool {
        // Initiate all states
        // let mut states: HashMap::new();
        self.calculate_possible_states();

        // Find word based on states and return lowest entropy words
        let words: Vec<Word> = WFC::find_lowest_entopy_words();

        for word in words {
            // For all lowest entropy words put first word on board
            self.board.put_word_on_board(word)

            // Remove necessary states

            // Initiate states

            // Recalculate entropies

            /** If entropy is 0
                       - remove word from board
                       - if no more words are available go one recursive step back - return false
                       - continue in for loop
                   **/

            /** If entropy is >0
                       - go to next state (call this function again with new states as param)
                   **/

            // in the end return true
        }



        return false;
    }

    fn invalidate_all_states(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.states.remove(&(row * self.rows + col));
            }
        }
    }

    fn invalidate_required_states(&mut self, word: &Word) {
        // TODO : For each character in word (depth) calculate
        for depth in 0..word.word.len() {
            for dir in Direction::DIRECTION_MATRIX() {
                let mut row = word.coords.0 as i32 + (word.direction.getRow() * depth as i32);
                let mut col = word.coords.1 as i32 + (word.direction.getCol() * depth as i32);

                // TODO Reconsider !!!
                // invalidate board.rows - row items or untill wall is touched
                for idx in 0..max(self.rows, self.cols) {
                    if row > 0 && col > 0 && row < self.rows as i32 && col < self.cols as i32 {
                        let c = self.board.board[row as usize][col as usize];
                        // println!("{}.{} r{}c{} = [{}]", idx, (row as usize * self.rows + col as usize), row, col, c);

                        if (c == '*') {
                            break;
                        }

                        // invalidate states cache
                        self.states.remove(&(row as usize * self.rows + col as usize));
                    } else {
                        // if row or col is outside the board let's go to the next one
                        break;
                    }

                    row += dir.getRow();
                    col += dir.getCol();
                }
            }

        }
    }

    fn calculate_possible_states(&mut self)  {
        // For each row and col (each cell) traverse the position in all directions
        for (rowIndex, row) in self.board.board.iter().enumerate() {
            for (colIndex, col) in row.iter().enumerate() {
                let mut words: Vec<Word> = Vec::new();

                // Check states, if a word already has state do not process again, otherwise if states are invalidated get states
                if let Some (cached) = self.states.get(&(rowIndex * self.rows + colIndex)) {
                    words = cached.clone();
                } else {
                    let directional_sequences = self.board.get_sequences_from_position(rowIndex, colIndex).unwrap_or_else(HashMap::new);

                    words = WFC::calculate_entropy_for_acell(
                        rowIndex, colIndex, rowIndex * self.rows + colIndex,
                        directional_sequences, &self.dictionary, self.history.iter().map(|w| w.word.clone()).collect(),
                        &mut self.sequence_cache
                    );

                    self.states.insert((rowIndex * self.rows + colIndex), words.clone());
                }
            }
        }
    }

}