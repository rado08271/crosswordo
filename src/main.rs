use std::time::SystemTime;
use crate::logic::controller::Controller;
use unidecode::unidecode;
use rand::seq::SliceRandom;
use rand::thread_rng;
mod entities;
mod logic;

static  BOARD: [[char; 5]; 5] = [
    ['?', '?', 'b', '?', 'i'],
    ['?', '?', 'k', '?', '?'],
    ['?', '?', '?', 'e', '?'],
    ['?', '?', '?', '?', '?'],
    ['?', '?', '?', '?', '?']
];

fn read_dictionary(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|word| word.split("\t").nth(0).unwrap())
        .map(String::from)  // make each slice into a string
        .filter(|s| s.len() >= MAX)
        .collect()  // gather them together into a vector
}

const MAX: usize = 3;

fn main() {

    // let dictionary = vec![String::from("space"), String::from("place"), String::from("craze"), String::from("crate"), String::from("state"), String::from("plate"), String::from("blade"), String::from("blato"), String::from("care"), String::from("spar")];
    let mut dictionary: Vec<String> = read_dictionary("./res/en.dr");

    // filter using ruleset
    let filtered_dictionary: Vec<String> = dictionary
        .choose_multiple(&mut thread_rng(), dictionary.len())
        .cloned()
        .map(|stn| unidecode(stn.to_lowercase().as_str()))
        .filter(|e| e.len() >= MAX)
        .collect::<Vec<String>>()[0..].to_vec();

    const ROWS: usize = 4;
    const COLS: usize = 6;

    let mut controller = Controller::new("stolicka", ROWS, COLS, filtered_dictionary);


    // for i in 0..3 {
    let mut i = 0;
    // loop {
    // for _ in 0..20 {
        controller.perform_action();
        // i+=1;
    // }
}