fn sort_uniq_ascending(mut vec: Vec<u32>) -> Vec<u32> {
    vec.sort_unstable();
    vec.dedup();
    vec
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors = factors.to_owned();
    let factors = sort_uniq_ascending(factors.to_vec());

    let mut multiples = vec![];

    for f in factors.iter().filter(|&&x| x != 0) {
        let mut x = *f;
        while x < limit {
            multiples.push(x);
            x += f;
        }
    }
    
    println!("{multiples:?}");
    sort_uniq_ascending(multiples).iter().sum()
}
