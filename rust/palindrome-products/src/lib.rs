use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> Option<u64> {
        if !self.factors.is_empty() {
            Some(self.factors[0].0 * self.factors[0].1)
        } else {
            None
        }
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut r = n;
    let mut reverse = 0;

    while r != 0 {
        reverse = reverse * 10 + r % 10;
        r = r / 10;
    }

    reverse == n
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut map: BTreeMap<u64, Palindrome> = BTreeMap::new();

    for i in min..=max {
        for j in i..=max {
            if is_palindrome(i * j) {
                map.entry(i * j).or_default().insert(i, j);
            }
        }
    }

    if let (Some(pmin), Some(pmax)) = (map.iter().next(), map.iter().next_back()) {
        Some((pmin.1.clone(), pmax.1.clone()))
    } else {
        None
    }
}
