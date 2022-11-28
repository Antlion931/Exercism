use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {

    let mut heap = Vec::new();
    let brackets = HashMap::from([ 
        ( '}', '{' ), 
        ( ']', '[' ), 
        ( ')', '(' ) ]);

    for c in string.chars() {
        if brackets.values().any(|&v| v == c) {
            heap.push(c);
        } else if brackets.contains_key(&c) {
            if let Some(p) = heap.last() {
                if brackets[&c] == *p{
                    heap.pop();
                    continue;
                }
            }
            return false;
        }
    }
    heap.is_empty()
}