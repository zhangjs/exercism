pub fn factors(n: u64) -> Vec<u64> {
    let mut f = vec![];
    let mut d = n;

    for i in 2..=n {
        while d % i == 0 {
            f.push(i);
            d /= i;
        }

        if d < 2 {
            break;
        }
    }

    f
}
