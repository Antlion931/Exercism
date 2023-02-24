use itertools::Itertools;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine = magazine.iter().counts();

    for (word, note_count) in note.iter().counts() {
        if magazine.get(&word).filter(|magazine_count| **magazine_count >= note_count).is_none() {
            return false;
        }
    }
    true
}
