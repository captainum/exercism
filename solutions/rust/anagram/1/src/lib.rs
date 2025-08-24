use std::collections::{HashSet, HashMap};

fn hash_it(word: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();

    word.to_lowercase().chars().for_each(
        |ch| {
            result.insert(ch, *result.get(&ch).unwrap_or(&0) + 1);
        }
    );

    result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let symbols = hash_it(word);

    for &anagram in possible_anagrams {
        if word.to_lowercase() != anagram.to_lowercase() && symbols == hash_it(anagram) {
            result.insert(anagram);
        }
    }

    result
}