use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut chars = HashSet::<char>::new();
    for ch in candidate.replace("-", "").replace(" ", "").to_lowercase().chars() {
        if chars.contains(&ch) {
            return false;
        }
        chars.insert(ch);
    }

    true
}