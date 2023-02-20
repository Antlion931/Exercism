use std::collections::HashMap;

pub struct CodonsInfo<'a>(HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.0.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chars| self.name_for(&chars.iter().collect::<String>()))
            .take_while(|option| option.filter(|name| *name == "stop codon").is_none())
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo( pairs.into_iter().collect() )
}
