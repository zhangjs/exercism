use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || s.is_empty() {
        return None;
    }

    if !s.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    if !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    Some(
        key.chars()
            .cycle()
            .zip(s.chars())
            .map(|(k, c)| {
                ((c as i32 + k as i32 - 2 * 'a' as i32).rem_euclid(26) as u8 + b'a') as char
            })
            .collect(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || s.is_empty() {
        return None;
    }

    if !s.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    if !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    Some(
        key.chars()
            .cycle()
            .zip(s.chars())
            .map(|(k, c)| (b'a' + (c as i32 - k as i32).rem_euclid(26) as u8) as char)
            .collect(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = (0..100)
        .map(|_| (thread_rng().gen_range(0, 26) + b'a') as char)
        .collect();

    let e = encode(&key, s).unwrap();
    (key, e)
}
