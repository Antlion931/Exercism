use std::{str::FromStr, ops::{Add, Sub, Mul}};
use num_bigint::*;
use num_integer::Integer;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(PartialEq, Eq, Debug)]
pub struct Decimal {
    numerator: BigInt,
    denumerator: BigInt,
}

impl Decimal {
    pub fn try_from(mut input: &str) -> Option<Decimal> {
        if input.starts_with("+") {
            input = &input[1..];
        }

        let dot_positon = input.find('.').unwrap_or(input.len());
        let mut input = input.to_string();
        input.retain(|c| c != '.');
        let numerator = BigInt::from_str(input.as_str()).ok()?;
        let denumerator = (BigInt::from(10)).pow((input.len() - dot_positon) as u32);


        Some(Self::from_quantifiable(numerator, denumerator))
    }

    pub fn from_quantifiable(mut numerator: BigInt, mut denumerator: BigInt) -> Self {
        let gcd = numerator.gcd(&denumerator);
        numerator /= gcd.clone();
        denumerator /= gcd;

        Self { numerator, denumerator }
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.numerator *= rhs.denumerator.clone();
        rhs.numerator *= self.denumerator.clone();

        Self::from_quantifiable(self.numerator + rhs.numerator, self.denumerator * rhs.denumerator)
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self.numerator *= rhs.denumerator.clone();
        rhs.numerator *= self.denumerator.clone();

        Self::from_quantifiable(self.numerator - rhs.numerator, self.denumerator * rhs.denumerator)
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_quantifiable(self.numerator * rhs.numerator, self.denumerator * rhs.denumerator)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.numerator.clone() * other.denumerator.clone()).partial_cmp(&(other.numerator.clone() * self.denumerator.clone()))
    }
}
