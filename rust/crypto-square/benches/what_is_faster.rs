#![feature(test)]
extern crate test;

use crypto_square::{encrypt, faster_encrypt};
use test::black_box;
use test::Bencher;

#[bench]
fn precious(b: &mut Bencher) {
    b.iter(|| {
        encrypt(black_box(
            r#"
We choose to go to the moon.

We choose to go to the moon in this decade and do the other things,
not because they are easy, but because they are hard, because that
goal will serve to organize and measure the best of our energies and
skills, because that challenge is one that we are willing to accept,
one we are unwilling to postpone, and one which we intend to win,
and the others, too.

-- John F. Kennedy, 12 September 1962
        "#,
        ))
    });
}
