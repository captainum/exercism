pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let filtered = self.code.replace(" ", "");

        filtered.len() > 1 &&
        filtered.chars().all(char::is_numeric) &&
        filtered.chars().rev().enumerate().map(
            |(idx, ch)| {
                let mut result = ch.to_digit(10).unwrap();
                if idx % 2 != 0 {
                    result *= 2;
                    if result > 9 {
                        result - 9
                    } else { result }
                } else { result }
            }
        ).sum::<u32>() % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Self { code: input.to_string() }
    }
}
