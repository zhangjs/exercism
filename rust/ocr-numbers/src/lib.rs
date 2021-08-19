use std::collections::HashMap;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.split('\n').collect();

    let rows = lines.len();
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    }

    for i in 0..rows / 4 {
        if lines[i].len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(lines[i].len()));
        }

        for j in 1..4 {
            if lines[j].len() != lines[i].len() {
                return Err(Error::InvalidColumnCount(lines[j].len()));
            }
        }
    }

    let nums = vec![
        " _     _  _     _  _  _  _  _ ",
        "| |  | _| _||_||_ |_   ||_||_|",
        "|_|  ||_  _|  | _||_|  ||_| _|",
        "                              ",
    ];

    let map: HashMap<String, String> = (0..10)
        .into_iter()
        .map(|col| (digit(&nums, 0, col), col.to_string()))
        .collect();

    Ok((0..lines.len() / 4)
        .map(|row| {
            (0..lines[row].len() / 3)
                .map(|col| {
                    let key = digit(&lines, row, col);
                    map.get(&key).unwrap_or(&"?".to_string()).to_owned()
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(","))
}

fn digit(input: &Vec<&str>, row: usize, col: usize) -> String {
    (0..4)
        .map(|l| &input[row * 4 + l][3 * col..3 * col + 3])
        .collect::<Vec<&str>>()
        .join("")
}
