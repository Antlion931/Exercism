use itertools::Itertools;
use std::collections::HashMap;

fn count_words<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    itertools::sorted(words.iter())
            .dedup_with_count()
            .map(|(n, w)| (*w, n)).collect::<HashMap<_, _>>()
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine = count_words(magazine);

    for (word, note_count) in count_words(note) {
        if magazine.get(word).filter(|magazine_count| **magazine_count >= note_count).is_none() {
            return false;
        }
    }
    true
}
