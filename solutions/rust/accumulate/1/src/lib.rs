/// What should the type of function be?
pub fn map<T, V, F>(input: Vec<T>, mut function: F) -> Vec<V>
where F: FnMut(T) -> V {
    let mut result = Vec::new();

    for elem in input {
        result.push(function(elem));
    }

    result
}