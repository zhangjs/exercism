/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(swap)
        .collect::<Vec<char>>()
        .chunks(5)
        .collect::<Vec<_>>()
        .join(&' ')
        .iter()
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(swap).collect::<String>()
}

fn swap(c: char) -> Option<char> {
    match c {
        'a'..='z' => Some((b'a' + b'z' - c as u8) as char),
        'A'..='Z' => Some((b'a' + b'Z' - c as u8) as char),
        '0'..='9' => Some(c),
        _ => None,
    }
}
