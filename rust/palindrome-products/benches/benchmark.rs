#![feature(test)]

use palindrome_products::*;

extern crate test;
use test::{black_box, Bencher};

/*#[bench]
fn checking_plalidron_with_string(b: &mut Bencher) {
    b.iter(|| black_box(Palindrome::with_string(black_box(123456789))))
}*/

#[bench]
fn checking_plalidron_with_math(b: &mut Bencher) {
    b.iter(|| black_box(Palindrome::new(black_box(123456789))))
}
