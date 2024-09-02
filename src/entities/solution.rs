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
    pub locations: BTreeMap<usize, char>,
    processed: String
}

impl Solution {
    pub fn new(input: &str, rows: usize, cols: usize) -> Solution {
        // The preprocessing of an input removes special characters, whitespaces and numeric characters
        let maxLength: usize = ((cols * rows) / 2);
        let output: String = input.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

        if output.len() == 0 {
            panic!("Support for no solution is not available.");
        }

        if output.len() > maxLength {
            panic!("The solution you provided is too long, please ensure you solution does not exceeds {}!", maxLength);
        }

        Solution {
            rows, cols,
            locations: BTreeMap::new(),
            processed: output
        }
    }

    pub fn calculateConstraints(&mut self) -> bool {

        // First we devide the whole board into the clusters based on solution length
        let mut clusters: Vec<Vec<(usize, usize)>> = Vec::new();
        let clusterSize: usize = (self.cols * self.rows) / self.processed.len();
        let clusterRemainder = (self.rows * self.cols) % self.processed.len();

        for clusterNumber in 0..self.processed.len() {
            let remainder = (i32::try_from(clusterNumber + clusterRemainder).unwrap() - i32::try_from(self.processed.len()).unwrap() + 1);

            let clusterPositions: Range<usize> = if (remainder <= 0) {
                 (clusterNumber * clusterSize ) .. ((clusterNumber + 1) * clusterSize)
            } else {
                let tempUsizeRemainder = usize::try_from(remainder).unwrap_or(0);
                (clusterNumber * clusterSize + tempUsizeRemainder - 1) .. ((clusterNumber + 1) * clusterSize + tempUsizeRemainder)
            };

            clusters.push(
                clusterPositions.map(|position| {
                    let rowIdx = position / self.cols;
                    let colIdx = position % self.cols;

                    return (rowIdx, colIdx)
                }).collect()
            );
        }

        // for each cluster we are trying to find ideal position
        for cluster in clusters {
            // If there is at least once a situation where there is nowhere to put the solution it cannot be constructed
            if !self.placeInCluster(cluster) {
                println!("placement seems invalid");
                return false;
            }
        }

        println!("placement seems valid");
        return true;
    }

    // We will go through shuffled cluster and try to put the solution on board
    fn placeInCluster(&mut self, cluster: Vec<(usize, usize)>) -> bool{
        let shuffleCluster: Vec<_> = cluster.choose_multiple(&mut thread_rng(), cluster.len()).cloned().collect();
        let clusterItem = self.processed.chars().nth(self.locations.len()).unwrap();

        for (row, col) in shuffleCluster {
            if self.isValidPlacement(row, col) {
                let currentLocation: usize = (row * self.cols) + col;
                self.locations.insert(currentLocation, clusterItem);
                if (!self.revalidateSolution()) {
                    self.locations.remove(&currentLocation);
                } else {
                    return true;
                }
            }
        }
        return false
    }

    fn isValidPlacement(&self, row: usize, col: usize) -> bool {
        let currentLocation: usize = (row * self.cols) + col;

        for DIRECTION_MATRIX_CELL in Direction::DIRECTION_MATRIX() {
            if self.isValidInLine(1, DIRECTION_MATRIX_CELL, row, col) {
                return true
            }
        }

        return false;
    }

    fn isValidInLine(&self, depth: i32, directionIndex: Direction, row: usize, col: usize) -> bool {
        let rowDirection = (directionIndex.getRow() * depth) + i32::try_from(row).unwrap();
        let colDirection = (directionIndex.getCol() * depth) + i32::try_from(col).unwrap();
        let directionalIdx = (rowDirection * i32::try_from(self.cols).unwrap()) + colDirection;

        if rowDirection != i32::try_from(row).unwrap() || colDirection != i32::try_from(col).unwrap() {
            if rowDirection >= 0 && colDirection >= 0 && rowDirection < (i32::try_from(self.rows).unwrap()) && colDirection < (i32::try_from(self.cols).unwrap()) {
                if self.locations.get(&(usize::try_from(directionalIdx).unwrap())).is_none() {
                    if depth == i32::try_from(MAX).unwrap() {
                        return true;
                    }
                    return self.isValidInLine(depth + 1, directionIndex, row, col);
                }
            }
        }

        return false;
    }

    fn revalidateSolution(&self) -> bool {
        let mut isValidBoard = true;
        for (position, c) in &self.locations {
            let row = *position / self.cols;
            let col = *position % self.cols;

            isValidBoard = self.isValidPlacement(row, col);

            if !isValidBoard {
                return false;
            }
        }

        return isValidBoard
    }

    pub fn printSolutionOnBoard(&mut self) {
        let mut board: Vec<char> = vec!['?'; self.rows * self.cols];
        for (position, c) in &self.locations {
            board[*position] =  *c;
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                let currentPosition = row * self.cols + col;
                print!("{}\t", board.iter().nth(currentPosition).unwrap());
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
        let result = solution.calculateConstraints();

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