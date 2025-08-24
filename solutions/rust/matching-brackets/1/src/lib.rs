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
        if c == '[' || c == '{' || c == '(' {
            stack.push(c);
        }
        else if (c == ']' || c == '}' || c == ')') && (stack.pop() != Some(*map.get(&c).unwrap())) {
            return false;
        }
    }
    
    stack.is_empty()
}
