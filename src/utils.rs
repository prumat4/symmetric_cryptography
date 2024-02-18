use std::collections::HashMap;

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
    text.chars().filter(|&c| c != ' ').collect()
}

pub fn print_bigram_frequencies(bigram_frequencies: &HashMap<String, i64>) {
    let mut letters: Vec<char> = bigram_frequencies.keys()
                                                .flat_map(|s| s.chars())
                                                .collect::<Vec<char>>();
    
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
    let mut letters: Vec<char> = bigram_frequencies.keys()
                                                .flat_map(|s| s.chars())
                                                .collect::<Vec<char>>();
    
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