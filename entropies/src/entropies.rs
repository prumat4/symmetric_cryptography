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

    processed_text
}

fn get_letter_frequency(text: &str) -> HashMap<char, i64> {
    let mut frequencies: HashMap<char, i64> = HashMap::new();

    for c in text.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    frequencies
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

    frequencies
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

// looks awful btw
// mb try print like a table (or even create a separate text file with this table)?
fn print_bigram_frequencies(bigram_frequencies: &HashMap<String, i64>) {
    let mut sorted_bigrams: Vec<String> = bigram_frequencies.keys().cloned().collect();
    sorted_bigrams.sort();
    for bigram in sorted_bigrams {
        if let Some(&frequency) = bigram_frequencies.get(&bigram) {
            println!("{}: {}", bigram, frequency);
        }
    }
}

fn letters_count(letter_frequencies: &HashMap<char, i64>) -> i64 {
    let mut count = 0;
    for (_key, _value) in letter_frequencies {
        count += _value;
    }

    count
}

fn count_letters_probabilities(letter_frequencies: &HashMap<char, i64>) -> HashMap<char, f64> {
    let mut probabilities: HashMap<char, f64> = HashMap::new();
    let number_of_characters = letters_count(letter_frequencies) as f64;

    for (_key, _value) in letter_frequencies {
        probabilities.insert(*_key, (*_value as f64) / number_of_characters);
    }
    probabilities
}

fn print_letters_probabilities(probabilities: &HashMap<char, f64>) {
    let mut sorted_probabilities: Vec<char> = probabilities.keys().cloned().collect();
    sorted_probabilities.sort();
    for letter in sorted_probabilities {
        if let Some(&frequency) = probabilities.get(&letter) {
            println!("{}: {}", letter, frequency);
        }
    }
}

fn compute_h1(letter_frequencies: &HashMap<char, i64>) -> f64 {
    let mut h1 = 0.0;
    let probabilities = count_letters_probabilities(&letter_frequencies);

    for (_key, _value) in probabilities {
        h1 += _value * f64::log2(_value);
    }

    h1 = -h1;
    h1
}

fn bigram_count(bigram_frequencies: &HashMap<String, i64>) -> i64 {
    let mut count = 0;
    for (_key, _value) in bigram_frequencies {
        count += _value;
    }

    count
}

fn count_bigram_probabilities(bigram_frequencies: &HashMap<String, i64>) -> HashMap<String, f64> {
    let mut probabilities: HashMap<String, f64> = HashMap::new();
    let number_of_bigrams = bigram_count(bigram_frequencies) as f64;

    for (_key, _value) in bigram_frequencies {
        probabilities.insert(_key.clone(), (*_value as f64) / number_of_bigrams);
    }

    probabilities
}

fn print_bigram_probabilities(probabilities: &HashMap<String, f64>) {
    let mut sorted_probabilities: Vec<String> = probabilities.keys().cloned().collect();
    sorted_probabilities.sort();
    for bigram in sorted_probabilities {
        if let Some(&frequency) = probabilities.get(&bigram) {
            println!("{}: {}", bigram, frequency);
        }
    }
}

fn compute_h2(bigram_frequencies: &HashMap<String, i64>) -> f64 {
    let mut h2 = 0.0;
    let probabilities = count_bigram_probabilities(&bigram_frequencies);

    for (_key, _value) in probabilities {
        h2 += _value * f64::log2(_value);
    }

    h2 = -h2/2.0;
    h2
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
    // print_letter_frequencies(&letter_frequencies);

    // let prob = count_probabilities(&letter_frequencies);
    // print_probabilities(&prob);

    let h1 = compute_h1(&letter_frequencies);
    println!("h1: {}", h1);

    let bigram_frequencies = get_bigram_frequency(&text);
    // print_bigram_frequencies(&bigram_frequencies);
    let h2 = compute_h2(&bigram_frequencies);
    println!("h2: {}", h2);

    println!("Text preprocessing completed. Processed text saved to {}", display);

    Ok(())
}
