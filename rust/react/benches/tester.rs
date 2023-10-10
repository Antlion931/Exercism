#![feature(test)]
extern crate test;

use react::*;
use test::{black_box, Bencher};

#[bench]
fn tester_creating_new(b: &mut Bencher) {
    b.iter(|| black_box(Reactor::<u32>::new()))
}

#[bench]
fn tester_adding_1000_input_cells(b: &mut Bencher) {
    b.iter(|| {
        let mut react = Reactor::new();
        for i in 0..1000 {
            react.create_input(i);
        }

        black_box(react)
    })
}

#[bench]
fn tester_adding_1000_compute_cells(b: &mut Bencher) {
    b.iter(|| {
        let mut react = Reactor::new();
        for _ in 0..1000 {
            react.create_compute(&[], |_| 1).unwrap();
        }

        black_box(react)
    })
}

#[bench]
fn tester_getting_value_of_1000_input_cells(b: &mut Bencher) {
    let mut react = Reactor::new();

    let id_vec: Vec<_> = (0..1000)
        .map(|i| CellId::Input(react.create_input(i)))
        .collect();

    b.iter(|| {
        for i in id_vec.iter() {
            black_box(react.value(*i));
        }
    })
}

#[bench]
fn tester_getting_value_of_1000_compute_cells(b: &mut Bencher) {
    let mut react = Reactor::new();

    let id_vec: Vec<_> = (0..1000)
        .map(|_| CellId::Compute(react.create_compute(&[], |_| 1).unwrap()))
        .collect();

    b.iter(|| {
        for i in id_vec.iter() {
            black_box(react.value(*i));
        }
    })
}

#[bench]
fn tester_100_chained_update_first(b: &mut Bencher) {
    let mut react = Reactor::<u128>::new();

    let mut all = Vec::with_capacity(1000);

    let input_id = react.create_input(0);

    all.push(CellId::Input(input_id));

    for _ in 1..100 {
        all.push(CellId::Compute(
            react.create_compute(&all, |x| x.iter().sum()).unwrap(),
        ));
    }

    b.iter(|| {
        react.set_value(input_id, 1);
        black_box(react.value(all[99]));
        react.set_value(input_id, 0);
        black_box(react.value(all[99]));
    })
}
