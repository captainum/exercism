pub fn rotate(input: &str, key: u8) -> String {
    input.chars().map(
        |ch| match ch {
            'a'..='z' => ((ch as u8 - b'a' + key) % 26 + b'a') as char,
            'A'..='Z' => ((ch as u8 - b'A' + key) % 26 + b'A') as char,
            _ => ch,
        }
    ).collect::<String>()
}