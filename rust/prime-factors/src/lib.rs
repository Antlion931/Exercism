pub fn factors( mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    
    for i in 2.. {
        while n % i == 0 {
            n /= i;
            result.push(i);
        }

        if n == 1 {
            break;
        }
    }
    result
}
