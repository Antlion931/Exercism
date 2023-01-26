use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    
    let words: String = words.chars().map(|c| {
        if c.is_whitespace() || c.is_alphanumeric() || c == '\'' {
            c 
        } else {
            ' '
        }
    }).collect();

    words.to_lowercase()
        .split_whitespace().for_each(|w| {
           *result.entry(w.trim_end_matches('\'').trim_start_matches('\'').to_string()).or_insert(0) += 1;
    });

    result
}

