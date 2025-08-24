pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.ends_with("?");

    let is_capitalized = message.chars().any(
        |ch| {
            ch.is_alphabetic()
        }
    ) && msg == msg.to_uppercase();

    if is_capitalized && is_question {
        "Calm down, I know what I'm doing!"
    }
    else if is_capitalized {
        "Whoa, chill out!"
    }
    else if is_question {
        "Sure."
    }
    else {
        "Whatever."
    }
}