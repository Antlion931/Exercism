pub fn collatz(mut n: u64) -> Option<u64> {
    let mut result = 0;

    if n == 0 {
        return None;
    }

    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;       
        }

        result += 1;
    }
    Some(result)
} 
