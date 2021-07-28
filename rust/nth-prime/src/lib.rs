pub fn nth(n: u32) -> u32 {
    let mut total = 0;
    let mut size_factor = 2u32;
    let nth = n + 1;
    let mut s: u64 = nth as u64 * size_factor as u64;
    let mut primes: Vec<u32> = vec![];

    while total < nth {
        primes = get_primes(s);

        total = primes[2..].iter().sum::<u32>();

        size_factor += 1;
        s = nth as u64 * size_factor as u64;
    }

    let nth_prime = count_primes(nth, primes);

    return nth_prime;
}

// get primes less than n
fn get_primes(n: u64) -> Vec<u32> {
    let mut primes = vec![1; n as usize];

    for i in 2..n {
        if primes[i as usize] == 1 {
            for j in i..n {
                if i * j < n {
                    primes[(i * j) as usize] = 0;
                } else {
                    break;
                }
            }
        }
    }

    primes
}

fn count_primes(nth: u32, primes: Vec<u32>) -> u32 {
    let mut count = 0;
    for i in 2..primes.len() {
        count += primes[i];
        if count == nth {
            return i as u32;
        }
    }

    0
}

fn is_prime(n: u32) -> bool {
    !(2..n).any(|i| n % i == 0)
}

pub fn nth2(n: u32) -> u32 {
    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}
