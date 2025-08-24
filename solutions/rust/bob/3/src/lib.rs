pub fn reply(message: &str) -> &str {
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    
    let is_question = message.trim().ends_with("?");
    
    let is_silence = message.chars().filter(
        |&ch| {
            !ch.is_whitespace() && !ch.is_control()
        }
    ).collect::<String>().is_empty();

    let message_filtered: String = message[..message.len() - 1].chars().filter(|&ch| {
        !ch.is_numeric() && !ch.is_whitespace() && !ch.is_control() && !ch.is_ascii_punctuation()
    }).collect();
    let is_capitalized = !message_filtered.is_empty() && (&message_filtered.to_uppercase() == &message_filtered.to_string());
    
    if is_capitalized && is_question {
        "Calm down, I know what I'm doing!"
    }
    else if is_capitalized {
        "Whoa, chill out!"
    }
    else if is_question {
        "Sure."
    }
    else if is_silence {
        "Fine. Be that way!"
    }
    else {
        "Whatever."
    }
}