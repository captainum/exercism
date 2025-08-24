pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let garden_size: usize = garden.len();
    let line_size: usize = garden.first().unwrap().len();

    let mut result_int = vec![
        vec![0; line_size]; garden_size
    ];

    for (idx, &line) in garden.iter().enumerate() {
        for (idx2, ch) in line.chars().enumerate() {
            if ch == '*' {
                let has_upper = idx > 0;
                let has_lower = idx < garden_size - 1;
                let has_left = idx2 > 0;
                let has_right = idx2 < line_size - 1;

                if has_upper {
                    result_int[idx - 1][idx2] += 1;
                }
                if has_lower {
                    result_int[idx + 1][idx2] += 1;
                }
                if has_left {
                    result_int[idx][idx2 - 1] += 1;
                }
                if has_right {
                    result_int[idx][idx2 + 1] += 1;
                }
                if has_upper && has_left {
                    result_int[idx - 1][idx2 - 1] += 1;
                }
                if has_upper && has_right {
                    result_int[idx - 1][idx2 + 1] += 1;
                }
                if has_lower && has_left {
                    result_int[idx + 1][idx2 - 1] += 1;
                }
                if has_lower && has_right {
                    result_int[idx + 1][idx2 + 1] += 1;
                }
            }
        }
    }

    let mut result = Vec::with_capacity(garden_size);

    for (idx, &line) in garden.iter().enumerate() {
        let mut line_str = String::new();
        line_str.reserve(line.len());

        for (idx2, ch) in line.chars().enumerate() {
            if ch == '*' {
                line_str.push(ch);
            }
            else if result_int[idx][idx2] == 0 {
                line_str.push(' ');
            }
            else {
                line_str.push_str(result_int[idx][idx2].to_string().as_str());
            }
        }

        result.push(line_str);
    }

    result
}