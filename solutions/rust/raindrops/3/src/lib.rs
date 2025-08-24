pub fn raindrops(n: u32) -> String {
    let values = vec![
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong"),
    ];

    let mut result = String::new();

    for &(key, value) in &values {
        if n % key == 0 {
            result.push_str(value);
        }
    }

    if result.is_empty() {
        return n.to_string();
    }

    result
}
