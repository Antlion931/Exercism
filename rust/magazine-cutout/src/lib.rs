// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for m in magazine.into_iter() {
        *map.entry(m).or_insert(0) += 1;
    }

    for m in note.into_iter() {
        *map.entry(m).or_insert(0) -= 1;
        if(*map.get(m).unwrap() < 0) {
            return false;
        }
    }

    true
}
