#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.char_indices() {
            match c {
                'A' | 'C' | 'G' | 'T' => (),
                _ => return Err(i),
            }
        }

        Ok(Dna {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .dna
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                c => c,
            })
            .collect();

        Rna { rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.char_indices() {
            match c {
                'A' | 'C' | 'G' | 'U' => (),
                _ => return Err(i),
            }
        }

        Ok(Rna {
            rna: rna.to_string(),
        })
    }
}
