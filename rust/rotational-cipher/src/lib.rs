pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => (b'a' + (c as i32 - 'a' as i32 + key as i32).rem_euclid(26) as u8) as char,
            'A'..='Z' => (b'A' + (c as i32 - 'A' as i32 + key as i32).rem_euclid(26) as u8) as char,
            _ => c,
        })
        .collect()
}
