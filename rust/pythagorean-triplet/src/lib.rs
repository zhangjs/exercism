use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (2..=sum / 3)
        .flat_map(|a| {
            (a + 1..=(sum - a) / 2).filter_map(move |b| {
                let c = sum - a - b;
                (a * a + b * b == c * c).then(|| [a, b, c])
            })
        })
        .collect()
}
