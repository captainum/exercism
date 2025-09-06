use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.chars().map(
        |ch| {
            if ch.is_ascii_punctuation() && ch != '\'' {
                ' '
            } else {
                ch.to_ascii_lowercase()
            }
        }
    ).collect::<String>().split_whitespace().fold(
        HashMap::new(),
        |mut result, word| {
            let string = word
                .trim_start_matches('\'')
                .trim_end_matches('\'')
                .to_string();

            if !string.is_empty() {
                *result.entry(string).or_insert(0) += 1;
            }

            result
        }
    )
}
