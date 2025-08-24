pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let idx = array.len() / 2;

    if key != array[idx] && idx == 0 {
        None
    }
    else if key < array[idx] {
        find(&array[..idx], key)
    }
    else if key > array[idx] {
        match find(&array[idx + 1..], key) {
            Some(idx2) => Some(idx + 1 + idx2),
            None => None,
        }
    }
    else {
        Some(idx)
    }
}