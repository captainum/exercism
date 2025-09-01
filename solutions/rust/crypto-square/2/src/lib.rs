pub fn encrypt(input: &str) -> String {
    let normalized = input.chars().filter_map(
        |ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else {
                None
            }
        }
    ).collect::<Vec<char>>();

    let mut c = 1;
    let mut r = 1;

    loop {
        if c * r >= normalized.len() {
            break;
        }

        if c > r {
            r += 1;
            c = r;
        } else {
            c += 1;
        }
    }

    let slices = normalized.chunks(c).map(
        |chars| {
            let mut result: Vec<&char> = vec![];
            result.append(&mut chars.iter().collect::<Vec<&char>>());

            result
        }
    );

    let mut squared_slices = Vec::<Vec<char>>::with_capacity(c);
    for _ in 0..c {
        squared_slices.push(Vec::<char>::with_capacity(r));
    }

    for slice in slices {
        let mut left = 0;
        for (idx, &ch) in slice.iter().enumerate() {
            squared_slices.get_mut(idx).unwrap().push(*ch);
            left += 1;
        }
        while left < c {
            squared_slices.get_mut(left).unwrap().push(' ');
            left += 1;
        }
    }
    for squared_slice in squared_slices.iter_mut().take(c - 1) {
        squared_slice.push(' ');
    }

    squared_slices.iter().flatten().collect::<String>()
}