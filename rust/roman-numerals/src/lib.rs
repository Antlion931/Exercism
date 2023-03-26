use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    digits: Vec<String>,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.digits.concat())
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let roman_digits = ["I", "V", "X", "L", "C", "D", "M"];
        let mut reverse_digits = Vec::new();
        let mut index = 0;
        
        while num > 0 {
            reverse_digits.push(match num % 10 {
                0 => String::new(),
                x @ 1..=3 => format!("{}", vec![roman_digits[index]; x as usize].concat()),
                4 => format!("{}{}", roman_digits[index], roman_digits[index + 1]),
                x @ 5..=8 => format!("{}{}", roman_digits[index + 1], vec![roman_digits[index]; x as usize - 5].concat()),
                9 => format!("{}{}", roman_digits[index], roman_digits[index + 2]),
                _ => unreachable!("All reminers of modulo 10 have been covered"),
            });

            index += 2;
            num /= 10;
        }

        Self { digits: reverse_digits.into_iter().rev().collect() }
    }
}
