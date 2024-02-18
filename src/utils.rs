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