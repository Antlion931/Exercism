use itertools::Itertools;

pub trait ToStringWithPositons {
   fn to_string_with_positions(&self, new_postions: impl Iterator<Item = usize>) -> String; 
}

impl ToStringWithPositons for str {
    fn to_string_with_positions(&self, new_postions: impl Iterator<Item = usize>) -> String {
        self.chars()
            .zip( new_postions )
            .sorted_by(|a, b| a.1.cmp(&b.1) )
            .map(|(char, _)| char)
            .collect()
    }
}
