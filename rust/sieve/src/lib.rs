pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (0..=upper_bound).map(Option::from).collect();

    (2..=upper_bound)
        .filter_map(|i| {
            let prime = numbers[i as usize]?;
            (prime..=upper_bound)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|i| numbers[i as usize] = None);

            Some(prime)
        })
        .collect()
}
