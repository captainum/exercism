use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let idx = array.len() / 2;

    match key.cmp(array.get(idx)?) {
        Ordering::Equal => Some(idx),
        Ordering::Less => find(&array[..idx], key),
        Ordering::Greater => find(&array[idx + 1 ..], key).map(|idx2| idx + 1 + idx2),
    }
}