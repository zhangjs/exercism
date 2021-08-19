/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn mmi(a: i32) -> Option<i32> {
    let a = a % 26;
    if a % 2 == 0 || a % 13 == 0 {
        return None;
    }

    (1..).filter(|&x| a * x % 26 == 1).take(1).next()
}

fn alphabet_to_index(c: char) -> i32 {
    c.to_ascii_lowercase() as i32 - 'a' as i32
}

fn index_to_alphabet(index: i32) -> char {
    (index.rem_euclid(26) as u8 + b'a') as char
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a % 2 == 0 || a % 13 == 0 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                let index = alphabet_to_index(c);
                Some(index_to_alphabet(a * index + b))
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .collect::<Vec<_>>()
        .join(&' ')
        .iter()
        .collect())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if let Some(m) = mmi(a) {
        Ok(ciphertext
            .chars()
            .filter_map(|c| match c {
                c if c.is_alphabetic() => {
                    let index = alphabet_to_index(c);
                    Some(index_to_alphabet(m * (index - b)))
                }
                c if c.is_numeric() => Some(c),
                _ => None,
            })
            .collect::<String>())
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}
