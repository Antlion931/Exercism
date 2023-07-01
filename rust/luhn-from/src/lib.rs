pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let chars = self.0.chars().filter(|c| !c.is_whitespace());

        if chars.clone().count() < 2 || chars.clone().any(|c| !c.is_ascii_digit()) {
            return false;
        }

        chars
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let x = (i as u32 % 2 + 1) * c.to_digit(10).unwrap();

                if x > 9 {
                    x - 9
                } else {
                    x
                }
            })
            .sum::<u32>()
            % 10
            == 0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
