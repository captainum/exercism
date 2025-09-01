/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

use std::cmp;

fn is_coprime(a: i32, b: i32) -> bool {
    for i in 2..=cmp::min(a, b) {
        if a % i == 0 && b % i == 0 {
            return false;
        }
    }

    true
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, mut a: i32, mut b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    a %= 26;
    b %= 26;

    Ok(
        plaintext.chars().filter_map(
            |ch| {
                if ch.is_ascii_alphabetic() {
                    let pos = ASCII_LOWER.iter().position(
                        |&val| val == ch.to_ascii_lowercase()
                    ).unwrap();
                    Some(
                        ASCII_LOWER.iter().nth((a as usize * pos + b as usize) % 26).unwrap().clone()
                    )
                } else if ch.is_numeric() {
                    Some(ch)
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
    )
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, mut a: i32, mut b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    a %= 26;
    b %= 26;

    let mut mmi = 1;
    
    loop {
        if a * mmi % 26 == 1 {
            break;
        }
        mmi += 1;
    }

    Ok(
        ciphertext.chars().filter_map(
            |ch| {
                if ch.is_ascii_alphabetic() {
                    let mut pos = ASCII_LOWER.iter().position(
                        |&val| val == ch.to_ascii_lowercase()
                    ).unwrap() as i32;

                    pos -= b;

                    if pos < 0 {
                        pos += 26;
                    }
                    Some(
                        ASCII_LOWER.iter().nth(
                            mmi as usize * pos as usize % 26
                        ).unwrap().clone()
                    )
                } else if ch.is_numeric() {
                    Some(ch)
                } else {
                    None
                }
            }
        ).collect()
    )
}
