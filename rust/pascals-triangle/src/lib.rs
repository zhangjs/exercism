pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => vec![],
            // 1 => vec![vec![1]],
            // r => {
            //     let mut rows = Self::new(r - 1).rows();

            //     let last = rows.last().unwrap();
            //     let mut row = vec![1];

            //     for i in 1..r - 1 {
            //         row.push(last[(i - 1) as usize] + last[i as usize]);
            //     }
            //     row.push(1);

            //     rows.push(row);
            //     rows
            // }
            r => {
                let mut rows = vec![vec![1]];
                for i in 1..r {
                    let last = rows.last().unwrap();
                    let mut row = vec![1];

                    for j in 1..i {
                        row.push(last[(j - 1) as usize] + last[j as usize]);
                    }
                    row.push(1);

                    rows.push(row);
                }

                rows
            }
        }
    }
}
