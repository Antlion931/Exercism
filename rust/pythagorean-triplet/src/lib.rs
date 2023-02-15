use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    
    for a in 1..=(sum.div_euclid(3) - 1) {
        for b in a+1..=(sum - a - 1).div_euclid(2) {
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                result.insert([a, b, c]);
            }
        }
    }

    result
}
