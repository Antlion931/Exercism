#![feature(test)]

extern crate test;
use spiral_matrix::*;
use test::*;

#[bench]
fn first_small(b: &mut test::Bencher) {
    b.iter(|| black_box(spiral_matrix(black_box(5))))
}

/*#[bench]
fn second_small(b: &mut test::Bencher) {
    b.iter(|| black_box(spiral_matrix_2(black_box(5))))
}*/

#[bench]
fn first_big(b: &mut test::Bencher) {
    b.iter(|| black_box(spiral_matrix(black_box(1000))))
}

/*#[bench]
fn second_big(b: &mut test::Bencher) {
    b.iter(|| black_box(spiral_matrix_2(black_box(1000))))
}*/
