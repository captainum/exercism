use std::collections::HashSet;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let chars= vec![
        HashSet::<char>::from(['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        HashSet::<char>::from(['D', 'G']),
        HashSet::<char>::from(['B', 'C', 'M', 'P']),
        HashSet::<char>::from(['F', 'H', 'V', 'W', 'Y']),
        HashSet::<char>::from(['K']),
        HashSet::<char>::from(['J', 'X']),
        HashSet::<char>::from(['Q', 'Z']),
    ];

    let scores = Vec::<u64>::from(
        [1, 2, 3, 4, 5, 8, 10, 0]
    );

    word.chars().map(
        |ch| {
            scores[
                chars.iter().position(
                    |chars| {
                        chars.contains(&ch.to_ascii_uppercase())
                    }
                ).unwrap_or(7)
            ]
        }
    ).sum::<u64>()
}
