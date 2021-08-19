#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .map(|&v| {
            let mut r = vec![];
            let mut div = v;
            loop {
                r.push((div & 0x7f) as u8);

                div = div >> 7;
                if div == 0 {
                    break;
                }
            }
            r.iter_mut().skip(1).for_each(|x| *x |= 0x80);
            r.reverse();

            r
        })
        .flatten()
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut v = vec![];
    let mut n: u32 = 0;
    let mut complete = false;

    for byte in bytes {
        if let Some(a) = n.checked_mul(128) {
            if let Some(b) = a.checked_add((byte & 0x7f) as u32) {
                n = b;
                complete = false;
            } else {
                return Err(Error::Overflow);
            }
        } else {
            return Err(Error::Overflow);
        }

        if byte & 0x80 == 0 {
            v.push(n);
            n = 0;
            complete = true;
        }
    }

    if !complete {
        Err(Error::IncompleteNumber)
    } else {
        Ok(v)
    }
}
