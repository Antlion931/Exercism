#![feature(test)]

use pascals_triangle::*;
extern crate test;
use test::{black_box, Bencher};

#[bench]
fn first(b: &mut Bencher) {
    b.iter(|| black_box(PascalsTriangle::new(black_box(100))))
}

/*
#[bench]
fn second(b: &mut Bencher) {
    b.iter(|| black_box(PascalsTriangle::new_2(black_box(100))))
}
*/
