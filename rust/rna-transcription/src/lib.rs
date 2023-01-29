mod contains_only;

use contains_only::ContainsOnly;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: Vec<char>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        Ok(Self { sequence: dna.chars().contains_only(&['A', 'C', 'G', 'T'])? } )
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(&self.sequence.iter().map(|c| {
            match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => unreachable!("dna only constains A, C, G and T characters"),
            }
        }).collect::<String>()).expect("Should only contains U, G, C and A characters")
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        Ok(Self { sequence: rna.chars().contains_only(&['A', 'C', 'G', 'U'])? } )
    }
}
