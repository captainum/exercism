#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        Err(Error::SpanTooLong)
    } else if string_digits.len() == 0 || span == 0 {
        Ok(1)
    } else if let Some(ch) = string_digits.chars().find(|ch| !ch.is_numeric()) {
        Err(Error::InvalidDigit(ch))
    } else {
        Ok(
            string_digits.chars().collect::<Vec<char>>().windows(span).map(
                |ch| {
                    ch.iter().map(
                        |ch| {
                            ch.to_digit(10).unwrap() as u64
                        }
                    ).product()
                }
            ).max().unwrap()
        )
    }
}
