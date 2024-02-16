use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn is_cyrillic(c: &char) -> bool {
    (*c as u32) >= 0x0400 && (*c as u32) <= 0x04FF
}

fn preprocess_text(text: &str) -> String {
    let mut processed_text = String::new();

    for c in text.chars() {
        if c.is_alphabetic() && is_cyrillic(&c) {
            processed_text.push(c.to_lowercase().next().unwrap());
        } else if c == ' ' {
            if processed_text.ends_with(' ') {
                continue;
            }
            processed_text.push(c);
        }
    }

    return processed_text;
}

fn get_letter_frequency(text: &str) -> HashMap<char, i64> {
    let mut frequencies: HashMap<char, i64> = HashMap::new();

    for c in text.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    return frequencies;
}

fn main() -> io::Result<()> {
    let file = File::open("../example.txt")?;
    let path = Path::new("../example_processed.txt");
    let display = path.display();
    
    let mut output_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(output_file) => output_file,
    };
    
    let reader = BufReader::new(file);
    let mut text_accumulator = String::new();
    
    for line in reader.lines() {
        let processed_line = preprocess_text(&line?);
        writeln!(output_file, "{}", processed_line)?;
        
        text_accumulator.push_str(&processed_line);
    }
    
    let letter_frequencies = get_letter_frequency(&text_accumulator);

    let mut sorted_keys: Vec<char> = letter_frequencies.keys().cloned().collect();
    sorted_keys.sort();
    for letter in sorted_keys {
        if let Some(&frequency) = letter_frequencies.get(&letter) {
            println!("{}: {}", letter, frequency);
        }
    }

    println!("Text preprocessing completed. Processed text saved to {}", display);

    Ok(())
}
