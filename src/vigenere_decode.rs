use std::fs;
use fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::collections::HashMap;

mod utils;
use utils::{process_file, coincidence};
use crate::utils::{get_letter_frequency, print_letter_frequencies};

const ALPHABET: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюя";
const PROBABILITIES: [f64; 32] = 
    [ 0.08143, 0.01667, 0.04604, 0.01632, 0.03084, 0.08027, 0.00884,
      0.01507, 0.07563, 0.01200, 0.03374, 0.03952, 0.03270, 0.06503, 0.11143,
      0.02931, 0.04774, 0.05482, 0.06829, 0.02647, 0.00310, 0.00827, 0.00455,
      0.01458, 0.00681, 0.00330, 0.01808, 0.01752, 0.00425, 0.00735, 0.01818,
      0.00036];

fn calculate_expected_i(probabilities: &[f64]) -> f64 {
    probabilities.iter().map(|&p| p.powi(2)).sum()
}


fn divide_into_blocks(text: &str, r: usize) -> Vec<String> {
    (0..r).map(|i| text.chars().skip(i).step_by(r).collect()).collect()
}

fn compute_r(text: &str) -> Option<usize> {
    let expected_i = calculate_expected_i(&PROBABILITIES);
    println!("expected i: {}", expected_i);

    let mut closest_r: Option<usize> = None;
    let mut closest_coincidence = f64::MAX;

    for r in 2..=20 {
        let blocks = divide_into_blocks(text, r);
        
        let mut blocks_coincidence: f64 = 0.0;
        for block in &blocks {
            blocks_coincidence += coincidence(block, &ALPHABET);
        }
        
        let average_coincidence = blocks_coincidence / blocks.len() as f64;
        
        let diff = (expected_i - average_coincidence).abs();
        if diff < closest_coincidence {
            closest_r = Some(r);
            closest_coincidence = diff;
        }
    }

    closest_r
}

fn crack_key_mi(text: &str, r: usize) -> String {
    let blocks = divide_into_blocks(text, r);
    let alphabet: Vec<char> = ALPHABET.chars().collect();
    let alph_size = alphabet.len();
    let mut key = String::new();

    for i in 0..r {
        let mut k_i = 0;
        let mut m_max = 0.0;

        for g in 0..alph_size {
            let mut try_m = 0.0;
            for t in 0..alph_size {
                let shift_index = (t + g) % alph_size;
                let shift_char = alphabet[shift_index];
                let count = blocks[i].matches(shift_char).count() as f64;
                try_m += PROBABILITIES[t] * count;
            }

            if try_m > m_max {
                m_max = try_m;
                k_i = g;
            }
        }

        key.push(alphabet[k_i]);
    }

    key
}

fn main() -> io::Result<()> {
    let input_file = "../text_files/vigenere_cipher/to_decode/input.txt";
    let preprocessed_file = "../text_files/vigenere_cipher/to_decode/preprocessed.txt";
    let text = process_file(input_file, preprocessed_file, false)?;

    if let Some(r) = compute_r(&text) {
        println!("Closest block size to i_m: {}", r);
    }


    // let key = crack_key_mi(&text, 4);

    for r in 2..50 {
        let key = crack_key_mi(&text, r);
        println!("key: {}: {}", r, key);
    }

    Ok(())
}
