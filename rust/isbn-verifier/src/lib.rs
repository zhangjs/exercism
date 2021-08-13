/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut count = 0;
    let mut sum = 0;

    for c in isbn.chars() {
        match c {
            '0'..='9' if count < 10 => {
                sum += (10 - count) * c.to_digit(10).unwrap();
                count += 1;
            }
            'X' if count == 9 => {
                sum += 10;
                count += 1;
            }
            '-' => (),
            _ => return false,
        }
    }

    count == 10 && sum % 11 == 0
}
