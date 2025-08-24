const NAMES: [&str; 12] = [
    "Alice",
    "Bob",
    "Charlie",
    "David",
    "Eve",
    "Fred",
    "Ginny",
    "Harriet",
    "Ileana",
    "Joseph",
    "Kincaid",
    "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let idx = NAMES.iter().position(|&name| name == student).unwrap() * 2;

    diagram.lines().flat_map(|line| {
        line[idx ..idx + 2].chars().map(
            |symbol| match symbol {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            }
        )
    }).collect()
}
