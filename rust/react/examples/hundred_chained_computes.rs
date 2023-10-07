#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
use react::*;

fn main() {
    let mut react = Reactor::<u128>::new();

    let mut all = Vec::with_capacity(1000);

    let input_id = react.create_input(0);

    all.push(CellId::Input(input_id));

    for _ in 1..100 {
        all.push(CellId::Compute(react.create_compute(&all, |x| x.iter().sum()).unwrap()));
    }

    react.set_value(input_id, 1);
    println!("{:?}", react.value(all[99]));
    react.set_value(input_id, 0);
    println!("{:?}", react.value(all[99]));
}
