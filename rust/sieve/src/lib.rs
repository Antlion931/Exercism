#[derive(Clone, PartialEq, Eq)]
pub enum NumberState {
    PotencialPrime,
    NotPrime,
}

pub fn change_state(array: &mut [NumberState], number: usize) {
    array
        .iter_mut()
        .skip(number * 2 - 1)
        .step_by(number)
        .for_each(|x| *x = NumberState::NotPrime);
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }

    let sqrt = (upper_bound as f64).sqrt() as usize;

    let mut array = vec![NumberState::PotencialPrime; upper_bound as usize];

    array[0] = NumberState::NotPrime;
    array[1] = NumberState::PotencialPrime;

    change_state(&mut array, 2);

    for i in (3..=sqrt).step_by(2) {
        // on position 2 there is 3rd number
        if array[i - 1] == NumberState::PotencialPrime {
            change_state(&mut array, i);
        }
    }

    array
        .iter()
        .enumerate()
        .filter(|(_, s)| **s == NumberState::PotencialPrime)
        .map(|(n, _)| n as u64 + 1)
        .collect()
}
