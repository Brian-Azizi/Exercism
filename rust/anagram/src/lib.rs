use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    for &anagram in possible_anagrams {
        if is_anagram(word.to_lowercase(), anagram.to_lowercase()) {
            result.insert(anagram);
        }
    }

    return result;
}

fn is_anagram(word: String, anagram: String) -> bool {
    if word == anagram {
        return false;
    }
    if sort_word(word) == sort_word(anagram) {
        return true;
    }
    return false;
}

fn sort_word(word: String) -> String {
    let mut sorted_word_vec = word.chars().collect::<Vec<char>>();
    sorted_word_vec.sort_unstable();
    let sorted_word = sorted_word_vec.iter().collect();
    return sorted_word;
}
