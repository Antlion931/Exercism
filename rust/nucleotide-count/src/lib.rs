use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .remove(&nucleotide)
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::from([
                                    ('A', 0),
                                    ('C', 0),
                                    ('G', 0),
                                    ('T', 0),
    ]);

    for c in dna.chars() {
        *result.get_mut(&c).ok_or(c)? += 1;
    }

    Ok(result)
}
