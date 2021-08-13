/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars().fold(0, |sum, c| match c.to_ascii_uppercase() {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => sum + 1,
        'D' | 'G' => sum + 2,
        'B' | 'C' | 'M' | 'P' => sum + 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => sum + 4,
        'K' => sum + 5,
        'J' | 'X' => sum + 8,
        'Q' | 'Z' => sum + 10,
        _ => sum,
    })
}
