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

fn get_bigram_frequency(text: &str) -> HashMap<String, i64> {
    let mut frequencies: HashMap<String, i64> = HashMap::new();

    let mut chars = text.chars().peekable();
    while let (Some(curr), Some(next)) = (chars.next(), chars.peek().cloned()) {
        if curr.is_alphabetic() && next.is_alphabetic() {
            let bigram = format!("{}{}", curr.to_lowercase().next().unwrap(), next.to_lowercase().next().unwrap());
            *frequencies.entry(bigram).or_insert(0) += 1;
        }
    }

    return frequencies;
}

fn print_letter_frequencies(letter_frequencies: &HashMap<char, i64>) {
    let mut sorted_letters: Vec<char> = letter_frequencies.keys().cloned().collect();
    sorted_letters.sort();
    for letter in sorted_letters {
        if let Some(&frequency) = letter_frequencies.get(&letter) {
            println!("{}: {}", letter, frequency);
        }
    }
}

fn print_bigram_frequencies(bigram_frequencies: &HashMap<String, i64>) {
    let mut sorted_bigrams: Vec<String> = bigram_frequencies.keys().cloned().collect();
    sorted_bigrams.sort();
    for bigram in sorted_bigrams {
        if let Some(&frequency) = bigram_frequencies.get(&bigram) {
            println!("{}: {}", bigram, frequency);
        }
    }
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
    let mut text = String::new();
    
    for line in reader.lines() {
        let processed_line = preprocess_text(&line?);
        writeln!(output_file, "{}", processed_line)?;
        
        text.push_str(&processed_line);
    }
    
    let letter_frequencies = get_letter_frequency(&text);
    print_letter_frequencies(&letter_frequencies);

    let bigram_frequencies = get_bigram_frequency(&text);
    print_bigram_frequencies(&bigram_frequencies);

    println!("Text preprocessing completed. Processed text saved to {}", display);

    Ok(())
}
