use std::collections::HashMap;
use itertools::Itertools;

pub fn encode(plain: &str) -> String {
    decode(plain).chars()
        .chunks(5)
        .into_iter()
        .map(|x| x.collect::<String>() )
        .collect_vec()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    let new_number = ('0'..='9').map(|char| (char, char));
    let new_letters = ('a'..='z').zip(('a'..='z').rev());
    let new_alphabet = new_letters.chain(new_number).collect::<HashMap<_, _>>();

    cipher.to_lowercase()
        .chars()
        .filter_map(|char| new_alphabet.get(&char))
        .collect()
}
