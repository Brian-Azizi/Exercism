// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();
    for word in magazine {
        let word_counter = magazine_map.entry(word).or_insert(0);
        *word_counter += 1;
    }

    for word in note {
        let word_counter = magazine_map.entry(word).or_insert(-1);
        *word_counter -= 1;
    }

    magazine_map.iter().all(|(key, item)| *item >= 0)
}
