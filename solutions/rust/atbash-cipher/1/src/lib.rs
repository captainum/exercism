static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars().filter_map(
        |ch| {
            if ch.is_ascii_digit(){
                Some(ch)
            } else if ch.is_ascii_alphabetic() {
                Some(
                    ASCII_LOWER[25 - ASCII_LOWER.iter().position(
                        |&val| val == ch.to_ascii_lowercase()
                    ).unwrap()]
                )
            } else {
                None
            }
        }
    ).collect::<Vec<char>>().chunks(5).enumerate().map(
        |(idx, chars)| {
            let mut result = String::new();
            if idx != 0 {
                result += " ";
            }
            result += chars.iter().collect::<String>().as_str();

            result
        }
    ).collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(
        |ch| {
            if ch.is_ascii_digit() {
                Some(ch)
            } else if ch.is_ascii_alphabetic() {
                Some(
                    ASCII_LOWER[ASCII_LOWER.iter().rev().position(
                        |&val| val == ch.to_ascii_lowercase()
                    ).unwrap()]
                )
            } else {
                None
            }
        }
    ).collect()
}