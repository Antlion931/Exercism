pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        vec!["".to_string(); digits.len() + 1]
    } else {
        digits.chars().collect::<Vec<_>>().windows(len).map(|c| c.iter().collect::<String>()).collect()
    }
}
