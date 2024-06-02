use std::fs;
use fs::File;
use std::io::{self, Write};
use std::path::Path;

mod utils;
use utils::{process_file, coincidence};

const ALPHABET: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюя";
const PROBABILITIES: [f64; 32] = 
    [ 0.08143, 0.01667, 0.04604, 0.01632, 0.03084, 0.08027, 0.00884,
      0.01507, 0.07563, 0.01200, 0.03374, 0.03952, 0.03270, 0.06503, 0.11143,
      0.02931, 0.04774, 0.05482, 0.06829, 0.02647, 0.00310, 0.00827, 0.00455,
      0.01458, 0.00681, 0.00330, 0.01808, 0.01752, 0.00425, 0.00735, 0.01818,
      0.00036];

const INPUT_FILE: &str = "../../text_files/vigenere_cipher/to_decode/input.txt";
const PREPROCESSED_FILE: &str = "../../text_files/vigenere_cipher/to_decode/preprocessed.txt";
const DECODED_FILE_PATH: &str = "../../text_files/vigenere_cipher/to_decode/decoded.txt";

fn calculate_expected_i(probabilities: &[f64]) -> f64 {
    probabilities.iter().map(|&p| p.powi(2)).sum()
}

fn divide_into_blocks(text: &str, r: usize) -> Vec<String> {
    // love it :))
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

fn find_most_frequent_letter() -> char {
    PROBABILITIES.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(idx, _)| ALPHABET.chars().nth(idx).unwrap_or_default()) 
        .unwrap_or_default()
}

fn determine_key_characters(text: &str, r: usize) -> String {
    let blocks = divide_into_blocks(text, r);
    let most_frequent_language_letter = find_most_frequent_letter();
    let most_frequent_language_letter_index = ALPHABET.chars().position(|c| c == most_frequent_language_letter).unwrap_or_default();
    let mut key = String::with_capacity(r);

    for block in blocks {
        let mut letter_frequencies = std::collections::HashMap::new();
        for letter in block.chars() {
            *letter_frequencies.entry(letter).or_insert(0) += 1;
        }

        if let Some((&most_frequent_block_letter, _)) = letter_frequencies.iter().max_by_key(|&(_, &count)| count) {
            let block_letter_index = ALPHABET.chars().position(|c| c == most_frequent_block_letter).unwrap_or_default();
            let shift = (block_letter_index + ALPHABET.len() - most_frequent_language_letter_index) % ALPHABET.len();
            key.push(ALPHABET.chars().nth(shift).unwrap_or('а'));
        } else {
            key.push('а');
        }
    }

    key
}

fn crack_key_mi(text: &str, key_length: usize) -> String {
    let blocks = divide_into_blocks(text, key_length);
    let alphabet: Vec<char> = ALPHABET.chars().collect();
    let mut key = String::new();

    for block_index in 0..key_length {
        let mut max_shift = 0;
        let mut max_m = 0.0;

        for g in 0..alphabet.len() {
            let mut current_m = 0.0;
            for t in 0..alphabet.len() {
                let shift_index = (t + g) % alphabet.len();
                let shifted_char = alphabet[shift_index];
                let char_count = blocks[block_index].matches(shifted_char).count() as f64;
                current_m += PROBABILITIES[t] * char_count;
            }

            if current_m > max_m {
                max_m = current_m;
                max_shift = g;
            }
        }

        key.push(alphabet[max_shift]);
    }

    key
}

fn decode_and_write(text: &str, key: &str) -> io::Result<()> {
    let alphabet: Vec<char> = ALPHABET.chars().collect();
    let mut decode_text = String::new();
    let key_length = key.chars().count();

    for (i, c) in text.chars().enumerate() {
        if let Some(text_index) = alphabet.iter().position(|&r| r == c) {
            let key_char = key.chars().nth(i % key_length).unwrap();
            if let Some(key_index) = alphabet.iter().position(|&r| r == key_char) {
                let decode_char_index = (text_index + alphabet.len() - key_index) % alphabet.len();
                decode_text.push(alphabet[decode_char_index]);
            } else {
                decode_text.push(c);
            }
        } else {
            decode_text.push(c);
        }
    }

    let path = Path::new(DECODED_FILE_PATH);
    let mut file = File::create(&path)?;
    file.write_all(decode_text.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let text = process_file(INPUT_FILE, PREPROCESSED_FILE, false)?;
    println!("Text processing completed.");

    let r = compute_r(&text).expect("Failed to compute key length.");
    println!("Optimal key length (r) found: {}", r);

    let key_frequency = determine_key_characters(&text, r);
    println!("Key determined by frequency method: {}", key_frequency);

    let key_mi = crack_key_mi(&text, r);
    println!("Key determined by m_i method: {}", key_mi);

    decode_and_write(&text, &key_mi)?;
    println!("Text deciphered using m_i method key and written to file.");

    Ok(())
}
