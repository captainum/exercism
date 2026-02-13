pub fn number(user_number: &str) -> Option<String> {
    let mut filtered = user_number.chars().filter_map(
        |ch| {
            if ch.is_ascii_digit() {
                ch.to_digit(10)
            } else {
                None
            }
        }
    ).collect::<Vec<u32>>();

    if filtered.len() == 11 && filtered[0] == 1 {
        filtered.remove(0);
    }

    if filtered.len() != 10 {
        return None;
    }

    if !filtered.iter().enumerate().all(
        |(idx, &val)| {
            if idx == 0 || idx == 3 {
                val > 1
            } else {
                true
            }
        }
    ) {
        None
    } else {
        Some(
            filtered.iter().map(
                |&val| {
                    val.to_string().chars().next().unwrap()
                }
            ).collect::<String>()
        )
    }
}