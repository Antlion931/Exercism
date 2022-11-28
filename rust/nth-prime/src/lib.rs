pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3];
    while primes.len() <= n as usize {
        let mut x = primes.last().unwrap() + 2;
        while primes.iter().any(|p| x % p == 0) {
            x += 2;
        }
        primes.push(x);
    }
    primes[n as usize]
}
