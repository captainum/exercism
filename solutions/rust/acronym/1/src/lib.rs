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
                if !first_symbol.is_none() {
                    let result = first_symbol.unwrap().to_ascii_uppercase();
                    first_symbol = None;

                    Some(result)
                } else { None }
            }
            else if first_symbol.is_none() {
                first_symbol = Some(elem);
                None
            }
            else if elem.is_ascii_uppercase() {
                if idx > 0 && !phrase.chars().nth(idx - 1).unwrap().is_ascii_uppercase() {
                    let result = first_symbol.unwrap().to_ascii_uppercase();
                    first_symbol = Some(elem);

                    Some(result)
                } else { None }
            }
            else { None }
        }
    ).collect::<String>();

    if !first_symbol.is_none() {
        result.push(first_symbol.unwrap().to_ascii_uppercase());
    }

    result
}