pub fn encode(source: &str) -> String {
    let mut result = String::new();

    let mut cur: Option<char> = None;
    let mut sum = 0;
    for ch in source.chars() {
        if cur.is_none() {
            cur = Some(ch);
            sum = 1;
        }
        else {
            if ch != cur.unwrap() {
                if sum != 1 {
                    result.push_str(&format!("{}{}", sum, cur.unwrap()));
                }
                else {
                    result.push(cur.unwrap());
                }

                cur = Some(ch);
                sum = 1;
            }
            else {
                sum += 1;
            }
        }
    }

    if let Some(ch) = cur {
        if sum != 1 {
            result.push_str(&format!("{}{}", sum, ch));
        }
        else {
            result.push(cur.unwrap());
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();

    let mut sum = 0;

    for ch in source.chars() {
        if ch.is_numeric() {
            sum *= 10;
            sum += ch.to_digit(10).unwrap() as usize;
        }
        else {
            if sum == 0 {
                sum = 1;
            }

            result.push_str(ch.to_string().repeat(sum).as_str());
            sum = 0;
        }
    }

    result
}