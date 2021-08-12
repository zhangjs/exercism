pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut r = vec![];

    for (i, row) in input.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if row.iter().all(|x| x <= v) && input.iter().all(|r| r[j] >= *v) {
                r.push((i, j))
            }
        }
    }

    r
}
