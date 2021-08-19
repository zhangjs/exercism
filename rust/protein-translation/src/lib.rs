use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        self.pairs.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(|codon| self.name_for(codon.unwrap()))
            .take_while(|&codon| codon != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.into_iter().collect(),
    }
}
