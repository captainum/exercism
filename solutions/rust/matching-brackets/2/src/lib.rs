use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    
    let map = HashMap::from(
        [
            (']', '['),
            ('}', '{'),
            (')', '('),
        ]
    );

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' | '}' | ')' => {
                if stack.pop() != Some(*map.get(&c).unwrap()) {
                    return false;
                }
            },
            _ => continue,
        }
    }
    
    stack.is_empty()
}
