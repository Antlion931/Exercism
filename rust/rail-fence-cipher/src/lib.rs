mod my_str_method;

use itertools::Itertools;
use my_str_method::ToStringWithPositons;

pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails as usize)
    }

    fn zigzag(&self) -> impl Iterator<Item = usize> {
        (0..(self.0 - 1))
            .chain( (1..self.0).rev() )
            .cycle()
    }

    fn positons(&self, lenght: usize) -> impl Iterator<Item = usize> {
        (0..lenght).zip( self.zigzag() )
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .map(|(positon, _)| positon)
    }

    pub fn encode(&self, text: &str) -> String {
        text.to_string_with_positions(self.zigzag())
    }

    pub fn decode(&self, cipher: &str) -> String {
        cipher.to_string_with_positions( self.positons(cipher.chars().count()) )
    }
}
