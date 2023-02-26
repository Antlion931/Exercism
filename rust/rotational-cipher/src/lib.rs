use std::collections::HashMap;

pub fn rotate(input: &str, key: i8) -> String {
    let key = key.rem_euclid(26) as usize;

    let new_small_letters = ('a'..='z').zip(('a'..='z').cycle().skip(key));
    let new_big_letters = new_small_letters.clone().map(|(x, y)| (x.to_ascii_uppercase(), y.to_ascii_uppercase()));
    let new_alphabet = new_small_letters.chain(new_big_letters).collect::<HashMap<_,_>>();

    input.chars().map(|char| new_alphabet.get(&char).cloned().unwrap_or(char)).collect()
}
