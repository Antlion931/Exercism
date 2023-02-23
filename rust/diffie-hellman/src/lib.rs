use rand::Rng;

fn modular_power(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    
    let mut base = base as u128;
    let mut exponent = exponent as u128;
    let modulus = modulus as u128;

    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result as u64
}

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_power(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_power(b_pub, a, p)
}
