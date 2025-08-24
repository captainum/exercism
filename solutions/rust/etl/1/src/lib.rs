use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (key, val) in h {
        for ch in val {
            result.insert(ch.to_ascii_lowercase(), *key);
        }
    }
    
    result
}
