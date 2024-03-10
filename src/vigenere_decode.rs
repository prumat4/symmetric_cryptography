use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::fs;

mod utils;
use utils::process_file;
use utils::coincidence;
use std::collections::HashMap;

fn divide_into_blocks(text: &str, r: usize) -> Vec<String> {
    let mut blocks = Vec::new();

    for chunk in text.chars().collect::<Vec<char>>().chunks(r) {
        let block: String = chunk.iter().collect();
        blocks.push(block);
    }

    blocks
}

fn compute_r(processed_text: &str, alphabet: &str) -> Option<usize> {
    let alphabet_len = alphabet.len();
    let i_input = coincidence(processed_text, alphabet);
    println!("I for input text (message): {}", i_input);

    let mut closest_r: Option<usize> = None;
    let mut closest_coincidence = f64::MAX;

    for r in 2..=20 {
        let blocks = divide_into_blocks(processed_text, r);

        let mut blocks_coincidence: f64 = 0.0;
        for block in &blocks {
            blocks_coincidence += coincidence(block, alphabet);
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
    let alphabet = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
    let input_file = "../text_files/vigenere_cipher/to_decode/input.txt";
    let preprocessed_file = "../text_files/vigenere_cipher/to_decode/preprocessed.txt";
    let processed_text = process_file(input_file, preprocessed_file, false)?;

    if let Some(r) = compute_r(&processed_text, alphabet) {
        println!("Closest block size to i_m: {}", r);
    }

    Ok(())
}