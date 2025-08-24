pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return Vec::new();
    }

    let mut result: Vec<String> = Vec::new();

    for i in 0 .. digits.len() - len + 1 {
        result.push(digits[i .. i + len].to_string());
    }

    result
}
