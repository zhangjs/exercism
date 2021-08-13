pub fn encode(source: &str) -> String {
    let mut r = String::new();
    let mut n = 0;

    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        n += 1;
        if chars.peek() != Some(&c) {
            if n > 1 {
                r.push_str(&n.to_string());
            }

            r.push(c);
            n = 0;
        }
    }

    r
}

pub fn decode(source: &str) -> String {
    let mut r = String::new();
    let mut n = 0;

    for c in source.chars() {
        if let Some(d) = c.to_digit(10) {
            n = n * 10 + d;
        } else {
            if n > 0 {
                r.push_str(&c.to_string().repeat(n as usize));
            } else {
                r.push(c);
            }
            n = 0;
        }
    }

    r
}
