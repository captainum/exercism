use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().flat_map(
        |(key, val)| {
            val.iter().map(
                |ch| {
                    (ch.to_ascii_lowercase(), *key)
                }
            )
        }
    ).collect()
}
