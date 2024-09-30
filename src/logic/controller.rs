use std::cmp::max;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::SystemTime;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
    states: BTreeMap<usize, Vec<Word>>,
    // Will hold information about sequences so no dictionary search is needed
    sequenceCache: HashMap<String, HashSet<String>>,
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
            states: BTreeMap::new(),
            sequenceCache: HashMap::new()
        };

        // FIXME : We could rather have special function for this
        controller.calculateSolution();
        controller.setupWallsOnBoard();

        return controller;
    }

    pub fn printBoard(&self) {
        for row in &self.board.board {
            for col in row {
                print!("{}\t", col);
            }
            println!();
        }
    }


    pub fn filterWords(&mut self) -> Vec<Word> {
        let entropyMin = self.states
            .iter()
            .filter(|(_, words)| !words.is_empty())
            .map(|(_, words)| words.len())
            .min()
            .unwrap_or(0);

        if (entropyMin == 0) {
            panic!("Cannot be initiated")
        }
        // TODO : Add weight to each direction or calculate weights based on all positions, open position and words in certain direction

        // We will save all possible positions for lowest entropies and select single word (for any direction) to put on board
        let mut savedWords: Vec<Word> = Vec::new();
        self.states
            .iter()
            .filter(|(_, words)| words.len() == entropyMin)
            .for_each(|(_, words)| savedWords.extend(words.clone()));

        let randomWords: Vec<Word> = savedWords
            .choose_multiple(&mut thread_rng(), savedWords.len())
            .cloned().collect::<Vec<Word>>();

        // println!("min {} dir {} words {} R{}C{}", entropyMin, randomWord.direction.getIndex(), randomWord.word, randomWord.coords.0, randomWord.coords.1);
        return randomWords;
    }

    fn putOrBacktrack(&mut self, words: &Vec<Word>) {
        let word = words.first().unwrap();

        // TODO : If loop is not finished but entropy is 0 attempt is considered a failed one, we should do some backtracking on possible words
        /**
         *       Here We need to create the backtracking mechanism
         *       - We need to recalculate entropies after word is put on board
         *       - We need to select next word from a list of words
         *       - If all words fail to fill the board we should go to
         *       previous state in history
         **/


        self.board.putWordOnBoard(word.clone());


        // clear cache
        self.invalidateStates(word);

    }

    pub fn performAction(&mut self) {
        self.printBoard();
        // self.printEntropies(entropy);

        let words = self.calculatePossibleWords();

        let words = &self.filterWords();

        self.putOrBacktrack(words);
    }

    fn invalidateStates(&mut self, word: &Word) {
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
                        println!("{}.{} r{}c{} = [{}]", idx, (row as usize * self.rows + col as usize), row, col, c);

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

    fn calculateSolution(&mut self) -> bool {
        self.solution.calculateConstraints()
    }

    fn setupWallsOnBoard(&mut self) {
        self.board.putSolutionOnBoard(&self.solution);
    }

    fn calculatePossibleWords(&mut self) -> Vec<Vec<Word>> {
        // FIXME : Remove
        let start = SystemTime::now();

        // each position may have words in 9 directions (8 + center)
        let mut possibleWords: Vec<Vec<Word>> = Vec::new();

        // For each row and col (each cell) traverse the position in all directions
        for (rowIndex, row) in self.board.board.iter().enumerate() {
            for (colIndex, col) in row.iter().enumerate() {

                let started = SystemTime::now();
                let mut avgSearch = 0;
                let mut avgSearchA = 0;

                print!("==== R{}C{} | ", rowIndex, colIndex);

                let mut words: Vec<Word> = Vec::new();

                if let Some (cached) = self.states.get(&(rowIndex * self.rows + colIndex)) {
                    words = cached.clone();
                } else {
                    let directionalSequences = self.board.getSequencesFromPosition(rowIndex, colIndex).unwrap_or_else(HashMap::new);

                    words = WFC::calculateEntropyForACell(
                        rowIndex, colIndex, rowIndex * self.rows + colIndex,
                        directionalSequences, &self.dictionary, self.history.iter().map(|w| w.word.clone()).collect(),
                        &mut self.sequenceCache
                    );

                    self.states.insert((rowIndex * self.rows + colIndex), words.clone());
                }

                possibleWords.push(words);
                if avgSearchA <= 0 {
                    avgSearchA = 1
                }
                println!(" seq ${}ms | avg ({}ms) per searchs ({}x) | took {}ms ", started.elapsed().unwrap().as_millis(), avgSearch, avgSearchA, avgSearch / avgSearchA);

            }
        }
        println!("calculating entropy took {}ms", start.elapsed().unwrap().as_millis());
        possibleWords
    }

}