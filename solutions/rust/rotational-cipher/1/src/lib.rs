static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

pub fn rotate(input: &str, key: u8) -> String {
    input.chars().map(
        |ch| {
            if ch.is_ascii_alphabetic() {
                let is_upper = ch.is_ascii_uppercase();
                let pos = ASCII_LOWER.iter().position(
                    |&val| val == ch.to_ascii_lowercase()
                ).unwrap();
                let new_pos = (pos + key as usize) % 26;
                
                let result = ASCII_LOWER[new_pos];
                
                if is_upper {
                    result.to_ascii_uppercase()
                } else {
                    result
                }
            } else {
                ch
            }
        }
    ).collect::<String>()
}