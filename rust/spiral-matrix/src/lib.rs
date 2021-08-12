static VECTORS_X: [isize; 4] = [0, 1, 0, -1];
static VECTORS_Y: [isize; 4] = [1, 0, -1, 0];

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut r = vec![vec![0u32; size]; size];

    if size == 0 {
        return r;
    }

    let mut x = 0;
    let mut y = -1;
    let mut v = 1u32;
    for i in 0..2 * size - 1 {
        for _ in 0..(2 * size - i) / 2 {
            x += VECTORS_X[i % 4];
            y += VECTORS_Y[i % 4];

            r[x as usize][y as usize] = v;
            v += 1;
        }
    }

    r
}
