#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    for c in string_digits.chars() {
        if !c.is_digit(10) {
            return Err(Error::InvalidDigit(c));
        }
    }

    string_digits
        .as_bytes()
        .windows(span)
        .map(|win| win.iter().map(|&b| (b - b'0') as u64).product::<u64>())
        .max()
        .ok_or(Error::SpanTooLong)
}
