pub fn number(user_number: &str) -> Option<String> {
    let mut v: Vec<u8> = user_number
        .as_bytes()
        .into_iter()
        .cloned()
        .filter(|&b| b >= b'0' && b <= b'9')
        .collect();

    if v.len() == 11 && v[0] == b'1' && v[1] >= b'2' && v[4] >= b'2' {
        v.remove(0);
    } else if v.len() == 10 && v[0] >= b'2' && v[3] >= b'2' {
    } else {
        return None;
    }

    Some(v.iter().map(|&b| b as char).collect())
}
