pub struct Pos(usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    let rows = lines.len();
    if rows < 2 {
        return 0;
    }
    let cols = lines[0].len();

    let mut counts = 0;
    for h in 1..rows {
        for w in 1..cols {
            for i in 0..rows - h {
                for j in 0..cols - w {
                    if is_rectangle(lines, Pos(i, j), Pos(i + h, j + w)) {
                        counts += 1;
                    }
                }
            }
        }
    }

    counts
}

pub fn is_rectangle(lines: &[&str], lu: Pos, dr: Pos) -> bool {
    lines[lu.0].as_bytes()[lu.1] == b'+'
        && lines[lu.0].as_bytes()[dr.1] == b'+'
        && lines[dr.0].as_bytes()[lu.1] == b'+'
        && lines[dr.0].as_bytes()[dr.1] == b'+'
        && lines[lu.0][lu.1..dr.1]
            .chars()
            .all(|c| c == '-' || c == '+')
        && lines[dr.0][lu.1..dr.1]
            .chars()
            .all(|c| c == '-' || c == '+')
        && lines[lu.0..dr.0]
            .iter()
            .all(|line| line.as_bytes()[lu.1] == b'+' || line.as_bytes()[lu.1] == b'|')
        && lines[lu.0..dr.0]
            .iter()
            .all(|line| line.as_bytes()[dr.1] == b'+' || line.as_bytes()[dr.1] == b'|')
}
