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

fn vigenere_encode(input_file: &str, encoded_file: &str, key: &str) -> io::Result<()> {
    let file = File::open(input_file)?;
    let encoded_path = Path::new(encoded_file);
    let encoded_display = encoded_path.display();

    let mut encoded_file = match File::create(&encoded_path) {
        Err(why) => panic!("couldn't create {}: {}", encoded_display, why),
        Ok(encoded_file) => encoded_file,
    };

    let reader = BufReader::new(file);
    let key_chars = key.chars().collect::<Vec<char>>();

    for line in reader.lines() {
        let line = line?;
        let mut encoded_line = String::new();
        let mut key_index = 0;
        
        for c in line.chars() {
            let key_char = key_chars[key_index % key_chars.len()];
            let encoded_char = encode_char(c, key_char);
            let mut e = c as u32;
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
    let encoded_file = "../text_files/vigenere_cipher/encoded.txt";
    
    let processed_text = process_file(input_file, preprocessed_file, false);
    let key = "агдзя";
    let _ = vigenere_encode(preprocessed_file, encoded_file, key);
    Ok(())
}
