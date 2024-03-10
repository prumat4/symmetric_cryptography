use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use File;
use Path;

pub fn is_cyrillic(c: &char) -> bool {
    (*c as u32) >= 0x0400 && (*c as u32) <= 0x04FF
}

pub fn preprocess_text(text: &str) -> Option<String> {
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

    if processed_text.is_empty() {
        None
    } else {
        Some(processed_text)
    }
}

pub fn remove_spaces(text: &str) -> String {
    text.chars().filter(|&c| c != ' ' && c != '\n' && c != '\0').collect()
}

pub fn print_bigram_frequencies(bigram_frequencies: &HashMap<String, i64>) {
    let mut letters: Vec<char> = bigram_frequencies.keys().flat_map(|s| s.chars()).collect::<Vec<char>>();
    
    letters.sort();
    letters.dedup();
    println!();
    print!("  |");
    for l in &letters {
        print!("   {} |", l);
    }
    println!();

    for _i in 1..208 {
        print!("_");
    }
    println!();

    for l in &letters {
        print!(" {}|", l);
        for c in &letters {
            let key = format!("{}{}", l, c);
            match bigram_frequencies.get(&key) {
                Some(&frequency) => print!("{:>5}", frequency),
                None => print!("{:>5}", 0),
            }
            print!("|");
        }
        println!();
    }
    println!();
}

pub fn print_bigram_probabilities(bigram_frequencies: &HashMap<String, f64>) {
    let mut letters: Vec<char> = bigram_frequencies.keys().flat_map(|s| s.chars()).collect::<Vec<char>>();
    
    letters.sort();
    letters.dedup();
    println!();
    print!("  |");
    for l in &letters {
        print!("   {} |", l);
    }
    println!();
    
    for _ in 1..208 {
        print!("_");
    }
    println!();

    for l in &letters {
        print!(" {}|", l);
        for c in &letters {
            let key = format!("{}{}", l, c);
            match bigram_frequencies.get(&key) {
                Some(&frequency) => print!("{:>5.3}", frequency),
                None => print!("{:>5.3}", 0.0),
            }
            print!("|");
        }
        println!();
    }
    println!();
}

pub fn print_letters_probabilities(probabilities: &HashMap<char, f64>) {
    let mut sorted_probabilities: Vec<(&char, &f64)> = probabilities.iter().collect();
    sorted_probabilities.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
    for (&letter, &probability) in sorted_probabilities {
        println!("{}: {}", letter, probability);
    }
    
    println!();
}

pub fn print_letter_frequencies(letter_frequencies: &HashMap<char, i64>) {
    let mut sorted_frequencies: Vec<(&char, &i64)> = letter_frequencies.iter().collect();
    sorted_frequencies.sort_by_key(|&(_, frequency)| *frequency);
    
    for (&letter, &frequency) in sorted_frequencies.iter().rev() {
        println!("{}: {}", letter, frequency);
    }
    
    println!();
}

pub fn process_file(input_file: &str, output_file: &str, with_spaces: bool) -> io::Result<String> {
    let file = File::open(input_file)?;
    let path = Path::new(output_file);
    let display = path.display();
    
    let mut output_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(output_file) => output_file,
    };
    
    let reader = BufReader::new(file);
    let mut processed_text = String::new();
    
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
                processed_text.push_str(&processed_line);
            }
        }
    }
    
    Ok(processed_text)
}

pub fn coincidence(input_text: &str, alphabet: &str) -> f64 {
    let text: Vec<char> = input_text.chars().collect();
    let mut sum: usize = 0;

    for c in alphabet.chars() {
        let occurrences = text.iter().filter(|&&x| x == c).count();
        sum = sum.checked_add(occurrences.checked_mul(occurrences.checked_sub(1).unwrap_or(0)).unwrap_or(0)).unwrap_or(0);
    }

    let text_len = text.len();
    let denominator = (text_len.checked_mul(text_len.checked_sub(1).unwrap_or(0)).unwrap_or(0)) as f64;

    (sum as f64) / denominator
}