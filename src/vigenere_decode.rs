use std::fs;
use fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::collections::HashMap;

mod utils;
use utils::{process_file, coincidence};

const ALPHABET: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
const PROBABILITIES: [f64; 33] = [
    0.08143, 0.01667, 0.04604, 0.01632, 0.03084, 0.08027, 0.00150, 0.00884,
    0.01507, 0.07563, 0.01200, 0.03374, 0.03952, 0.03270, 0.06503, 0.11143,
    0.02931, 0.04774, 0.05482, 0.06829, 0.02647, 0.00310, 0.00827, 0.00455,
    0.01458, 0.00681, 0.00330, 0.01808, 0.01752, 0.00425, 0.00735, 0.01818,
    0.00036,
];

fn divide_into_blocks(text: &str, r: usize) -> Vec<String> {
    let mut blocks = Vec::new();

    for chunk in text.chars().collect::<Vec<char>>().chunks(r) {
        let block: String = chunk.iter().collect();
        blocks.push(block);
    }

    blocks
}

fn compute_r(processed_text: &str) -> Option<usize> {
    let i_input = coincidence(processed_text, ALPHABET);
    println!("I for input text (message): {}", i_input);

    let mut closest_r: Option<usize> = None;
    let mut closest_coincidence = f64::MAX;

    for r in 2..=20 {
        let blocks = divide_into_blocks(processed_text, r);

        let mut blocks_coincidence: f64 = 0.0;
        for block in &blocks {
            blocks_coincidence += coincidence(block, ALPHABET);
        }

        let average_coincidence = blocks_coincidence / blocks.len() as f64;

        let diff = (i_input - average_coincidence).abs();
        println!("{}", diff);
        if diff < closest_coincidence {
            closest_r = Some(r);
            closest_coincidence = diff;
        }
    }

    closest_r
}

fn main() -> io::Result<()> {
    let input_file = "../text_files/vigenere_cipher/to_decode/input.txt";
    let preprocessed_file = "../text_files/vigenere_cipher/to_decode/preprocessed.txt";
    let processed_text = process_file(input_file, preprocessed_file, false)?;

    if let Some(r) = compute_r(&processed_text) {
        println!("Closest block size to i_m: {}", r);
    }

    Ok(())
}
