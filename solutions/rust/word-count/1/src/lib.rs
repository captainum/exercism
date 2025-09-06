use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::<String, u32>::new();

    for word in words.chars().filter_map(
        |ch| {
            if ch.is_ascii_punctuation() && ch != '\'' {
                Some(' ')
            } else {
                Some(ch.to_ascii_lowercase())
            }
        }
    ).collect::<String>().split_whitespace() {
        let string = word.trim_start_matches('\'').trim_end_matches('\'').to_string();
        if !string.is_empty() {
            *result.entry(string).or_insert(0) += 1;
        }
    }

    result
}
