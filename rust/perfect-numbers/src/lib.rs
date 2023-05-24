use std::cmp::Ordering;

use num_integer::Roots;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    (num > 0).then(
        || match factors(num).into_iter().sum::<u64>().cmp(&(num * 2)) {
            Ordering::Less => Classification::Deficient,
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Abundant,
        },
    )
}

fn factors(num: u64) -> Vec<u64> {
    let sqrt = num.nth_root(2);
    let mut result = Vec::with_capacity(sqrt as usize * 2 - 1);
    let mut sqrt_limit = sqrt;

    if num == sqrt * sqrt {
        result.push(sqrt);
    } else {
        sqrt_limit += 1;
    }

    for i in 1..sqrt_limit {
        if num % i == 0 {
            result.push(i);
            result.push(num / i);
        }
    }

    result
}
