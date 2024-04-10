use std::fs;
use fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::collections::HashMap;

mod utils;
use utils::{process_file, coincidence};

const ALPHABET1: &str = "абвгдежзийклмнопрстуфхцчшщыьэюя";
const ALPHABET2: &str = "абвгдежзийклмнопрстуфхцчшщьыэюя";
const ALPHABET: &str = ALPHABET2;
const M: usize = ALPHABET.chars().count() * ALPHABET.chars().count();

const MOST_FREQUENT_BIGRAMS: [&str; 5] = ["ст", "но", "то", "на", "ен"];

let mut ring: HashMap<char, usize> = HashMap::new();
let mut bigram_ring: HashMap<String, usize> = HashMap::new();
let mut bigrams: Vec<String> = vec![String::new(); M];

for (pos, char) in ALPHABET.chars().enumerate() {
    ring.insert(char, pos);
}

for (i, char_i) in ALPHABET.chars().enumerate() {
    for (j, char_j) in ALPHABET.chars().enumerate() {
        let idx = i * ALPHABET.chars().count() + j;
        let bigram = format!("{}{}", char_i, char_j);
        bigram_ring.insert(bigram.clone(), idx);
        bigrams[idx] = bigram;
    }
}

const PROBABILITIES: [f64; 32] = 
    [ 0.08143, 0.01667, 0.04604, 0.01632, 0.03084, 0.08027, 0.00884,
      0.01507, 0.07563, 0.01200, 0.03374, 0.03952, 0.03270, 0.06503, 0.11143,
      0.02931, 0.04774, 0.05482, 0.06829, 0.02647, 0.00310, 0.00827, 0.00455,
      0.01458, 0.00681, 0.00330, 0.01808, 0.01752, 0.00425, 0.00735, 0.01818,
      0.00036];
      
const INPUT_FILE: &str = "../text_files/affine_bigram_subs_analysis/input.txt";
const PREPROCESSED_FILE: &str = "../text_files/affine_bigram_subs_analysis/preprocessed.txt";
const DECODED_FILE_PATH: &str = "../text_files/affine_bigram_subs_analysis/decoded.txt";

fn calculate_expected_i(probabilities: &[f64]) -> f64 {
    probabilities.iter().map(|&p| p.powi(2)).sum()
}

fn main() -> io::Result<()> {
    let text = process_file(INPUT_FILE, PREPROCESSED_FILE, false)?;
    println!("Text processing completed.");

    Ok(())
}