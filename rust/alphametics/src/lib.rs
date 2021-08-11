use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .collect_vec();

    let mut letters: Vec<char> = input
        .chars()
        .filter_map(|c| if c.is_alphabetic() { Some(c) } else { None })
        .collect();
    letters.sort();
    letters.dedup();

    let len = letters.len();
    if len > 10 {
        return None;
    }

    let words_len = words.len();
    let perms = (0..10u8).permutations(len);
    for perm in perms {
        let map: HashMap<char, u8> = letters.iter().cloned().zip(perm.into_iter()).collect();

        if words
            .iter()
            .any(|word| word.len() > 1 && map.get(&word.chars().nth(0).unwrap()) == Some(&0))
        {
            continue;
        }

        let numbers: Vec<u64> = words
            .iter()
            .map(|word| {
                word.chars()
                    .map(|c| map.get(&c).cloned().unwrap().to_string())
                    .collect::<String>()
                    .parse()
                    .unwrap()
            })
            .collect();

        if numbers[words_len - 1] == numbers[0..words_len - 1].iter().sum() {
            return Some(map);
        }
    }

    None
}
