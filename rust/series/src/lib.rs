pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_owned(); digits.len() + 1];
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|win| win.into_iter().map(|&b| b as char).collect::<String>())
        .collect()
}
