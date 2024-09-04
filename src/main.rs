use std::time::SystemTime;
use crate::logic::controller::Controller;
use unidecode::unidecode;

mod entities;
mod utils;
mod logic;

static  BOARD: [[char; 5]; 5] = [
    ['?', '?', 'b', '?', 'i'],
    ['?', '?', 'k', '?', '?'],
    ['?', '?', '?', 'e', '?'],
    ['?', '?', '?', '?', '?'],
    ['?', '?', '?', '?', '?']
];

fn readDictionary(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|word| word.split("\t").nth(0).unwrap())
        .map(String::from)  // make each slice into a string
        .filter(|s| s.len() >= 2)
        .collect()  // gather them together into a vector
}

// TODO : We should also consider max length which is the longest word
const MAX: usize = 3;

fn main() {

    // let dictionary = vec![String::from("space"), String::from("place"), String::from("craze"), String::from("crate"), String::from("state"), String::from("plate"), String::from("blade"), String::from("blato"), String::from("care"), String::from("spar")];
    let dictionary: Vec<String> = readDictionary("./res/en.dr");
    // filter using ruleset
    println!("reading dictionary started");
    let started = SystemTime::now();
    let filteredDictionary: Vec<String> = dictionary.iter().map(|stn| unidecode(stn.to_lowercase().as_str())).filter(|e| e.len() > MAX).collect();

    println!("size of dic {} read in {}", filteredDictionary.len(), started.elapsed().unwrap().as_millis());
    const ROWS: usize = 10;
    const COLS: usize = 10;
    // const ROWS: usize = 6;
    // const COLS: usize = 6;

    let mut controller = Controller::new("bola to celkom pekna stolicka vsak", ROWS, COLS, filteredDictionary);
    // let mut controller = GameController::new("bola celkom pekna", ROWS, COLS, filteredDictionary);
    println!("processed solution in {}ms", started.elapsed().unwrap().as_millis());


    // for i in 0..3 {
    let mut i = 0;
    loop {
        println!("========== {} ==========\t\t{}ms", i, started.elapsed().unwrap().as_millis());
        controller.performAction();
        i+=1;
        controller.printBoard();
    }
    // let mut board = &mut Board::new(ROWS, COLS);
    //     board.printBoard();


}