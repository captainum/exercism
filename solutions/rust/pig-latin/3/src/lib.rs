pub fn _translate(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }

    static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    if VOWELS.contains(&input.chars().next().unwrap()) ||
        input.starts_with("xr") ||
        input.starts_with("yt") {
        return input.to_string() + "ay";
    }

    if let Some(idx) = input.find("qu") {
        let mut idx2 = None;
        for (i, ch) in input.chars().enumerate() {
            if VOWELS.contains(&ch) {
                idx2 = Some(i);
                break
            }
        }

        match idx2 {
            None => {
                let result = input[idx+2..].to_string();
                return result + "qu";
            }
            Some(i) => {
                if i > idx {
                    let mut result = input[idx+2..].to_string();
                    result.push_str(&input[..=idx+1]);

                    return result + "ay";
                }
            }
        }
    }

    if let Some(idx) = input.find("y") {
        if idx != 0 {
            let mut idx2 = None;
            for (i, ch) in input.chars().enumerate() {
                if VOWELS.contains(&ch) {
                    idx2 = Some(i);
                    break;
                }
            }

            if let Some(i) = idx2 {
                if i > idx {
                    let mut result = input[idx..].to_string();
                    result.push_str(&input[..idx]);

                    return result + "ay";
                }
            } else {
                let mut result = input[idx..].to_string();
                result.push_str(&input[..idx]);

                return result + "ay";
            }
        }
    }

    let mut idx = 0;
    for (i, ch) in input.chars().enumerate() {
        if VOWELS.contains(&ch) {
            idx = i;
            break;
        }
    }

    let mut result = input[idx..].to_string();

    result.push_str(&input[..idx]);
    result + "ay"
}

pub fn translate(input: &str) -> String {
    let mut result = String::new();
    for word in input.split(" ") {
        if !result.is_empty() {
            result += " ";
        }
        result += &_translate(word);
    }

    result
}