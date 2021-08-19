pub fn encrypt(input: &str) -> String {
    let v: Vec<char> = input
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();

    let c = (v.len() as f64).sqrt().ceil() as usize;

    (0..c)
        .map(|i| {
            v.chunks(c)
                .map(|chunk| if i < chunk.len() { chunk[i] } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
