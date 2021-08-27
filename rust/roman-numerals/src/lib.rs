use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        assert!(num <= 3000);
        let base = [("I", "V"), ("X", "L"), ("C", "D"), ("M", "M")];
        let s: String = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .rev()
            .enumerate()
            .map(|(i, c)| match c {
                1..=3 => base[i].0.repeat(c),
                4 => base[i].1.to_owned() + base[i].0,
                5 => base[i].1.to_owned(),
                6..=8 => base[i].0.repeat(c - 5) + base[i].1,
                9 => base[i + 1].0.to_owned() + base[i].0,
                _ => "".to_owned(),
            })
            .collect();

        Self(s.chars().rev().collect())
    }
}
