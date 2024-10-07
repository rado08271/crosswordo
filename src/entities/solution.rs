use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use std::vec::IntoIter;
use rand::{random, Rng, thread_rng};
use rand::seq::SliceRandom;
use regex::Regex;
use crate::entities::board::Board;
use crate::entities::direction::Direction;
use crate::MAX;

pub struct Solution {
    rows: usize, cols: usize,
    pub locations: HashMap<usize, char>,
    processed: String
}

impl Solution {
    pub fn new(input: &str, rows: usize, cols: usize) -> Solution {
        // The preprocessing of an input removes special characters, whitespaces and numeric characters
        let max_length: usize = ((cols * rows) / 2);
        let output: String = input.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

        if output.len() == 0 {
            panic!("Support for no solution is not available.");
        }

        if output.len() > max_length {
            panic!("The solution you provided is too long, please ensure you solution does not exceeds {}!", max_length);
        }

        Solution {
            rows, cols,
            locations: HashMap::new(),
            processed: output
        }
    }

    pub fn calculate_constraints(&mut self) -> bool {

        // First we devide the whole board into the clusters based on solution length
        let mut clusters: Vec<Vec<(usize, usize)>> = Vec::new();
        let cluster_size: usize = (self.cols * self.rows) / self.processed.len();
        let cluster_remainder = (self.rows * self.cols) % self.processed.len();

        for clusterNumber in 0..self.processed.len() {
            let remainder = (i32::try_from(clusterNumber + cluster_remainder).unwrap() - i32::try_from(self.processed.len()).unwrap() + 1);

            let cluster_positions: Range<usize> = if (remainder <= 0) {
                 (clusterNumber * cluster_size) .. ((clusterNumber + 1) * cluster_size)
            } else {
                let temp_usize_remainder = usize::try_from(remainder).unwrap_or(0);
                (clusterNumber * cluster_size + temp_usize_remainder - 1) .. ((clusterNumber + 1) * cluster_size + temp_usize_remainder)
            };

            clusters.push(
                cluster_positions.map(|position| {
                    let row_idx = position / self.cols;
                    let col_idx = position % self.cols;

                    return (row_idx, col_idx)
                }).collect()
            );
        }

        // for each cluster we are trying to find ideal position
        for cluster in clusters {
            // If there is at least once a situation where there is nowhere to put the solution it cannot be constructed
            if !self.place_in_cluster(cluster) {
                println!("placement seems invalid");
                return false;
            }
        }

        println!("placement seems valid");
        return true;
    }

    // We will go through shuffled cluster and try to put the solution on board
    fn place_in_cluster(&mut self, cluster: Vec<(usize, usize)>) -> bool{
        let shuffle_cluster: Vec<_> = cluster.choose_multiple(&mut thread_rng(), cluster.len()).cloned().collect();
        let cluster_item = self.processed.chars().nth(self.locations.len()).unwrap();

        for (row, col) in shuffle_cluster {
            if self.is_valid_placement(row, col) {
                let current_location: usize = (row * self.cols) + col;
                self.locations.insert(current_location, cluster_item);
                if (!self.revalidate_solution()) {
                    self.locations.remove(&current_location);
                } else {
                    return true;
                }
            }
        }
        return false
    }

    fn is_valid_placement(&self, row: usize, col: usize) -> bool {
        for DIRECTION_MATRIX_CELL in Direction::DIRECTION_MATRIX() {
            if self.is_valid_in_line(1, DIRECTION_MATRIX_CELL, row, col) {
                return true
            }
        }

        return false;
    }

    fn is_valid_in_line(&self, depth: i32, direction_index: Direction, row: usize, col: usize) -> bool {
        let row_direction = (direction_index.getRow() * depth) + i32::try_from(row).unwrap();
        let col_direction = (direction_index.getCol() * depth) + i32::try_from(col).unwrap();
        let directional_idx = (row_direction * i32::try_from(self.cols).unwrap()) + col_direction;

        if row_direction != i32::try_from(row).unwrap() || col_direction != i32::try_from(col).unwrap() {
            if row_direction >= 0 && col_direction >= 0 && row_direction < (i32::try_from(self.rows).unwrap()) && col_direction < (i32::try_from(self.cols).unwrap()) {
                if self.locations.get(&(usize::try_from(directional_idx).unwrap())).is_none() {
                    if depth == i32::try_from(MAX).unwrap() {
                        return true;
                    }
                    return self.is_valid_in_line(depth + 1, direction_index, row, col);
                }
            }
        }

        return false;
    }

    fn revalidate_solution(&self) -> bool {
        let mut is_valid_board = true;
        for (position, c) in &self.locations {
            let row = *position / self.cols;
            let col = *position % self.cols;

            is_valid_board = self.is_valid_placement(row, col);

            if !is_valid_board {
                return false;
            }
        }

        return is_valid_board
    }

    pub fn print_solution_on_board(&mut self) {
        let mut board: Vec<char> = vec!['?'; self.rows * self.cols];
        for (position, c) in &self.locations {
            board[*position] =  *c;
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                let current_position = row * self.cols + col;
                print!("{}\t", board.iter().nth(current_position).unwrap());
            }

            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ascii::AsciiExt;
    use super::*;

    #[test]
    fn test_word() {
        let solution = Solution::new("bike", 5, 5);

        assert_eq!(solution.processed, "bike")
    }

    #[test]
    fn test_case_insensitive() {
        let solution = Solution::new("BIKE", 5, 5);

        assert_eq!(solution.processed, "bike")
    }

    #[test]
    fn test_sentence() {
        let solution = Solution::new("I was driving my bike down the road", 100, 100);

        assert_eq!(solution.processed, "iwasdrivingmybikedowntheroad")
    }

    #[test]
    fn test_numbers() {
        let solution = Solution::new("I was driving my 2 bikes down the 14 road", 100, 100);

        assert_eq!(solution.processed, "iwasdrivingmybikesdowntheroad")
    }

    #[test]
    fn test_special() {
        let solution = Solution::new("I was driving my bike, which is blue, down the road!", 100, 100);

        assert_eq!(solution.processed, "iwasdrivingmybikewhichisbluedowntheroad")
    }

    #[test]
    fn test_utf() {
        let solution = Solution::new("I was driving my Škoda bike, which is blue, down the road!", 100, 100);

        assert_eq!(solution.processed, "iwasdrivingmyškodabikewhichisbluedowntheroad")
    }

    #[test]
    #[should_panic("empty")]
    fn test_empty() {
        let mut solution = Solution::new("      ", 5, 5);
    }

    #[test]
    #[should_panic("empty")]
    fn test_blank() {
        let mut solution = Solution::new("", 5, 5);
    }

    #[test]
    #[should_panic("too long")]
    fn test_too_long() {
        let mut solution = Solution::new("This is looong very long text", 5, 5);
    }

    #[test]
    fn test_is_valid() {
        let mut solution = Solution::new("testtesttest", 4, 6);
        let result = solution.calculate_constraints();

        if result {
            let mut prevItem: usize = 0;
            for item in solution.locations.keys() {
                let range = (prevItem..*item);
                if range.count() > 5 {
                    assert!(false)
                }

                prevItem = *item;
            }
        }

        assert!(true)
    }

}