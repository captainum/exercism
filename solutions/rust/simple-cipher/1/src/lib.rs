use rand::prelude::*;

fn is_valid_key(key: &str) -> bool {
   !key.is_empty() && key.chars().all(|ch| ch.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if is_valid_key(key) {
        Some(
            s.chars().zip(key.chars().cycle()).map(
                |(ch, k)| {
                    let ch = ch as u8;
                    let k = k as u8;

                    ((ch + k - 2 * b'a') % 26 + b'a') as char
                }
            ).collect::<String>()
        )
    } else { None }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if is_valid_key(key) {
        Some(
            s.chars().zip(key.chars().cycle()).map(
                |(ch, k)| {
                    let mut ch = ch as u8;
                    let k = k as u8;

                    if ch < k {
                        ch += 26;
                    }

                    (ch - k + b'a') as char
                }
            ).collect::<String>()
        )
    } else { None }
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..100).map(
        |_| rand::rng().sample(rand::distr::Alphabetic).to_ascii_lowercase() as char
    ).collect::<String>();

    let cipher = encode(&key, s).unwrap();

    (key, cipher)
}
