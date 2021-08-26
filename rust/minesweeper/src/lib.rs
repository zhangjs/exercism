pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }

    let mut v = vec![];
    for i in 0..rows {
        v.push(minefield[i].as_bytes().to_vec());
    }

    let cols = minefield[0].len();

    let mut count = |i: Option<usize>, j: Option<usize>| match (i, j) {
        (Some(row), Some(col)) if row < rows && col < cols => match v[row][col] {
            b' ' => v[row][col] = b'1',
            b'0'..=b'7' => v[row][col] += 1,
            _ => (),
        },
        _ => (),
    };

    for i in 0..rows {
        for (j, &c) in minefield[i].as_bytes().iter().enumerate() {
            if c == b'*' {
                count(i.checked_sub(1), j.checked_sub(1));
                count(i.checked_sub(1), Some(j));
                count(i.checked_sub(1), j.checked_add(1));
                count(Some(i), j.checked_sub(1));
                count(Some(i), j.checked_add(1));
                count(i.checked_add(1), j.checked_sub(1));
                count(i.checked_add(1), Some(j));
                count(i.checked_add(1), j.checked_add(1));
            }
        }
    }

    v.iter()
        .map(|v| v.iter().map(|&c| c as char).collect::<String>())
        .collect()
}
