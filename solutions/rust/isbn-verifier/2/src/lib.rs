/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let prepared = isbn.replace("-", "");

    prepared.len() == 10 &&
    prepared.chars().enumerate().all(
        |(idx, ch)| ch.is_numeric() || (ch == 'X' && idx == 9)
    ) &&
    prepared.chars().enumerate().map(
        |(idx, ch)| {
            match ch {
                'X' => 10,
                _ => ch.to_digit(10).unwrap() as usize * (10 - idx),
            }
        }
    ).sum::<usize>() % 11 == 0
}
