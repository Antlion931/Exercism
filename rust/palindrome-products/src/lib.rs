use itertools::{Itertools, MinMaxResult};
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /*pub fn with_string(value: u64) -> Option<Palindrome> {
        let original = value.to_string();
        let reversed: String = original.chars().rev().collect();

        (original == reversed).then_some(Palindrome(value))
    }*/

    pub fn new(value: u64) -> Option<Palindrome> {
        let mut reversed = 0;
        let mut copy = value;

        while copy > 0 {
            let digit = copy % 10;
            reversed = reversed * 10 + digit;
            copy /= 10;
        }

        (reversed == value).then_some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    //let all_palidroms: Vec<_> = (min..=max).combinations(2).filter_map(|vec| Palindrome::new(vec[0] * vec[1])).collect();
    let minmax = (min..=max)
        .combinations_with_replacement(2)
        .filter_map(|vec| Palindrome::new(vec[0] * vec[1]))
        .minmax();

    match minmax {
        MinMaxResult::MinMax(a, b) => Some((a, b)),
        MinMaxResult::OneElement(a) => Some((a, a)),
        MinMaxResult::NoElements => None,
    }
}
