/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(sum, count), c| {
            c.to_digit(10)
                .map(|num| {
                    if count % 2 == 1 {
                        if 2 * num > 9 {
                            2 * num - 9
                        } else {
                            2 * num
                        }
                    } else {
                        num
                    }
                })
                .map(|num| (sum + num, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
