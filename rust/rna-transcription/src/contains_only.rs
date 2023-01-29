pub trait ContainsOnly: Iterator {
    fn contains_only(self, contained: &[Self::Item]) -> Result<Vec<Self::Item>, usize> 
        where
            Self::Item: Eq, 
            Self: Sized,
    {
        let copy: Vec<_> = self.collect();
        if let Some(positon_of_first_bad) = copy.iter().position(|t| !contained.contains(&t)) {
            return Err(positon_of_first_bad);
        }
        Ok(copy)
    }
}

impl<I: Iterator> ContainsOnly for I {}

