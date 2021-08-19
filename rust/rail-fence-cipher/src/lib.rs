use std::collections::BTreeMap;

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let n = (text.len() as f64 / self.rails as f64).ceil() as usize;
        let mut v: Vec<String> = (0..=self.rails).map(|_| String::with_capacity(n)).collect();

        for (c, i) in text.chars().zip(zigzag(self.rails)) {
            v[i].push(c);
        }

        v.into_iter().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut indexes: Vec<_> = zigzag(self.rails).zip(1..).take(cipher.len()).collect();
        indexes.sort();

        let char_with_index: BTreeMap<usize, char> = cipher
            .chars()
            .zip(indexes)
            .map(|(c, (_, i))| (i, c))
            .collect();

        char_with_index.values().collect()
    }
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}
