use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

mod utils;
use crate::utils::{process_file};

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

fn main() -> io::Result<()> {
    let input_file = "../text_files/vigenere_cipher/input.txt";
    let preprocessed_file = "../text_files/vigenere_cipher/preprocessed.txt";
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
            "../text_files/vigenere_cipher/encoded_{}.txt",
            key_size
        );
    
        let _ = vigenere_encode(&processed_text, &encoded_file_name, key);
    }

    Ok(())
}