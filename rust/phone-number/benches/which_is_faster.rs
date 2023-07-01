#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn collect_string_to_string(b: &mut Bencher) {
    let numbers: Vec<_> = (0..10).collect();

    b.iter(|| black_box(numbers.iter().map(|x| x.to_string()).collect::<String>()))
}

#[bench]
fn sum_digits_to_number_then_to_string(b: &mut Bencher) {
    let numbers: Vec<_> = (0..10).collect();

    b.iter(|| black_box(numbers.iter().fold(0, |acc, x| acc * 10 + x).to_string()))
}
