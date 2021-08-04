use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let map = nucleotide_counts(dna)?;

    map.get(&nucleotide).cloned().ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<_, _> = ['A', 'C', 'G', 'T'].iter().map(|&c| (c, 0)).collect();

    for c in dna.chars() {
        map.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }

    Ok(map)
}
