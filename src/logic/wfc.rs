use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::SystemTime;
use crate::entities::direction::Direction;
use crate::entities::trie::Trie;
use crate::entities::word::Word;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::MAX;

pub struct WFC {

}

impl WFC {

    pub fn find_lowest_entropy(
        states: &HashMap<usize, Vec<Word>>
    ) -> usize {
        let mut new_words = Vec::new();

        let words = states
            .iter()
            .filter(|(_, words)| {
                new_words.push(words.clone());
                return !words.is_empty()
            });

        let entropy_min = words
            .map(|(_, words)| {
                words.len()
            })
            .min()
            .unwrap_or(0);

        // println!("entropy {}", entropy_min);
        return entropy_min;
    }

    pub fn find_random_lowest_entropy_words(
        states: &HashMap<usize, Vec<Word>>
    ) -> Vec<Word> {
        let entropy_min = WFC::find_lowest_entropy(states);

        if (entropy_min == 0) {
            panic!("Cannot be initiated")
        }
        // TODO : Add weight to each direction or calculate weights based on all positions, open position and words in certain direction

        // We will save all possible positions for lowest entropies and select single word (for any direction) to put on board
        let mut saved_words: Vec<Word> = Vec::new();

        states
            .iter()
            .filter(|(_, words)| words.len() == entropy_min)
            .for_each(|(_, words)| saved_words.extend(words.clone()));

        let random_words: Vec<Word> = saved_words
            .choose_multiple(&mut thread_rng(), saved_words.len())
            .cloned().collect::<Vec<Word>>();

        return random_words;
    }

    pub fn calculate_entropy_for_acell(
        row: usize, col: usize, idx: usize,
        directional_sequences: HashMap<Direction, String>,
        dictionary: &Trie,
        used: Vec<String>,
        sequence_cache: &mut HashMap<String, HashSet<String>>,
    ) -> Vec<Word> {
        // For each row and col (each cell) traverse the position in all directions
        let mut words: Vec<Word> = Vec::new();

        // From default sequence we might have
        for (direction, sequence) in directional_sequences {
            // for sequences lower than MAX we won't compile
            if (sequence.len() >= MAX) {
                // FIXME : Traversing all positions again is unnecessary
                // for depth in MAX..(sequence.len() + 1) {
                    // FIXME temporary to establish longest word will be selected
                    // let subsequence = &sequence[..depth];
                    let subsequence = &sequence;
                    let trie_search_result = sequence_cache.entry(subsequence.to_string()).or_insert(dictionary.search(subsequence));

                    let words_processed: Vec<Word> = trie_search_result
                        .iter()
                        .filter(|word| !used.contains(word))
                        .map(|word| Word::new(word.clone(), direction, (row, col)))
                        .collect();

                    words.extend(words_processed);
                // }
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