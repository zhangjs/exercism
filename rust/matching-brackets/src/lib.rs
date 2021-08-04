pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v = vec![];

    for c in string.chars() {
        match c {
            '[' | '(' | '{' => v.push(c),
            ']' => {
                if v.pop() != Some('[') {
                    return false;
                }
            }
            ')' => {
                if v.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if v.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }

    v.is_empty()
}
