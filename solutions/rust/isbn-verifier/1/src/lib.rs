/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    let mut result = isbn.replace("-", "");

    let mut last_is_x = false;
    if result.chars().last().unwrap() == 'X' {
        result.pop();
        last_is_x = true;
    }

    if result.len() != 9 + (!last_is_x as usize) ||
        !result.chars().all(
            |ch| ch.is_numeric()
        ) {
        return false;
    }

    let mut sum = 0;
    for (idx, ch) in result.chars().enumerate() {
        sum += ch.to_digit(10).unwrap() as usize * (10 - idx)
    }

    if last_is_x {
        sum += 10;
    }

    sum % 11 == 0
}
