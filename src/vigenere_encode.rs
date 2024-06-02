use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::fs;

mod utils;
use crate::utils::{process_file};
use utils::coincidence;

fn encode_char(c: char, key_char: char) -> char {
    let mut ans = (c as u32) + (key_char as u32) - 'а' as u32 - 'а' as u32;
    if ans > 31 {
        ans = ans - 31;
    }

    ans = ans + 'а' as u32;
    char::from_u32(ans).unwrap_or('і')
}

fn vigenere_encode(input_text: &str, encoded_file: &str, key: &str) -> io::Result<()> {
    let encoded_path = Path::new(encoded_file);
    let encoded_display = encoded_path.display();
    let mut encoded_file = File::create(&encoded_path)?;

    let key_chars: Vec<char> = key.chars().collect();

    for line in input_text.lines() {
        let mut encoded_line = String::new();
        let mut key_index = 0;

        for c in line.chars() {
            let key_char = key_chars[key_index % key_chars.len()];
            let encoded_char = encode_char(c, key_char);
            encoded_line.push(encoded_char);
            key_index += 1;
        }

        writeln!(encoded_file, "{}", encoded_line)?;
    }

    println!("File encoded successfully. Encoded text saved to {}", encoded_display);

    Ok(())
}

fn i_m_theoretical(probabilities: Vec<f64>) -> f64 {
    let mut i_m = 0.0;
    
    for prob in probabilities {
        i_m += prob * prob;
    }

    i_m
}

fn main() -> io::Result<()> {
    let alphabet = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
    let probabilities: Vec<f64> = vec![
        0.08143, 0.01667, 0.04604, 0.01632, 0.03084, 0.08027, 0.00150, 0.00884,
        0.01507, 0.07563, 0.01200, 0.03374, 0.03952, 0.03270, 0.06503, 0.11143,
        0.02931, 0.04774, 0.05482, 0.06829, 0.02647, 0.00310, 0.00827, 0.00455,
        0.01458, 0.00681, 0.00330, 0.01808, 0.01752, 0.00425, 0.00735, 0.01818,
        0.00036,
    ];

    let input_file = "../../text_files/vigenere_cipher/to_encode//input.txt";
    let preprocessed_file = "../../text_files/vigenere_cipher/to_encode//preprocessed.txt";
    let processed_text = process_file(input_file, preprocessed_file, false)?;
    
    let keys: [(&str, i8); 6] = [
        ("оф", 2),
        ("енз", 3),
        ("ивац", 4),
        ("ежпол", 5),
        ("ьськарозвидка", 14),
        ("дайтидокиевазатридня", 20),
    ];

    for (key, key_size) in keys {
        let encoded_file_name = format!(
            "../../text_files/vigenere_cipher/to_encode//encoded_{}.txt",
            key_size
        );
    
        let _ = vigenere_encode(&processed_text, &encoded_file_name, key);
    }

    println!{"<-- coincidence -->"};
    let coincidence_index: f32 = 1.0 / 33.0;
    println!{"theoretical I_0:  {}", coincidence_index};

    let i_m = i_m_theoretical(probabilities);
    println!{"theoretical I_m:  {}", i_m};

    let i_input = coincidence(&processed_text, alphabet);
    println!{"I for input text (message):  {}", i_input};

    for (key, key_size) in keys {
        let encoded_file_name = format!(
            "../../text_files/vigenere_cipher/to_encode/encoded_{}.txt",
            key_size
        );

        let encoded_text = fs::read_to_string(&encoded_file_name)?;
        let i_encoded = coincidence(&encoded_text, alphabet);
        println!("I for key '{}' (size {}): {}", key, key_size, i_encoded);
    }

    Ok(())
}