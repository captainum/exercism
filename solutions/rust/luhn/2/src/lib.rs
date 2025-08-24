pub fn is_valid(code: &str) -> bool {
    let v = code.chars().filter(|&ch| ch != ' ').collect::<Vec<char>>();

    v.len() > 1 &&
    v.iter().all(|ch| ch.is_numeric()) &&
    v.iter().rev().enumerate().map(
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