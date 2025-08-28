use std::collections::HashSet;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut chars= Vec::<HashSet<char>>::new();
    chars.push(HashSet::<char>::from(['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']));
    chars.push(HashSet::<char>::from(['D', 'G']));
    chars.push(HashSet::<char>::from(['B', 'C', 'M', 'P']));
    chars.push(HashSet::<char>::from(['F', 'H', 'V', 'W', 'Y']));
    chars.push(HashSet::<char>::from(['K']));
    chars.push(HashSet::<char>::from(['J', 'X']));
    chars.push(HashSet::<char>::from(['Q', 'Z']));

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
