use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::entities::board::Board;
use crate::entities::direction::Direction;
use crate::entities::solution::Solution;
use crate::entities::trie::Trie;
use crate::entities::word::Word;
use crate::MAX;

pub enum LoopState {
    INITIALIZED, PLAYING, FINISHED, FAILED
}

pub struct Controller {
    rows: usize, cols: usize,
    board: Board,
    dictionary: Trie,
    solution: Solution,
    used: Vec<String>,
    // History holds information about word being placed
    history: Vec<Word>,
    // Will hold information about sequences and possible words for those sequences if it is
    sequenceCache: HashMap<String, HashSet<String>>,
    // Holds current state for a board. If a word is placed on board and game is not finished, it is
    // recalculated in all possible directions
    statesCache: HashMap<usize, Vec<Word>>
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
            dictionary: trie,
            solution: Solution::new(solution, rows, cols),
            used: Vec::new(),
            history: Vec::new(),
            statesCache: HashMap::new(),
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

    fn getMinimumEntropy(&self, possibleWords: &Vec<Vec<Word>>) -> usize {
        possibleWords
            .iter()
            .filter(|words| !words.is_empty())
            .map(|words| words.len())
            .min()
            .unwrap_or(0)
    }

    pub fn filterWords(&mut self, possibleWords: &Vec<Vec<Word>>) -> Vec<Word> {
        let entropyMin = self.getMinimumEntropy(possibleWords);
        if (entropyMin == 0) {
            panic!("Cannot be initiated")
        }
        // TODO : Add weight to each direction or calculate weights based on all positions, open position and words in certain direction

        // We will save all possible positions for lowest entropies and select single word (for any direction) to put on board
        let mut savedWords: Vec<Word> = Vec::new();
        possibleWords
            .iter()
            .filter(|words| words.len() == entropyMin)
            .for_each(|words| savedWords.extend(words.clone()));

        let randomWords: Vec<Word> = savedWords
            .choose_multiple(&mut thread_rng(), savedWords.len())
            .cloned().collect::<Vec<Word>>();

        // println!("min {} dir {} words {} R{}C{}", entropyMin, randomWord.direction.getIndex(), randomWord.word, randomWord.coords.0, randomWord.coords.1);
        return randomWords;
    }

    pub fn printEntropies(&self, entropies: &Vec<HashSet<Word>>) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let entropy = entropies[row * self.rows + col].len();
                // print!("{}\t", entropy);
                print!("R{}C{} has {} items\t", row, col, entropy)
            }
            println!();
        }
        println!();
    }

    pub fn performAction(&mut self) {
        // self.printBoard();

        let allPossibleWords = &self.calculatePossibleWords();
        // self.printEntropies(entropy);
        // TODO : Word Selection should take previous steps into consideration because it might get stuck
        let words = &self.filterWords(allPossibleWords);
        let word = words.first().unwrap();
        self.board.putWordOnBoard(word.clone());
        // TODO : If game is not finished but entropy is 0 attempt is considered a failed one, we should do some backtracking on possible words

        // Put selected word in a list of excluded words
        // TODO : Consider putting Word in used words
        self.used.push(word.clone().word);

        // clear cache
        self.invalidateCache(word);
        // self.printBoard()
    }

    fn invalidateCache(&mut self, word: &Word) {
        // TODO : For each character in word (depth) calculate
        for depth in 0..word.word.len() {
            for dir in Direction::DIRECTION_MATRIX() {
                let mut row = word.coords.0 as i32 + (word.direction.getRow() * depth as i32);
                let mut col = word.coords.1 as i32 + (word.direction.getCol() * depth as i32);

                // invalidate board.rows - row items or untill wall is touched
                for idx in 0..max(self.rows, self.cols) {
                    if row > 0 && col > 0 && row < self.rows as i32 && col < self.cols as i32 {
                        println!("{}.{} r{}c{}", idx, (row as usize * self.rows + col as usize), row, col);
                        // invalidate cache
                        self.statesCache.remove(&(row as usize * self.rows + col as usize));
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

                // HashSet ensures we don't have duplicit words
                let mut words: Vec<Word> = Vec::new();

                if let Some (cached) = self.statesCache.get(&(rowIndex * self.rows + colIndex)) {
                    words = cached.clone();
                } else {
                    let directionalSequences = self.board.getSequencesFromPosition(rowIndex, colIndex).unwrap_or_else(HashMap::new);

                    // From default sequence we might have
                    for (direction, sequence) in directionalSequences {
                        // println!("dir {} - seq {}", direction.getIndex(), sequence);
                        // for sequences lower than MAX we won't compile
                        if (sequence.len() >= MAX) {
                            // FIXME : Traversing all positions again is unnecessary
                            for depth in MAX..(sequence.len() + 1) {
                                let searchStarted = SystemTime::now();

                                let subsequence = &sequence[..depth];
                                let trieSearchResult = self.sequenceCache.entry(subsequence.to_string()).or_insert(self.dictionary.search(subsequence));

                                // println!("\t\tsearch depth {} took {}ms", depth, searchStarted.elapsed().unwrap().as_millis());
                                avgSearch = avgSearch + searchStarted.elapsed().unwrap().as_millis();
                                avgSearchA = avgSearchA + 1;

                                let wordsProcessed: Vec<Word> = trieSearchResult
                                    .iter()
                                    .filter(|word| !self.used.contains(word))
                                    .map(|word| Word::new(word.clone(), direction, (rowIndex, colIndex)))
                                    .collect();

                                words.extend(wordsProcessed);
                            }
                        }
                    }
                    self.statesCache.insert((rowIndex * self.rows + colIndex), words.clone());
                }

                // entropy.insert(rowIndex * self.rows + colIndex, words);
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