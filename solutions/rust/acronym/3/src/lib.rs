pub fn abbreviate(phrase: &str) -> String {
    let mut first_symbol: Option<char> = None;

    let mut result = phrase.chars().enumerate().filter_map(
        |(idx, mut elem)| {
            if elem == ' ' || elem == '-' {
                elem = ' ';
            }

            if elem.is_ascii_punctuation() {
                None
            }
            else if elem == ' ' {
                match (first_symbol) {
                    Some(ch) => {
                        let result = ch.to_ascii_uppercase();
                        first_symbol = None;

                        Some(result)
                    },
                    _ => None,
                }
            }
            else {
                if let Some(ch) = first_symbol {
                    if elem.is_ascii_uppercase() {
                        if idx > 0 && !phrase.chars().nth(idx - 1).unwrap().is_ascii_uppercase() {
                            let result = ch.to_ascii_uppercase();
                            first_symbol = Some(elem);

                            Some(result)
                        } else { None }
                    }
                    else { None }
                }
                else {
                    first_symbol = Some(elem);
                    None
                }
            }
        }
    ).collect::<String>();

    if let Some(ch) = first_symbol {
        result.push(ch.to_ascii_uppercase());
    }

    result
}