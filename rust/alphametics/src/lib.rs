use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters: Vec<_> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .unique()
        .sorted()
        .collect();
    let leading: Vec<_> = input
        .split_whitespace()
        .map(|w| w.chars().next().unwrap())
        .filter(|c| c.is_alphabetic())
        .unique()
        .collect();

    let equal_position = input.find('=').unwrap();

    let left = &input[..equal_position];
    let right = &input[equal_position + 3..];

    let mut left_counts = [0; 10];

    left.split_whitespace().filter(|w| *w != "+").for_each(|w| {
        let mut position = 1;
        for c in w.chars().rev() {
            left_counts[letters.binary_search(&c).unwrap()] += position;
            position *= 10;
        }
    });

    let mut right_counts = [0; 10];

    {
        let mut position = 1;
        for c in right.chars().rev() {
            right_counts[letters.binary_search(&c).unwrap()] += position;
            position *= 10;
        }
    }

    println!("{letters:?}\n{left_counts:?}\n{right_counts:?}");

    for p in (0u8..=9u8).permutations(letters.len()) {
        if leading
            .iter()
            .any(|c| p[letters.binary_search(c).unwrap()] == 0)
        {
            continue;
        }

        let left: u64 = p
            .iter()
            .enumerate()
            .map(|(i, x)| *x as u64 * left_counts[i])
            .sum();
        let right: u64 = p
            .iter()
            .enumerate()
            .map(|(i, x)| *x as u64 * right_counts[i])
            .sum();

        if left == right {
            return Some(letters.into_iter().zip(p.into_iter()).collect());
        }
    }

    None
}
