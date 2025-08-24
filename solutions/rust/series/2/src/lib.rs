pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|s| s.iter().collect::<String>())
        .collect()
}
