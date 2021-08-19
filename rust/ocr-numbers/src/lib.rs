// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut lines = input.lines();

    let rows = lines.by_ref().count();
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    }

    for line in lines {
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
    }

    Ok("".to_string())
}
