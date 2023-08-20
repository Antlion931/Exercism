#![feature(test)]
extern crate test;

use std::collections::HashMap;
use test::{black_box, Bencher};

#[bench]
fn iter_values_in_hashmap(b: &mut Bencher) {
    let mut hm = HashMap::new();
    for i in 0..100_000 {
        hm.insert(i, i);
    }

    b.iter(|| black_box(black_box(&hm).values().sum::<u64>()));
}

#[bench]
fn iter_values_in_vector(b: &mut Bencher) {
    let mut hm = Vec::new();
    for i in 0..100_000 {
        hm.push(i);
    }

    b.iter(|| black_box(black_box(&hm).iter().sum::<u64>()));
}
