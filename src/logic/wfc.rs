use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::SystemTime;
use crate::entities::direction::Direction;
use crate::entities::trie::Trie;
use crate::entities::word::Word;
use crate::MAX;

pub struct WFC {

}

impl WFC {
    pub fn calculateEntropyForACell(
        row: usize, col: usize, idx: usize,
        directionalSequences: HashMap<Direction, String>,
        dictionary: &Trie,
        used: Vec<String>,
        sequenceCache: &mut HashMap<String, HashSet<String>>,
    ) -> Vec<Word> {
        // FIXME : Remove
        let start = SystemTime::now();

        // For each row and col (each cell) traverse the position in all directions
        let started = SystemTime::now();
        let mut avgSearch = 0;
        let mut avgSearchA = 0;
        print!("==== R{}C{} | ", row, col);

        let mut words: Vec<Word> = Vec::new();

        // From default sequence we might have
        for (direction, sequence) in directionalSequences {
            // println!("dir {} - seq {}", direction.getIndex(), sequence);
            // for sequences lower than MAX we won't compile
            if (sequence.len() >= MAX) {
                // FIXME : Traversing all positions again is unnecessary
                for depth in MAX..(sequence.len() + 1) {
                    let searchStarted = SystemTime::now();

                    let subsequence = &sequence[..depth];
                    let trieSearchResult = sequenceCache.entry(subsequence.to_string()).or_insert(dictionary.search(subsequence));

                    // println!("\t\tsearch depth {} took {}ms", depth, searchStarted.elapsed().unwrap().as_millis());
                    avgSearch = avgSearch + searchStarted.elapsed().unwrap().as_millis();
                    avgSearchA = avgSearchA + 1;

                    let wordsProcessed: Vec<Word> = trieSearchResult
                        .iter()
                        .filter(|word| used.contains(word))
                        .map(|word| Word::new(word.clone(), direction, (row, col)))
                        .collect();

                    words.extend(wordsProcessed);
                }
            }
        }

        words
    }

    // pub fn printEntropies(&self, entropies: &Vec<HashSet<Word>>) {
        // for row in 0..self.rows {
        //     for col in 0..self.cols {
        //         let entropy = entropies[row * self.rows + col].len();
        //         // print!("{}\t", entropy);
        //         print!("R{}C{} has {} items\t", row, col, entropy)
        //     }
        //     println!();
        // }
        // println!();
    // }
}