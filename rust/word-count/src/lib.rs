use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for word in words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
    {
        let word: String = word.chars().skip_while(|&c| c == '\'').collect();
        let word: String = word.chars().rev().skip_while(|&c| c == '\'').collect();
        let word: String = word.chars().rev().map(|c| c.to_ascii_lowercase()).collect();

        if !word.is_empty() {
            *map.entry(word).or_default() += 1;
        }
    }

    map
}
