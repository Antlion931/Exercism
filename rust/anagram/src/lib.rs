use std::collections::HashSet;
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams.iter()
        .map(|s| (s.to_lowercase(), *s))
        .filter(|(s, _)| s != &word.to_lowercase())
        .map(|(s, x)| (s.chars().sorted().collect::<String>(), x))
        .filter(|(s, _)| s == &word.to_lowercase().chars().sorted().collect::<String>())
        .map(|(_,s)| s)
        .collect()
}
