use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

mod utils;
use crate::utils::{preprocess_text, remove_spaces};
use crate::utils::{print_bigram_frequencies, print_bigram_probabilities};
use crate::utils::{print_letters_probabilities, print_letter_frequencies};

fn get_letter_frequency(text: &str) -> HashMap<char, i64> {
    let mut frequencies: HashMap<char, i64> = HashMap::new();

    for c in text.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    frequencies
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


fn compute_h1(letter_frequencies: &HashMap<char, i64>) -> f64 {
    let mut h1 = 0.0;
    let probabilities = count_letters_probabilities(&letter_frequencies);

    for (_key, _value) in probabilities {
        h1 += _value * f64::log2(_value);
    }

    h1 = -h1;
    h1
}

fn get_bigram_frequency(text: &str) -> HashMap<String, i64> {
    let mut frequencies: HashMap<String, i64> = HashMap::new();

    let mut chars = text.chars().peekable();
    while let (Some(curr), Some(&next)) = (chars.next(), chars.peek()) {
        if curr.is_alphabetic() && next.is_alphabetic() {
            let bigram = format!("{}{}", curr.to_lowercase(), next.to_lowercase());
            *frequencies.entry(bigram).or_insert(0) += 1;
        } else if curr.is_alphabetic() && next.is_whitespace() {
            let bigram = format!("{} ", curr.to_lowercase());
            *frequencies.entry(bigram).or_insert(0) += 1;
        } else if curr.is_whitespace() && next.is_alphabetic() {
            let bigram = format!(" {}", next.to_lowercase());
            *frequencies.entry(bigram).or_insert(0) += 1;
        }
    }

    frequencies
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

fn compute_h2(bigram_frequencies: &HashMap<String, i64>) -> f64 {
    let mut h2 = 0.0;
    let probabilities = count_bigram_probabilities(&bigram_frequencies);

    for (_key, _value) in probabilities {
        h2 += _value * f64::log2(_value);
    }
    h2 = -h2/2.0;
    
    h2
}

fn analyze_file(input_file: &str, output_file: &str, with_spaces: bool) -> io::Result<()> {
    let file = File::open(input_file)?;
    let path = Path::new(output_file);
    let display = path.display();
    
    let mut output_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(output_file) => output_file,
    };
    
    let reader = BufReader::new(file);
    let mut text = String::new();
    
    for line in reader.lines() {
        let line = line?;

        if !line.trim().is_empty() {
            let processed_line: Option<String>;

            if with_spaces {
                processed_line = preprocess_text(&line);
            } else {
                processed_line = Some(remove_spaces(&line));
            }
            
            if let Some(processed_line) = processed_line {
                writeln!(output_file, "{}", processed_line)?;
                text.push_str(&processed_line);
            }
        }
    }
    
    let letter_frequencies = get_letter_frequency(&text);
    print_letter_frequencies(&letter_frequencies);
    let letter_prob = count_letters_probabilities(&letter_frequencies);
    print_letters_probabilities(&letter_prob);
    let h1 = compute_h1(&letter_frequencies);
    println!("h1: {}", h1);


    let bigram_frequencies = get_bigram_frequency(&text);
    print_bigram_frequencies(&bigram_frequencies);
    let bigram_prob = count_bigram_probabilities(&bigram_frequencies);
    print_bigram_probabilities(&bigram_prob);
    let h2 = compute_h2(&bigram_frequencies);
    println!("h2: {}", h2);

    println!("File analyzing completed. Processed text saved to {}", display);

    Ok(())
}

fn main() -> io::Result<()> {
    let input_file = "../text_files/entropy/boloto.txt";
    let processed_file = "../text_files/entropy/boloto_processed.txt";
    let without_spaces_file = "../text_files/entropy/boloto_without_spaces.txt";
    let mut with_spaces = true;

    let _ = analyze_file(input_file, processed_file, with_spaces);
    
    with_spaces = false;
    let _ = analyze_file(processed_file, without_spaces_file, with_spaces);

    Ok(())
}