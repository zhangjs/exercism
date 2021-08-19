use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lowercase = word.to_lowercase();
    let mut v: Vec<char> = lowercase.chars().collect();

    v.sort_unstable();

    possible_anagrams
        .iter()
        .filter_map(|&anagram| {
            let al = anagram.to_lowercase();
            if al == lowercase {
                return None;
            }

            let mut av: Vec<char> = al.chars().collect();
            av.sort_unstable();
            if av == v {
                Some(anagram)
            } else {
                None
            }
        })
        .collect()
}
