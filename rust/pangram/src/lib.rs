const NUMBER_OF_LETTERS_IN_ALPHABET: usize = 26;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut vec_of_chars = sentence.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_uppercase().to_string())
        .collect::<Vec<_>>();

    vec_of_chars.sort_unstable();
    vec_of_chars.dedup();
    vec_of_chars.len() == NUMBER_OF_LETTERS_IN_ALPHABET
}
