pub fn nth(n: u32) -> u32 {
    let nth = n + 1;
    for i in (nth..).step_by(nth as usize) {
        let primes = get_primes(i);
        if primes.len() >= nth as usize {
            return primes[(nth - 1) as usize];
        }
    }

    n
}

// get primes up to n
fn get_primes(n: u32) -> Vec<u32> {
    let mut numbers: Vec<_> = (0..=n).map(Option::from).collect();

    (2..=n)
        .filter_map(|i| {
            let prime = numbers[i as usize]?;
            (prime..=n)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|i| numbers[i as usize] = None);

            Some(prime)
        })
        .collect()
}
